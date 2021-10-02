use std::net::IpAddr;
use std::collections::HashMap;
use crate::bgp::attributes::AttributeMap;
use crate::network::{Afi, Asn, NetworkPrefix, Safi};

#[derive(Debug)]
pub struct TableDumpMessage {
    pub view_number: u16,
    pub sequence_number: u16,
    pub prefix: NetworkPrefix,
    pub status: u8,
    pub originated_time: u64,
    pub peer_address: IpAddr,
    pub peer_asn: Asn,
    pub attributes: AttributeMap,
}

#[derive(Debug)]
pub enum TableDumpV2Message {
    PeerIndexTable(PeerIndexTable),
    RibAfiEntries(RibAfiEntries),
    RibGenericEntries(RibGenericEntries),
}

#[derive(Debug)]
pub struct RibAfiEntries{
    pub sequence_number: u32,
    pub prefix: NetworkPrefix,
    pub rib_entries: Vec<RibEntry>,
}

#[derive(Debug)]
pub struct RibGenericEntries{
    pub sequence_number: u32,
    pub afi: Afi,
    pub safi: Safi,
    pub nlri: NetworkPrefix,
    pub rib_entries: Vec<RibEntry>,
}

#[derive(Debug)]
pub struct RibEntry {
    pub peer_index: u16,
    pub originated_time: u32,
    pub attributes: AttributeMap
}

/// https://www.iana.org/assignments/mrt/mrt.xhtml#subtype-codes
#[derive(Debug, Primitive)]
pub enum TableDumpV2Type{
    PeerIndexTable = 1,
    RibIpv4Unicast = 2,
    RibIpv4Multicast = 3,
    RibIpv6Unicast = 4,
    RibIpv6Multicast = 5,
    RibGeneric = 6,
    GeoPeerTable = 7,
    RibIpv4UnicastAddPath = 8,
    RibIpv4MulticastAddPath = 9,
    RibIpv6UnicastAddPath = 10,
    RibIpv6MulticastAddPath = 11,
    RibGenericAddPath = 12,
}

#[derive(Debug)]
pub struct PeerIndexTable{
    pub collector_bgp_id: u32,
    pub view_name_length: u16,
    pub view_name: String,
    pub peer_count: u16,
    pub peers_map: HashMap<u32, Peer>
}

#[derive(Debug)]
pub struct Peer {
    pub peer_type: u8,
    pub peer_bgp_id: u32,
    pub peer_address: IpAddr,
    pub peer_asn: Asn,
}

