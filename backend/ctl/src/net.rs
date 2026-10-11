//! The zone matrix `deploy/zones.yaml`, and what is rendered from it.
//!
//! The file says which network every service is on and which service may reach which on which port. Two
//! things are read out of it here: the host firewall rules that keep the CBS reachable from `akr-ingestd`
//! only, and the table of reachability that `make zone-check` probes.

use std::collections::{BTreeMap, BTreeSet};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::Path;

use serde::Deserialize;
use serde_json::Value;
use yaml_rust2::{Yaml, YamlLoader};

use crate::error::CtlError;

pub const INGESTD: &str = "akr-ingestd";
pub const CBS_NETWORK: &str = "z1_cbs";
const VERSION: u32 = 1;
const NFT_TABLE: &str = "akr_zones";
const MAX_PREFIX: u8 = 32;

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Zones {
    pub version: u32,
    pub networks: BTreeMap<String, Network>,
    pub services: BTreeMap<String, Service>,
    pub edges: Vec<Edge>,
    #[serde(default)]
    pub cbs: Cbs,
    #[serde(default)]
    pub cluster: Cluster,
    #[serde(default)]
    pub gateway: Gateway,
    #[serde(default)]
    pub backup: Backup,
    #[serde(default)]
    pub egress: Egress,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Network {
    /// `true`: no route out of the network.
    pub internal: bool,
    /// Fixed when a rendered rule names the network.
    #[serde(default)]
    pub subnet: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Service {
    pub networks: Vec<String>,
    /// The ports the service listens on for other services.
    #[serde(default)]
    pub ports: Vec<u16>,
    /// A fixed address on a network, for a service a rendered rule names.
    #[serde(default)]
    pub addresses: BTreeMap<String, Ipv4Addr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub ports: Vec<u16>,
}

/// The CBS listeners the bank declares at install.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cbs {
    #[serde(default)]
    pub listeners: Vec<SocketAddrV4>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cluster {
    #[serde(default)]
    pub apiserver: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Gateway {
    #[serde(default)]
    pub cidrs: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Backup {
    #[serde(default)]
    pub repository: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Egress {
    #[serde(default)]
    pub proxy: Option<String>,
}

/// What a probe from one service's position to another service's port must find.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reach {
    /// An edge allows it.
    Open,
    /// No network joins the two.
    Closed,
    /// A network joins the two and no edge names the port. Compose separates by network, so the port
    /// answers; the cluster profile allows each edge alone and closes it.
    Zone,
}

impl Reach {
    pub fn as_str(self) -> &'static str {
        match self {
            Reach::Open => "open",
            Reach::Closed => "closed",
            Reach::Zone => "zone",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Probe {
    pub from: String,
    pub to: String,
    pub port: u16,
    pub expect: Reach,
}

fn to_json(yaml: &Yaml) -> Result<Value, CtlError> {
    Ok(match yaml {
        Yaml::Null => Value::Null,
        Yaml::Boolean(flag) => Value::Bool(*flag),
        Yaml::Integer(number) => Value::from(*number),
        Yaml::String(text) => Value::String(text.clone()),
        Yaml::Array(items) => Value::Array(items.iter().map(to_json).collect::<Result<_, _>>()?),
        Yaml::Hash(entries) => {
            let mut object = serde_json::Map::new();
            for (key, value) in entries {
                let Yaml::String(key) = key else {
                    return Err(CtlError::usage("a key of the zone file is not a string"));
                };
                if object.insert(key.clone(), to_json(value)?).is_some() {
                    return Err(CtlError::usage(format!(
                        "the zone file names `{key}` twice"
                    )));
                }
            }
            Value::Object(object)
        }
        Yaml::Real(text) => {
            return Err(CtlError::usage(format!(
                "the zone file holds the number `{text}`; quote it if it is text"
            )));
        }
        Yaml::Alias(_) | Yaml::BadValue => {
            return Err(CtlError::usage(
                "the zone file holds a value that cannot be read",
            ));
        }
    })
}

fn parse_subnet(text: &str) -> Option<(Ipv4Addr, u8)> {
    let (address, prefix) = text.split_once('/')?;
    let address: Ipv4Addr = address.parse().ok()?;
    let prefix: u8 = prefix.parse().ok()?;
    (prefix <= MAX_PREFIX && u32::from(address) & !mask(prefix) == 0).then_some((address, prefix))
}

fn mask(prefix: u8) -> u32 {
    u32::MAX
        .checked_shl(u32::from(MAX_PREFIX - prefix))
        .unwrap_or(0)
}

fn in_subnet(address: Ipv4Addr, (network, prefix): (Ipv4Addr, u8)) -> bool {
    u32::from(address) & mask(prefix) == u32::from(network)
}

impl Zones {
    pub fn parse(text: &str) -> Result<Self, CtlError> {
        let documents = YamlLoader::load_from_str(text)
            .map_err(|e| CtlError::usage(format!("the zone file is not YAML: {e}")))?;
        let [document] = documents.as_slice() else {
            return Err(CtlError::usage("the zone file holds one YAML document"));
        };
        let zones: Zones = serde_json::from_value(to_json(document)?)
            .map_err(|e| CtlError::usage(format!("the zone file: {e}")))?;
        zones.validate()?;
        Ok(zones)
    }

    pub fn load(path: &Path) -> Result<Self, CtlError> {
        let text = std::fs::read_to_string(path).map_err(CtlError::file(path))?;
        Self::parse(&text)
    }

    fn validate(&self) -> Result<(), CtlError> {
        let wrong = |what: String| Err(CtlError::usage(format!("the zone file: {what}")));
        if self.version != VERSION {
            return wrong(format!("version {} is not {VERSION}", self.version));
        }
        for (name, network) in &self.networks {
            if let Some(subnet) = &network.subnet
                && parse_subnet(subnet).is_none()
            {
                return wrong(format!(
                    "network `{name}`: `{subnet}` is not an IPv4 subnet"
                ));
            }
        }
        for (name, service) in &self.services {
            let mut seen = BTreeSet::new();
            for network in &service.networks {
                if !self.networks.contains_key(network) {
                    return wrong(format!(
                        "service `{name}` is on the unknown network `{network}`"
                    ));
                }
                if !seen.insert(network) {
                    return wrong(format!("service `{name}` names `{network}` twice"));
                }
            }
            for (network, address) in &service.addresses {
                let subnet = seen
                    .contains(network)
                    .then(|| self.networks[network].subnet.as_deref())
                    .flatten()
                    .and_then(parse_subnet);
                if !subnet.is_some_and(|subnet| in_subnet(*address, subnet)) {
                    return wrong(format!(
                        "service `{name}`: {address} is not an address of its network `{network}`"
                    ));
                }
            }
        }
        for edge in &self.edges {
            let (Some(from), Some(to)) =
                (self.services.get(&edge.from), self.services.get(&edge.to))
            else {
                return wrong(format!(
                    "the edge `{}` -> `{}` names an unknown service",
                    edge.from, edge.to
                ));
            };
            if edge.from == edge.to || edge.ports.is_empty() {
                return wrong(format!(
                    "the edge `{}` -> `{}` needs two services and a port",
                    edge.from, edge.to
                ));
            }
            if let Some(port) = edge.ports.iter().find(|port| !to.ports.contains(port)) {
                return wrong(format!(
                    "the edge `{}` -> `{}` names the port {port}, which `{}` does not listen on",
                    edge.from, edge.to, edge.to
                ));
            }
            if !from.networks.iter().any(|n| to.networks.contains(n)) {
                return wrong(format!(
                    "the edge `{}` -> `{}` has no network that joins the two",
                    edge.from, edge.to
                ));
            }
        }
        Ok(())
    }

    fn allowed(&self, from: &str, to: &str, port: u16) -> bool {
        self.edges
            .iter()
            .any(|edge| edge.from == from && edge.to == to && edge.ports.contains(&port))
    }

    /// Every service's position against every port of every other service.
    pub fn matrix(&self) -> Vec<Probe> {
        let mut probes = Vec::new();
        for (from, source) in &self.services {
            for (to, target) in &self.services {
                if from == to {
                    continue;
                }
                let joined = source.networks.iter().any(|n| target.networks.contains(n));
                for &port in &target.ports {
                    let expect = if self.allowed(from, to, port) {
                        Reach::Open
                    } else if joined {
                        Reach::Zone
                    } else {
                        Reach::Closed
                    };
                    probes.push(Probe {
                        from: from.clone(),
                        to: to.clone(),
                        port,
                        expect,
                    });
                }
            }
        }
        probes
    }

    /// One line per probe: `from`, `to`, `port`, `networks of from`, `expect`, separated by tabs.
    pub fn matrix_lines(&self) -> Vec<String> {
        self.matrix()
            .into_iter()
            .map(|probe| {
                format!(
                    "{}\t{}\t{}\t{}\t{}",
                    probe.from,
                    probe.to,
                    probe.port,
                    self.services[&probe.from].networks.join(","),
                    probe.expect.as_str()
                )
            })
            .collect()
    }

    /// The host rules: of everything the host forwards, only `akr-ingestd` opens a connection to a CBS
    /// listener, and nothing on the CBS network opens one to anything else.
    pub fn render_nft(&self) -> Result<String, CtlError> {
        let missing = |what: &str| {
            CtlError::usage(format!(
                "the zone file gives no {what}; the host rules need it"
            ))
        };
        let subnet = self
            .networks
            .get(CBS_NETWORK)
            .and_then(|network| network.subnet.as_deref())
            .ok_or_else(|| missing("subnet for `z1_cbs`"))?;
        let ingestd = self
            .services
            .get(INGESTD)
            .and_then(|service| service.addresses.get(CBS_NETWORK))
            .ok_or_else(|| missing("address for `akr-ingestd` on `z1_cbs`"))?;
        if let Some((other, _)) = self.services.iter().find(|(name, service)| {
            *name != INGESTD && service.networks.iter().any(|n| n == CBS_NETWORK)
        }) {
            return Err(CtlError::usage(format!(
                "the zone file puts `{other}` on `z1_cbs`; `akr-ingestd` is the only service there"
            )));
        }

        let listeners: BTreeSet<SocketAddrV4> = self.cbs.listeners.iter().copied().collect();
        let mut out = String::new();
        out.push_str("#!/usr/sbin/nft -f\n");
        out.push_str(
            "# Rendered by `akr-ctl net render` from deploy/zones.yaml. Change that file and render again.\n\n",
        );
        out.push_str(&format!(
            "table inet {NFT_TABLE}\ndelete table inet {NFT_TABLE}\n\n"
        ));
        out.push_str(&format!("table inet {NFT_TABLE} {{\n"));
        out.push_str("\tset cbs_listeners {\n\t\ttype ipv4_addr . inet_service\n");
        if !listeners.is_empty() {
            let elements: Vec<String> = listeners
                .iter()
                .map(|listener| format!("{} . {}", listener.ip(), listener.port()))
                .collect();
            out.push_str(&format!("\t\telements = {{ {} }}\n", elements.join(", ")));
        }
        out.push_str("\t}\n\n\tchain forward {\n");
        out.push_str("\t\ttype filter hook forward priority filter - 10; policy accept;\n");
        out.push_str("\t\tct state established,related accept\n");
        out.push_str(&format!(
            "\t\tip saddr {ingestd} ip daddr . tcp dport @cbs_listeners accept\n"
        ));
        out.push_str(&format!(
            "\t\tip saddr {subnet} ip daddr != {subnet} drop\n"
        ));
        out.push_str("\t\tip daddr . tcp dport @cbs_listeners drop\n");
        out.push_str("\t}\n}\n");
        Ok(out)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    const ZONES: &str = r#"
version: 1
networks:
  z1_cbs: { internal: false, subnet: 172.31.247.0/28 }
  z1_db: { internal: true }
  z2_app: { internal: true }
  z2_store: { internal: true }
services:
  postgres: { networks: [z1_db, z2_app], ports: [5432] }
  valkey: { networks: [z2_app], ports: [6379] }
  objectstore: { networks: [z2_store], ports: [9000] }
  akr-api: { networks: [z2_app] }
  akr-ingestd:
    networks: [z1_cbs, z1_db]
    addresses: { z1_cbs: 172.31.247.2 }
edges:
  - { from: akr-api, to: postgres, ports: [5432] }
  - { from: akr-api, to: valkey, ports: [6379] }
  - { from: akr-ingestd, to: postgres, ports: [5432] }
cbs:
  listeners: ["10.20.0.15:1521", "10.20.0.16:1433"]
"#;

    fn expect(zones: &Zones, from: &str, to: &str, port: u16) -> Reach {
        zones
            .matrix()
            .into_iter()
            .find(|probe| probe.from == from && probe.to == to && probe.port == port)
            .unwrap()
            .expect
    }

    #[test]
    fn the_matrix_follows_the_edges_and_the_networks() {
        let zones = Zones::parse(ZONES).unwrap();
        assert_eq!(expect(&zones, "akr-ingestd", "postgres", 5432), Reach::Open);
        assert_eq!(expect(&zones, "akr-ingestd", "valkey", 6379), Reach::Closed);
        assert_eq!(
            expect(&zones, "akr-api", "objectstore", 9000),
            Reach::Closed
        );
        assert_eq!(expect(&zones, "valkey", "postgres", 5432), Reach::Zone);
        assert_eq!(zones.matrix().len(), 4 * 3);
    }

    #[test]
    fn the_host_rules_let_only_ingestd_reach_the_listeners() {
        let rules = Zones::parse(ZONES).unwrap().render_nft().unwrap();
        assert!(rules.contains("elements = { 10.20.0.15 . 1521, 10.20.0.16 . 1433 }"));
        assert!(rules.contains("ip saddr 172.31.247.2 ip daddr . tcp dport @cbs_listeners accept"));
        assert!(rules.contains("ip saddr 172.31.247.0/28 ip daddr != 172.31.247.0/28 drop"));
        let accept = rules.find("@cbs_listeners accept").unwrap();
        let drop = rules
            .find("ip daddr . tcp dport @cbs_listeners drop")
            .unwrap();
        assert!(accept < drop);
    }

    #[test]
    fn no_listener_declared_means_nothing_is_reachable() {
        let zones = Zones::parse(&ZONES.replace(
            "listeners: [\"10.20.0.15:1521\", \"10.20.0.16:1433\"]",
            "listeners: []",
        ))
        .unwrap();
        let rules = zones.render_nft().unwrap();
        assert!(!rules.contains("elements"));
        assert!(rules.contains("ip saddr 172.31.247.0/28 ip daddr != 172.31.247.0/28 drop"));
    }

    #[test]
    fn a_file_that_contradicts_itself_is_refused() {
        let refused = |from: &str, to: &str| {
            let text = ZONES.replace(from, to);
            assert_ne!(text, ZONES, "`{from}` is not in the fixture");
            assert!(Zones::parse(&text).is_err(), "{to}");
        };
        refused("version: 1", "version: 2");
        refused(
            "networks: [z2_app], ports: [6379]",
            "networks: [z9], ports: [6379]",
        );
        refused("to: valkey, ports: [6379]", "to: valkey, ports: [6380]");
        refused(
            "from: akr-ingestd, to: postgres",
            "from: akr-ingestd, to: valkey",
        );
        refused("z1_cbs: 172.31.247.2", "z1_cbs: 172.31.248.2");
        refused("subnet: 172.31.247.0/28", "subnet: 172.31.247.1/28");
        refused("10.20.0.15:1521", "cbs.bank.example:1521");
        refused("cbs:\n", "surprise: 1\ncbs:\n");
    }

    #[test]
    fn a_second_service_on_the_cbs_network_is_refused_at_render() {
        let zones = Zones::parse(&ZONES.replace(
            "akr-api: { networks: [z2_app] }",
            "akr-api: { networks: [z2_app, z1_cbs] }",
        ))
        .unwrap();
        assert!(zones.render_nft().is_err());
    }

    #[test]
    fn a_subnet_is_a_network_address_and_a_prefix() {
        assert_eq!(
            parse_subnet("172.31.247.0/28"),
            Some((Ipv4Addr::new(172, 31, 247, 0), 28))
        );
        assert_eq!(parse_subnet("0.0.0.0/0"), Some((Ipv4Addr::UNSPECIFIED, 0)));
        assert_eq!(parse_subnet("172.31.247.0/33"), None);
        assert_eq!(parse_subnet("172.31.247.0"), None);
        let subnet = parse_subnet("172.31.247.0/28").unwrap();
        assert!(in_subnet(Ipv4Addr::new(172, 31, 247, 15), subnet));
        assert!(!in_subnet(Ipv4Addr::new(172, 31, 247, 16), subnet));
    }
}
