use crate::mrt::bgp4mp::Bgp4Mp;
use crate::mrt::tabledump::{TableDumpMessage, TableDumpV2Message};

pub mod tabledump;
pub mod bgp4mp;

/// MRT record entry-point struct.
///
/// A MRT record is constcuted as the following:
///  0                   1                   2                   3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                           Timestamp                           |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |             Type              |            Subtype            |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                             Length                            |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                      Message... (variable)
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///
///
/// Or with extended timestamp:
///  0                   1                   2                   3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                           Timestamp                           |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |             Type              |            Subtype            |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                             Length                            |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                      Microsecond Timestamp                    |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                      Message... (variable)
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
#[derive(Debug)]
pub struct MrtRecord {
    pub common_header: CommonHeader,
    pub message: MrtMessage,
}

#[derive(Debug)]
pub enum MrtMessage {
    TableDumpMessage(TableDumpMessage),
    TableDumpV2Message(TableDumpV2Message),
    Bgp4Mp(Bgp4Mp),
}

#[derive(Debug, Primitive)]
#[allow(non_camel_case_types)]
pub enum EntryType {
    // START DEPRECATED
    NULL = 0,
    START = 1,
    DIE = 2,
    I_AM_DEAD = 3,
    PEER_DOWN = 4,
    BGP = 5,
    RIP = 6,
    IDRP = 7,
    RIPNG = 8,
    BGP4PLUS = 9,
    BGP4PLUS_01 = 10,
    // END DEPRECATED
    OSPFv2 = 11,
    TABLE_DUMP = 12,
    TABLE_DUMP_V2 = 13,
    BGP4MP = 16,
    BGP4MP_ET = 17,
    ISIS = 32,
    ISIS_ET = 33,
    OSPFv3 = 48,
    OSPFv3_ET = 49,
}

/// MRT common header
///
/// The headers include the following:
/// - timestamp: 32 bits
/// - entry_type: [EntryType] enum
/// - entry_subtype: entry subtype
/// - length: length of the message in octets
/// - (`ET` type only) microsecond_timestamp: microsecond part of the timestamp.
///   only applicable to the MRT message type with `_ET` suffix, such as
///   `BGP4MP_ET`
#[derive(Debug)]
pub struct CommonHeader {
    pub timestamp: u32,
    pub microsecond_timestamp: Option<u32>,
    pub entry_type: EntryType,
    pub entry_subtype: u16,
    pub length: u32,
}

