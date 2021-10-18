//! BGP messages and relevant structs.

pub mod attributes;
pub mod display;

use std::net::Ipv4Addr;
use crate::bgp::attributes::Attributes;
use crate::network::*;

#[derive(Debug, Primitive)]
pub enum BgpMessageType {
    OPEN = 1,
    UPDATE = 2,
    NOTIFICATION = 3,
    KEEPALIVE = 4,
}

// https://tools.ietf.org/html/rfc4271#section-4
#[derive(Debug)]
pub enum BgpMessage{
    Open(BgpOpenMessage),
    Update(BgpUpdateMessage),
    Notification(BgpNotificationMessage),
    KeepAlive(BgpKeepAliveMessage),
}

/// BGP Open Message
///
/// ```text
///  0                   1                   2                   3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
///  +-+-+-+-+-+-+-+-+
///  |    Version    |
///  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///  |     My Autonomous System      |
///  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///  |           Hold Time           |
///  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///  |                         BGP Identifier                        |
///  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///  | Opt Parm Len  |
///  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///  |                                                               |
///  |             Optional Parameters (variable)                    |
///  |                                                               |
///  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ```
#[derive(Debug)]
pub struct BgpOpenMessage {
    pub version: u8,
    pub asn: Asn,
    pub hold_time: u16,
    pub sender_ip: Ipv4Addr,
    pub opt_params: Vec<OptParam>
}

#[derive(Debug)]
pub struct OptParam {

}

#[derive(Debug)]
pub struct BgpUpdateMessage {
    pub withdrawn_prefixes: Vec<NetworkPrefix>,
    pub attributes: Attributes,
    pub announced_prefixes: Vec<NetworkPrefix>,
}

#[derive(Debug)]
pub struct BgpNotificationMessage {
    pub error_code: u8,
    pub error_subcode: u8,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct BgpKeepAliveMessage {

}
