use std::net::IpAddr;
use crate::bgp::BgpMessage;
use crate::network::{Afi, Asn};

#[derive(Debug)]
pub enum Bgp4Mp {
    Bgp4MpStateChange(Bgp4MpStateChange),
    Bgp4MpStateChangeAs4(Bgp4MpStateChange),
    Bgp4MpMessage(Bgp4MpMessage),
    Bgp4MpMessageLocal(Bgp4MpMessage),
    Bgp4MpMessageAs4(Bgp4MpMessage),
    Bgp4MpMessageAs4Local(Bgp4MpMessage),
}

#[derive(Debug)]
pub struct Bgp4MpStateChange {
    pub peer_asn: Asn,
    pub local_asn: Asn,
    pub interface_index: u16,
    pub address_family: Afi,
    pub peer_addr: IpAddr,
    pub local_addr: IpAddr,
    pub old_state: BgpState,
    pub new_state: BgpState,
}

#[derive(Debug)]
pub struct Bgp4MpStateChangeAs4 {
    pub peer_asn: Asn,
    pub local_asn: Asn,
    pub interface_index: u16,
    pub address_family: Afi,
    pub peer_addr: IpAddr,
    pub local_addr: IpAddr,
    pub old_state: BgpState,
    pub new_state: BgpState,
}

#[derive(Debug, Primitive)]
pub enum BgpState {
    Idle = 1,
    Connect = 2,
    Active = 3,
    OpenSent = 4,
    OpenConfirm = 5,
    Established = 6,
}

#[derive(Debug)]
pub struct Bgp4MpMessage {
    pub peer_asn: Asn,
    pub local_asn: Asn,
    pub interface_index: u16,
    pub afi: Afi,
    pub peer_ip: IpAddr,
    pub local_ip: IpAddr,
    pub bgp_message: BgpMessage
}

#[derive(Debug, Primitive)]
pub enum Bgp4MpType {
    Bgp4MpStateChange = 0,
    Bgp4MpMessage = 1,
    Bgp4MpMessageAs4 = 4,
    Bgp4MpStateChangeAs4 = 5,
    Bgp4MpMessageLocal = 6,
    Bgp4MpMessageAs4Local = 7,
    Bgp4MpMessageAddpath = 8,
    Bgp4MpMessageAs4Addpath = 9,
    Bgp4MpMessageLocalAddpath = 10,
    Bgp4MpMessageLocalAs4Addpath = 11,
}

