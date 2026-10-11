//! The commands, as the operator types them.

use std::path::PathBuf;

use akr_common::config::Env;
use chrono::Utc;
use clap::{Parser, Subcommand};
use zeroize::Zeroizing;

use crate::bundle::build::{BuildOptions, build};
use crate::bundle::import::{Registry, import};
use crate::bundle::signing::DevSigner;
use crate::bundle::verify::{self, Verified};
use crate::conn::ConnArgs;
use crate::db;
use crate::error::CtlError;
use crate::net::Zones;
use crate::operator::Trail;
use crate::ops::{self, DrillResult};
use crate::store::{self, ConformanceArgs, StoreArgs};

/// The role `ops` commands sign in as: the one the API uses, so they change what an administrator could.
const APP_ROLE: &str = "akr_app";

#[derive(Debug, Parser)]
#[command(
    name = "akr-ctl",
    version,
    about = "The operator's command line of an AKR-TMS installation"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// The product's database: roles, migrations, checks
    #[command(subcommand)]
    Db(DbCommand),
    /// The object store: buckets and the conformance suite
    #[command(subcommand)]
    Store(StoreCommand),
    /// The release bundle: build, verify, load into the registry
    #[command(subcommand)]
    Bundle(BundleCommand),
    /// Records of operations the bank carries out
    #[command(subcommand)]
    Ops(OpsCommand),
    /// What is rendered from the zone file
    #[command(subcommand)]
    Net(NetCommand),
}

#[derive(Debug, Subcommand)]
pub enum DbCommand {
    /// Creates the database, the migration role and the login roles, or brings them to that state
    Provision(ConnArgs),
    /// Applies the migrations of this build as akr_migrate
    Migrate {
        #[command(flatten)]
        conn: ConnArgs,
        /// The approved release this migration belongs to. Required in production.
        #[arg(long, env = "AKR_RELEASE_ID")]
        release_id: Option<String>,
    },
    /// Checks the locale, the collations, the roles and their grants. Changes nothing.
    Check(ConnArgs),
}

#[derive(Debug, Subcommand)]
pub enum StoreCommand {
    /// Creates the locked buckets, or confirms them
    Init(StoreArgs),
    /// Runs the conformance suite against the store
    Conformance {
        #[command(flatten)]
        store: StoreArgs,
        #[command(flatten)]
        args: ConformanceArgs,
    },
}

#[derive(Debug, clap::Args)]
pub struct KeyArgs {
    /// The vendor's public key, a PEM file
    #[arg(long)]
    pub pubkey: PathBuf,
    /// The key's fingerprint, as received on the separate channel
    #[arg(long)]
    pub fingerprint: String,
}

#[derive(Debug, Subcommand)]
pub enum BundleCommand {
    /// Writes a bundle from a staged tree
    Build {
        /// The staged tree
        #[arg(long)]
        source: PathBuf,
        /// Where the bundle is written: absent or empty
        #[arg(long)]
        out: PathBuf,
        #[arg(long)]
        release: String,
        /// An OCI image layout to ship as images/
        #[arg(long)]
        images_layout: Option<PathBuf>,
        /// A PEM Ed25519 private key that signs a development bundle. A release is signed by the pipeline.
        #[arg(long)]
        signing_key: Option<PathBuf>,
    },
    /// Verifies a bundle against the vendor's key. Changes nothing.
    Verify {
        bundle: PathBuf,
        #[command(flatten)]
        key: KeyArgs,
        /// The bundle of the installed release, to list what changed in the bill of materials
        #[arg(long)]
        installed: Option<PathBuf>,
    },
    /// Verifies a bundle, then loads its images into the registry
    Import {
        bundle: PathBuf,
        #[command(flatten)]
        key: KeyArgs,
        /// The registry: https://host[:port]
        #[arg(long)]
        registry: String,
        /// The certificate authority of the registry, a PEM file
        #[arg(long)]
        cacert: Option<PathBuf>,
        #[arg(long, requires = "password_file")]
        username: Option<String>,
        #[arg(long, requires = "username")]
        password_file: Option<PathBuf>,
    },
}

#[derive(Debug, Subcommand)]
pub enum OpsCommand {
    /// Records a restore drill with its evidence
    RecordRestoreDrill {
        #[command(flatten)]
        conn: ConnArgs,
        #[arg(long, value_enum)]
        result: DrillResult,
        /// The drill's report
        #[arg(long)]
        evidence: PathBuf,
    },
}

#[derive(Debug, clap::Args)]
pub struct ZoneArgs {
    /// deploy/zones.yaml
    #[arg(long, env = "AKR_ZONES")]
    pub zones: PathBuf,
}

#[derive(Debug, Subcommand)]
pub enum NetCommand {
    /// Prints the host nftables rules that keep the CBS reachable from akr-ingestd only
    Render(ZoneArgs),
    /// Prints what a probe from every service's position must find
    Matrix(ZoneArgs),
}

impl Command {
    pub fn name(&self) -> &'static str {
        match self {
            Command::Db(DbCommand::Provision(_)) => "db provision",
            Command::Db(DbCommand::Migrate { .. }) => "db migrate",
            Command::Db(DbCommand::Check(_)) => "db check",
            Command::Store(StoreCommand::Init(_)) => "store init",
            Command::Store(StoreCommand::Conformance { .. }) => "store conformance",
            Command::Bundle(BundleCommand::Build { .. }) => "bundle build",
            Command::Bundle(BundleCommand::Verify { .. }) => "bundle verify",
            Command::Bundle(BundleCommand::Import { .. }) => "bundle import",
            Command::Ops(OpsCommand::RecordRestoreDrill { .. }) => "ops record-restore-drill",
            Command::Net(NetCommand::Render(_)) => "net render",
            Command::Net(NetCommand::Matrix(_)) => "net matrix",
        }
    }

    /// `true` when the command changes the installation or what it holds. Such a command says who ran it.
    /// The conformance suite is one of them: it writes and locks probe objects.
    pub fn changes(&self) -> bool {
        matches!(
            self,
            Command::Db(DbCommand::Provision(_) | DbCommand::Migrate { .. })
                | Command::Store(_)
                | Command::Bundle(BundleCommand::Import { .. })
                | Command::Ops(_)
        )
    }

    /// `false` for a command that reads files and prints: it runs on a host that has no installation yet.
    pub fn acts_on_installation(&self) -> bool {
        self.changes() || matches!(self, Command::Db(DbCommand::Check(_)))
    }
}

/// What a command printed, and how it ended. A command that fails may have printed what it found first.
pub type Outcome = (Vec<String>, Result<(), CtlError>);

fn settle(result: Result<Vec<String>, CtlError>) -> Outcome {
    match result {
        Ok(lines) => (lines, Ok(())),
        Err(e) => (Vec::new(), Err(e)),
    }
}

fn verified(bundle: &std::path::Path, key: &KeyArgs) -> Result<Verified, CtlError> {
    let pem = std::fs::read_to_string(&key.pubkey).map_err(CtlError::file(&key.pubkey))?;
    verify::verify(bundle, &pem, &key.fingerprint)
}

fn read_secret(path: &std::path::Path) -> Result<Zeroizing<String>, CtlError> {
    let text = Zeroizing::new(std::fs::read_to_string(path).map_err(CtlError::file(path))?);
    let line = text.trim();
    if line.is_empty() {
        return Err(CtlError::usage(format!("{} is empty", path.display())));
    }
    Ok(Zeroizing::new(line.to_owned()))
}

/// The commands that read files and print.
pub fn run_offline(command: Command) -> Outcome {
    settle(match command {
        Command::Bundle(BundleCommand::Build {
            source,
            out,
            release,
            images_layout,
            signing_key,
        }) => (|| {
            let signer = signing_key
                .map(|path| {
                    let pem = Zeroizing::new(
                        std::fs::read_to_string(&path).map_err(CtlError::file(&path))?,
                    );
                    DevSigner::from_pem(&pem)
                })
                .transpose()?;
            let options = BuildOptions {
                source,
                out,
                release,
                images_layout,
            };
            build(&options, signer.as_ref(), Utc::now())
        })(),
        Command::Bundle(BundleCommand::Verify {
            bundle,
            key,
            installed,
        }) => (|| {
            let checked = verified(&bundle, &key)?;
            let mut lines = verify::report(&bundle, &checked);
            if let Some(installed) = installed {
                lines.extend(verify::sbom_diff(&bundle, &installed)?.lines());
            }
            Ok(lines)
        })(),
        Command::Net(NetCommand::Render(args)) => Zones::load(&args.zones)
            .and_then(|zones| zones.render_nft())
            .map(|rules| vec![rules.trim_end().to_owned()]),
        Command::Net(NetCommand::Matrix(args)) => {
            Zones::load(&args.zones).map(|zones| zones.matrix_lines())
        }
        other => Err(CtlError::usage(format!(
            "`{}` acts on an installation",
            other.name()
        ))),
    })
}

/// The commands that reach the database, the store or the registry.
pub async fn run(command: Command, env: Env, trail: &Trail) -> Outcome {
    match command {
        Command::Db(DbCommand::Provision(conn)) => settle(db::provision(&conn).await),
        Command::Db(DbCommand::Migrate { conn, release_id }) => {
            settle(db::migrate(&conn, env, release_id.as_deref()).await)
        }
        Command::Db(DbCommand::Check(conn)) => match db::check(&conn).await {
            Ok(findings) => (
                findings.iter().map(db::Finding::line).collect(),
                db::verdict(&findings),
            ),
            Err(e) => (Vec::new(), Err(e)),
        },
        Command::Store(StoreCommand::Init(args)) => match args.client() {
            Ok(client) => settle(store::init(&client).await),
            Err(e) => (Vec::new(), Err(e)),
        },
        Command::Store(StoreCommand::Conformance { store, args }) => match store.client() {
            Ok(client) => store::conformance(&client, &args).await,
            Err(e) => (Vec::new(), Err(e)),
        },
        Command::Bundle(BundleCommand::Import {
            bundle,
            key,
            registry,
            cacert,
            username,
            password_file,
        }) => settle(
            async {
                let checked = verified(&bundle, &key)?;
                let basic = match (username, password_file) {
                    (Some(user), Some(path)) => Some((user, read_secret(&path)?)),
                    _ => None,
                };
                let registry = Registry::new(&registry, cacert.as_deref(), basic)?;
                let mut lines = verify::report(&bundle, &checked);
                lines.extend(import(&bundle, &checked, &registry).await?);
                Ok(lines)
            }
            .await,
        ),
        Command::Ops(OpsCommand::RecordRestoreDrill {
            conn,
            result,
            evidence,
        }) => settle(
            async {
                let pool = conn.pool(APP_ROLE).await?;
                let lines =
                    ops::record_restore_drill(&pool, trail, result, &evidence, Utc::now()).await;
                pool.close().await;
                lines
            }
            .await,
        ),
        offline => run_offline(offline),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    fn parse(line: &str) -> Command {
        Cli::try_parse_from(line.split_whitespace())
            .unwrap()
            .command
    }

    #[test]
    fn the_command_tree_is_well_formed() {
        Cli::command().debug_assert();
    }

    #[test]
    fn every_command_that_changes_something_is_named_for_the_trail() {
        let changing = [
            "akr-ctl db provision --secrets-dir /s",
            "akr-ctl db migrate --secrets-dir /s --release-id r1",
            "akr-ctl store init --endpoint http://127.0.0.1:9000 --access-key-file /a --secret-key-file /b",
            "akr-ctl store conformance --endpoint http://127.0.0.1:9000 --access-key-file /a --secret-key-file /b",
            "akr-ctl bundle import /b --pubkey /k --fingerprint ab --registry https://harbor.bank.example",
            "akr-ctl ops record-restore-drill --secrets-dir /s --result passed --evidence /e",
        ];
        for line in changing {
            let command = parse(line);
            assert!(command.changes(), "{line}");
            assert!(command.acts_on_installation(), "{line}");
            assert!(line.contains(command.name()), "{line}");
        }
    }

    #[test]
    fn a_command_that_reads_files_needs_no_installation() {
        for line in [
            "akr-ctl bundle verify /b --pubkey /k --fingerprint ab",
            "akr-ctl bundle build --source /s --out /o --release 1.0.0",
            "akr-ctl net render --zones /z",
            "akr-ctl net matrix --zones /z",
        ] {
            let command = parse(line);
            assert!(!command.changes(), "{line}");
            assert!(!command.acts_on_installation(), "{line}");
        }
        let check = parse("akr-ctl db check --secrets-dir /s");
        assert!(!check.changes());
        assert!(check.acts_on_installation());
    }

    #[test]
    fn a_registry_user_comes_with_its_password_file() {
        let line = "akr-ctl bundle import /b --pubkey /k --fingerprint ab --registry https://h --username robot";
        assert!(Cli::try_parse_from(line.split_whitespace()).is_err());
    }
}
