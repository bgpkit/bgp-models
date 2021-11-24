//! Common network-related structs.

use std::fmt::{Display, Formatter};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
use ipnetwork::IpNetwork;
use crate::err::BgpModelsError;

/// Meta information for an address/prefix.
///
/// [AddrMeta] is a struct that used to save address family and as number length information
/// when parsing [TableDumpMessage].
///
/// The meta information includes:
/// 1. `afi`: address family ([Afi]): IPv4 or IPv6,
/// 2. `asn_len`: AS number length ([AsnLength]): 16 or 32 bits.
#[derive(Debug, Clone)]
pub struct AddrMeta {
    pub afi: Afi,
    pub asn_len: AsnLength,
}

/// AS number length: 16 or 32 bits.
#[derive(Debug, Clone)]
pub enum AsnLength {
    Bits16,
    Bits32,
}

/// ASN -- Autonomous System Number
pub type Asn = u32;

/// AFI -- Address Family Identifier
///
/// https://www.iana.org/assignments/address-family-numbers/address-family-numbers.xhtml
#[derive(Debug, PartialEq, Primitive, Clone, Copy)]
pub enum Afi {
    Ipv4 = 1,
    Ipv6 = 2,
}

/// SAFI -- Subsequent Address Family Identifier
///
/// SAFI can be: Unicast, Multicast, or both.
#[derive(Debug, PartialEq, Primitive, Clone, Copy)]
pub enum Safi {
    Unicast = 1,
    Multicast = 2,
    UnicastMulticast = 3,
}

/// enum that represents the type of the next hop address.
///
/// [NextHopAddress] is used when parsing for next hops in [Nlri].
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum NextHopAddress {
    Ipv4(Ipv4Addr),
    Ipv6(Ipv6Addr),
    Ipv6LinkLocal(Ipv6Addr, Ipv6Addr),
}

/// A representation of a IP prefix with optional path ID.
#[derive(Debug, PartialEq, Clone)]
pub struct NetworkPrefix {
    pub prefix: IpNetwork,
    pub path_id: u32,
}

impl FromStr for NetworkPrefix {
    type Err = BgpModelsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let prefix = IpNetwork::from_str(s)?;
        Ok(
            NetworkPrefix{
                prefix,
                path_id: 0,
            }
        )
    }
}

impl NetworkPrefix {
    pub fn new(prefix: IpNetwork, path_id: u32) -> NetworkPrefix {
        NetworkPrefix { prefix, path_id }
    }
}

impl Display for NetworkPrefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.prefix)
    }
}

