use std::net::{Ipv4Addr, Ipv6Addr};
use ipnetwork::IpNetwork;

#[derive(Debug, Clone)]
pub struct AddrMeta {
    pub afi: Afi,
    pub asn_len: AsnLength,
}

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
#[derive(Debug, PartialEq, Primitive, Clone, Copy)]
pub enum Safi {
    Unicast = 1,
    Multicast = 2,
    UnicastMulticast = 3,
}

#[derive(Debug, PartialEq)]
pub enum NextHopAddress {
    Ipv4(Ipv4Addr),
    Ipv6(Ipv6Addr),
    Ipv6LinkLocal(Ipv6Addr, Ipv6Addr),
}

#[derive(Debug, PartialEq, Clone)]
pub struct NetworkPrefix {
    pub prefix: IpNetwork,
    pub path_id: u32,
}

impl NetworkPrefix {
    pub fn new(prefix: IpNetwork, path_id: u32) -> NetworkPrefix {
        NetworkPrefix { prefix, path_id }
    }
}

