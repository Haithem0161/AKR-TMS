//! How a command reaches PostgreSQL: where the server is, and the directory that holds one password file
//! per role.
//!
//! The directory is the operator's: filled from OpenBao at the installation ceremony, read here, and removed
//! when the command is done. The file of role `r` is `db_r`; the administrator's is `pg_superuser`.

use std::path::PathBuf;

use akr_common::config::{DbSecret, SslMode};
use sqlx::{Connection, PgConnection, PgPool};
use zeroize::Zeroizing;

use crate::error::CtlError;

const APPLICATION: &str = "akr-ctl";
const ADMIN_PASSWORD_FILE: &str = "pg_superuser";

#[derive(Debug, Clone, clap::Args)]
pub struct ConnArgs {
    #[arg(long, env = "AKR_DB_HOST", default_value = "127.0.0.1")]
    pub host: String,

    #[arg(long, env = "AKR_DB_PORT", default_value_t = 5432)]
    pub port: u16,

    /// The product's database.
    #[arg(long, env = "AKR_DB_NAME", default_value = "akr")]
    pub database: String,

    /// The certificate authority that signed the server's certificate. Without it, the system's trust
    /// store is used. The server's certificate and name are verified either way.
    #[arg(long, env = "AKR_DB_SSLROOTCERT")]
    pub sslrootcert: Option<PathBuf>,

    /// The role that creates databases and roles. Used by `db provision` and `db check` only.
    #[arg(long, env = "AKR_DB_ADMIN_USER", default_value = "postgres")]
    pub admin_user: String,

    /// The database the administrator connects to before the product's database exists.
    #[arg(long, env = "AKR_DB_MAINTENANCE_NAME", default_value = "postgres")]
    pub maintenance_database: String,

    /// The directory with one password file per role (`db_<role>`, `pg_superuser`).
    #[arg(long, env = "AKR_SECRETS_DIR")]
    pub secrets_dir: PathBuf,
}

impl ConnArgs {
    pub fn password_file(&self, role: &str) -> PathBuf {
        if role == self.admin_user {
            self.secrets_dir.join(ADMIN_PASSWORD_FILE)
        } else {
            self.secrets_dir.join(format!("db_{role}"))
        }
    }

    /// `None` when the role has no password file.
    pub fn password(&self, role: &str) -> Result<Option<Zeroizing<String>>, CtlError> {
        let path = self.password_file(role);
        let text = match std::fs::read_to_string(&path) {
            Ok(text) => Zeroizing::new(text),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(CtlError::file(path)(e)),
        };
        let password = text.trim_end_matches(['\r', '\n']);
        if password.is_empty() {
            return Err(CtlError::usage(format!("{} is empty", path.display())));
        }
        Ok(Some(Zeroizing::new(password.to_owned())))
    }

    pub fn required_password(&self, role: &str) -> Result<Zeroizing<String>, CtlError> {
        self.password(role)?.ok_or_else(|| {
            CtlError::usage(format!(
                "no password file for `{role}`: {} is missing",
                self.password_file(role).display()
            ))
        })
    }

    pub fn secret(&self, role: &str, database: &str) -> Result<DbSecret, CtlError> {
        Ok(DbSecret {
            host: self.host.clone(),
            port: self.port,
            database: database.to_owned(),
            username: role.to_owned(),
            password: self.required_password(role)?,
            sslmode: SslMode::VerifyFull,
            sslrootcert: self.sslrootcert.clone(),
        })
    }

    pub async fn connect(&self, role: &str, database: &str) -> Result<PgConnection, CtlError> {
        let secret = self.secret(role, database)?;
        let options = akr_db::pool::connect_options(&secret, APPLICATION);
        Ok(PgConnection::connect_with(&options).await?)
    }

    pub async fn admin(&self, database: &str) -> Result<PgConnection, CtlError> {
        self.connect(&self.admin_user, database).await
    }

    pub async fn pool(&self, role: &str) -> Result<PgPool, CtlError> {
        let secret = self.secret(role, &self.database)?;
        Ok(akr_db::connect(&secret, APPLICATION, 2).await?)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn args(dir: &std::path::Path) -> ConnArgs {
        ConnArgs {
            host: "db.bank.example".to_owned(),
            port: 5432,
            database: "akr".to_owned(),
            sslrootcert: None,
            admin_user: "postgres".to_owned(),
            maintenance_database: "postgres".to_owned(),
            secrets_dir: dir.to_path_buf(),
        }
    }

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("akr-ctl-conn-{}-{name}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn a_role_reads_its_own_file_and_the_administrator_reads_the_superuser_file() {
        let dir = scratch("files");
        std::fs::write(dir.join("db_akr_app"), "app-secret\n").unwrap();
        std::fs::write(dir.join("pg_superuser"), "root-secret\r\n").unwrap();
        let args = args(&dir);

        assert_eq!(
            args.password("akr_app").unwrap().unwrap().as_str(),
            "app-secret"
        );
        assert_eq!(
            args.password("postgres").unwrap().unwrap().as_str(),
            "root-secret"
        );
        assert!(args.password("akr_ingest").unwrap().is_none());

        let secret = args.secret("akr_app", "akr").unwrap();
        assert_eq!(secret.username, "akr_app");
        assert_eq!(secret.sslmode, SslMode::VerifyFull);
    }

    #[test]
    fn an_empty_file_is_an_error_and_a_missing_one_names_its_path() {
        let dir = scratch("empty");
        std::fs::write(dir.join("db_akr_app"), "\n").unwrap();
        let args = args(&dir);

        assert!(args.password("akr_app").is_err());
        let missing = args
            .required_password("akr_assess")
            .unwrap_err()
            .to_string();
        assert!(missing.contains("db_akr_assess"), "{missing}");
    }
}
