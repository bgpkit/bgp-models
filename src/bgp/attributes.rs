//! BGP attribute structs
use core::fmt;
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};
use std::net::Ipv4Addr;
use log::warn;
use crate::network::*;

/// The high-order bit (bit 0) of the Attribute Flags octet is the
/// Optional bit.  It defines whether the attribute is optional (if
/// set to 1) or well-known (if set to 0).
///
/// The second high-order bit (bit 1) of the Attribute Flags octet
/// is the Transitive bit.  It defines whether an optional
/// attribute is transitive (if set to 1) or non-transitive (if set
/// to 0).
///
/// For well-known attributes, the Transitive bit MUST be set to 1.
/// (See Section 5 for a discussion of transitive attributes.)
///
/// The third high-order bit (bit 2) of the Attribute Flags octet
/// is the Partial bit.  It defines whether the information
/// contained in the optional transitive attribute is partial (if
/// set to 1) or complete (if set to 0).  For well-known attributes
/// and for optional non-transitive attributes, the Partial bit
/// MUST be set to 0.
///
/// The fourth high-order bit (bit 3) of the Attribute Flags octet
/// is the Extended Length bit.  It defines whether the Attribute
/// Length is one octet (if set to 0) or two octets (if set to 1).
pub enum AttributeFlagsBit {
    /// 128 = 0b10000000
    OptionalBit = 0b10000000,
    /// 64 = 0b01000000
    TransitiveBit = 0b01000000,
    /// 32 = 0b00100000
    PartialBit = 0b00100000,
    /// 16 = 0b00010000
    ExtendedLengthBit = 0b00010000,
}

/// Attribute types.
///
/// <https://tools.ietf.org/html/rfc427>
/// ```text
/// Name               Value       Definition
/// ----               -----       ----------
/// ORIGIN              1          See Section 5.1.1
/// AS_PATH             2          See Section 5.1.2
/// NEXT_HOP            3          See Section 5.1.3
/// MULTI_EXIT_DISC     4          See Section 5.1.4
/// LOCAL_PREF          5          See Section 5.1.5
/// ATOMIC_AGGREGATE    6          See Section 5.1.6
/// AGGREGATOR          7          See Section 5.1.7
/// ```
///
/// <https://tools.ietf.org/html/rfc4760>
/// Name               Value
/// ----               -----
/// MP_REACH_NLRI      14
/// MP_UNREACH_NLRI    15
///
/// All attributes: <https://www.iana.org/assignments/bgp-parameters/bgp-parameters.xhtml#bgp-parameters-2>
#[allow(non_camel_case_types)]
#[derive(Debug, Primitive, PartialEq, Eq, Hash, Clone)]
pub enum AttrType {
    ORIGIN = 1,
    AS_PATH = 2,
    NEXT_HOP = 3,
    MULTI_EXIT_DISCRIMINATOR = 4,
    LOCAL_PREFERENCE = 5,
    ATOMIC_AGGREGATE = 6,
    AGGREGATOR = 7,
    COMMUNITIES = 8,
    /// <https://tools.ietf.org/html/rfc4456>
    ORIGINATOR_ID = 9,
    CLUSTER_LIST = 10,
    /// <https://tools.ietf.org/html/rfc4760>
    CLUSTER_ID = 13,
    MP_REACHABLE_NLRI = 14,
    MP_UNREACHABLE_NLRI = 15,
    EXTENDED_COMMUNITIES = 16,
    AS4_PATH = 17,
    AS4_AGGREGATOR = 18,
    LARGE_COMMUNITIES = 32,
    // FIXME: 33 is BGPsec_Path
    ATTRIBUTES_END = 33,
    UNASSINGED = 39,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Primitive, PartialEq, Eq, Hash, Clone)]
pub enum Origin {
    IGP = 0,
    EGP = 1,
    INCOMPLETE = 2,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Primitive, PartialEq, Eq, Hash, Clone)]
pub enum AtomicAggregate {
    NAG = 0,
    AG = 1,
}

#[derive(Debug, PartialEq)]
pub enum Attribute {
    Origin(Origin),
    AsPath(AsPath),
    NextHop(Ipv4Addr),
    MultiExitDiscriminator(u32),
    LocalPreference(u32),
    AtomicAggregate(AtomicAggregate),
    Aggregator(Asn, Ipv4Addr),
    Communities(Vec<Community>),
    LargeCommunities(Vec<LargeCommunity>),
    OriginatorId(Ipv4Addr),
    Clusters(Vec<Ipv4Addr>),
    MpReachableNlri(Nlri),
    MpUnreachableNlri(Nlri),
    As4Aggregator(Asn, Ipv4Addr),
    As4Path(AsPath),
}

// Attribute hasher

#[derive(Default)]
pub struct AttributeHasher {
    value: u64,
}

impl Hasher for AttributeHasher {
    fn finish(&self) -> u64 {
        self.value
    }

    fn write(&mut self, bytes: &[u8]) {
        // if bytes.len() != 1 {
        //     panic!("Trying to hash slice of size {:?}", bytes.len());
        // }
        self.value |= bytes[0] as u64;
    }
}

#[derive(Debug)]
pub struct Attributes {
    pub map: AttributeMap,
}

pub type AttributeMap = HashMap<AttrType, Attribute, BuildHasherDefault<AttributeHasher>>;

/////////////
// AS PATH //
/////////////

/// Enum of AS path segment.
#[derive(Debug, PartialEq, Clone)]
pub enum AsPathSegment {
    AsSequence(Vec<Asn>),
    AsSet(Vec<Asn>),
    ConfedSequence(Vec<Asn>),
    ConfedSet(Vec<Asn>),
}

impl AsPathSegment {
    pub fn count_asns(&self) -> usize {
        match self {
            AsPathSegment::AsSequence(v) => {
                v.len()
            },
            AsPathSegment::AsSet(_) => 1,
            AsPathSegment::ConfedSequence(_) | AsPathSegment::ConfedSet(_)=> 0,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AsPath {
    pub segments: Vec<AsPathSegment>,
}

impl AsPath {
    pub fn new() -> AsPath {
        AsPath { segments: vec![] }
    }

    pub fn from_segments(segments: Vec<AsPathSegment>) -> AsPath {
        AsPath { segments }
    }

    pub fn add_segment(&mut self, segment: AsPathSegment) {
        self.segments.push(segment);
    }

    pub fn segments(&self) -> &Vec<AsPathSegment> {
        &self.segments
    }

    pub fn count_asns(&self) -> usize {
        self.segments.iter().map(AsPathSegment::count_asns).sum()
    }


    /// Construct AsPath from AS_PATH and AS4_PATH
    ///
    /// https://datatracker.ietf.org/doc/html/rfc6793#section-4.2.3
    ///    If the number of AS numbers in the AS_PATH attribute is less than the
    ///    number of AS numbers in the AS4_PATH attribute, then the AS4_PATH
    ///    attribute SHALL be ignored, and the AS_PATH attribute SHALL be taken
    ///    as the AS path information.
    ///
    ///    If the number of AS numbers in the AS_PATH attribute is larger than
    ///    or equal to the number of AS numbers in the AS4_PATH attribute, then
    ///    the AS path information SHALL be constructed by taking as many AS
    ///    numbers and path segments as necessary from the leading part of the
    ///    AS_PATH attribute, and then prepending them to the AS4_PATH attribute
    ///    so that the AS path information has a number of AS numbers identical
    ///    to that of the AS_PATH attribute.  Note that a valid
    ///    AS_CONFED_SEQUENCE or AS_CONFED_SET path segment SHALL be prepended
    ///    if it is either the leading path segment or is adjacent to a path
    ///    segment that is prepended.
    pub fn merge_aspath_as4path(aspath: Option<&Attribute>, as4path: Option<&Attribute>) -> Option<AsPath> {
        if let (None, None) = (aspath, as4path) {
            return None
        } else if let (Some(Attribute::AsPath(v)), None) = (aspath, as4path) {
            return Some(v.clone())
        } else if let (None, Some(Attribute::As4Path(v))) = (aspath, as4path) {
            return Some(v.clone())
        } else if let (Some(Attribute::AsPath(aspath)), Some(Attribute::As4Path(as4path))) = (aspath, as4path) {
            if aspath.count_asns() < as4path.count_asns() {
                return Some(aspath.clone())
            } else {
                let mut as4iter = as4path.segments.iter();
                let mut as4seg = as4iter.next();
                let mut new_segs: Vec<AsPathSegment> = vec![];
                if as4seg.is_none(){
                    new_segs.extend(aspath.segments.clone());
                    return Some(AsPath{ segments: new_segs })
                }

                for seg in &aspath.segments {
                    let as4seg_unwrapped = as4seg.unwrap();
                    if let (AsPathSegment::AsSequence(seq), AsPathSegment::AsSequence(seq4)) = (seg, as4seg_unwrapped) {
                        let diff_len = seq.len() - seq4.len();
                        let mut new_seq: Vec<Asn> = vec![];
                        new_seq.extend(seq.iter().take(diff_len));
                        new_seq.extend(seq4);
                        new_segs.push(AsPathSegment::AsSequence(new_seq));
                    } else {
                        new_segs.push(as4seg_unwrapped.clone());
                    }
                    as4seg = as4iter.next();
                }
                return Some(AsPath{ segments: new_segs })
            }
        } else {
            warn!("unable to merge aspath and as4path");
            None
        }
    }
}

impl fmt::Display for AsPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (index, segment) in self.segments.iter().enumerate() {
            write!(f, "{}", segment)?;
            if index != self.segments.len() - 1 {
                f.write_str(" ")?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for AsPathSegment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut write_vec = |v: &Vec<Asn>, separator, prefix, suffix| {
            f.write_str(prefix)?;
            if v.len() > 0 {
                write!(f, "{}", &v[0])?;
                for i in &v[1..] {
                    write!(f, "{}{}", separator, i)?;
                }
            }
            f.write_str(suffix)
        };
        match self {
            AsPathSegment::AsSequence(ref s) | AsPathSegment::ConfedSequence(ref s) => {
                write_vec(s, " ", "", "")
            }
            AsPathSegment::AsSet(ref s) | AsPathSegment::ConfedSet(ref s) => {
                write_vec(s, ", ", "{ ", " }")
            }
        }
    }
}

/////////////////
// COMMUNITIES //
/////////////////

#[derive(Debug, PartialEq)]
pub enum Community {
    NoExport,
    NoAdvertise,
    NoExportSubConfed,
    Custom(Asn, u16),
}

#[derive(Debug, PartialEq)]
pub struct LargeCommunity {
    global_administrator: u32,
    local_data: [u32; 2],
}

impl LargeCommunity {
    pub fn new(global_administrator: u32, local_data: [u32; 2]) -> LargeCommunity {
        LargeCommunity {
            global_administrator,
            local_data,
        }
    }
}

//////////
// NLRI //
//////////

#[derive(Debug, PartialEq)]
pub struct Nlri {
    pub afi: Afi,
    pub safi: Safi,
    pub next_hop: Option<NextHopAddress>,
    pub prefixes: Vec<NetworkPrefix>,
}

#[derive(Debug, PartialEq)]
pub struct MpReachableNlri {
    afi: Afi,
    safi: Safi,
    next_hop: NextHopAddress,
    prefixes: Vec<NetworkPrefix>,
}

impl MpReachableNlri {
    pub fn new(
        afi: Afi,
        safi: Safi,
        next_hop: NextHopAddress,
        prefixes: Vec<NetworkPrefix>,
    ) -> MpReachableNlri {
        MpReachableNlri {
            afi,
            safi,
            next_hop,
            prefixes,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct MpReachableNlriV2 {
    next_hop: NextHopAddress,
}

#[derive(Debug, PartialEq)]
pub struct MpUnreachableNlri {
    afi: Afi,
    safi: Safi,
    prefixes: Vec<NetworkPrefix>,
}

impl MpUnreachableNlri {
    pub fn new(afi: Afi, safi: Safi, prefixes: Vec<NetworkPrefix>) -> MpUnreachableNlri {
        MpUnreachableNlri {
            afi,
            safi,
            prefixes,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aspath_as4path_merge() {
        let aspath = AsPath{
            segments: vec![AsPathSegment::AsSequence([1,2,3,5].to_vec())]
        };
        let as4path = AsPath{
            segments: vec![AsPathSegment::AsSequence([2,3,7].to_vec())]
        };
        let newpath = AsPath::merge_aspath_as4path(Some(&Attribute::AsPath(aspath)), Some(&Attribute::As4Path(as4path))).unwrap();
        assert_eq!(newpath.segments[0], AsPathSegment::AsSequence([1,2,3,7].to_vec()));
    }
}