//! Address blocks, for the one rule that depends on where a request comes from.

use std::net::IpAddr;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cidr {
    network: IpAddr,
    prefix: u8,
}

impl Cidr {
    pub fn contains(&self, address: IpAddr) -> bool {
        match (self.network, canonical(address)) {
            (IpAddr::V4(network), IpAddr::V4(address)) => {
                masked(u128::from(u32::from(network)), self.prefix, 32)
                    == masked(u128::from(u32::from(address)), self.prefix, 32)
            }
            (IpAddr::V6(network), IpAddr::V6(address)) => {
                masked(u128::from(network), self.prefix, 128)
                    == masked(u128::from(address), self.prefix, 128)
            }
            _ => false,
        }
    }
}

/// An IPv4 peer that reached an IPv6 listener arrives as `::ffff:a.b.c.d`.
fn canonical(address: IpAddr) -> IpAddr {
    match address {
        IpAddr::V6(v6) => v6.to_ipv4_mapped().map_or(address, IpAddr::V4),
        IpAddr::V4(_) => address,
    }
}

fn masked(bits: u128, prefix: u8, width: u8) -> u128 {
    if prefix == 0 {
        0
    } else {
        bits >> (width - prefix)
    }
}

impl FromStr for Cidr {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let (address, prefix) = match text.split_once('/') {
            Some((address, prefix)) => (address, Some(prefix)),
            None => (text, None),
        };
        let network: IpAddr = address
            .trim()
            .parse()
            .map_err(|_| format!("`{text}` is not an address block"))?;
        let width = if network.is_ipv4() { 32 } else { 128 };
        let prefix = match prefix {
            None => width,
            Some(prefix) => prefix
                .trim()
                .parse::<u8>()
                .ok()
                .filter(|prefix| *prefix <= width)
                .ok_or_else(|| format!("`{text}` has no usable prefix length"))?,
        };
        Ok(Self { network, prefix })
    }
}

/// A comma-separated list. An empty list is valid: nobody is the installer.
pub fn parse_list(text: &str) -> Result<Vec<Cidr>, String> {
    text.split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(str::parse)
        .collect()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn ip(text: &str) -> IpAddr {
        text.parse().unwrap()
    }

    #[test]
    fn a_block_holds_its_addresses_and_no_other() {
        let block: Cidr = "10.200.14.0/24".parse().unwrap();
        assert!(block.contains(ip("10.200.14.1")));
        assert!(block.contains(ip("10.200.14.255")));
        assert!(!block.contains(ip("10.200.15.1")));
        assert!(!block.contains(ip("fd00::1")));
        assert!(block.contains(ip("::ffff:10.200.14.7")));

        let host: Cidr = "127.0.0.1".parse().unwrap();
        assert!(host.contains(ip("127.0.0.1")));
        assert!(!host.contains(ip("127.0.0.2")));

        let v6: Cidr = "fd00::/8".parse().unwrap();
        assert!(v6.contains(ip("fd12:3456::1")));
        assert!(!v6.contains(ip("fe80::1")));

        let everyone: Cidr = "0.0.0.0/0".parse().unwrap();
        assert!(everyone.contains(ip("203.0.113.9")));
    }

    #[test]
    fn a_list_is_read_whole_or_not_at_all() {
        assert_eq!(parse_list("").unwrap(), []);
        assert_eq!(parse_list(" 10.0.0.0/8 , 127.0.0.1 ").unwrap().len(), 2);
        for bad in [
            "10.0.0.0/33",
            "ten",
            "10.0.0.0/",
            "fd00::/129",
            "10.0.0.0/8,nope",
        ] {
            assert!(parse_list(bad).is_err(), "{bad}");
        }
    }
}
