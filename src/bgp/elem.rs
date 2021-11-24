use std::fmt::{Display, Formatter};
use std::net::IpAddr;
use std::str::FromStr;
use itertools::Itertools;
use crate::bgp::attributes::{AsPath, AtomicAggregate, Community, Origin};
use crate::network::{Asn, NetworkPrefix};

/// Element type.
///
/// - ANNOUNCE: announcement/reachable prefix
/// - WITHDRAW: withdrawn/unreachable prefix
#[derive(Debug, Clone, Copy)]
pub enum ElemType {
    ANNOUNCE,
    WITHDRAW,
}

/// BgpElem represents per-prefix BGP element.
///
/// The information is for per announced/withdrawn prefix.
///
/// Note: it consumes more memory to construct BGP elements due to duplicate information
/// shared between multiple elements of one MRT record.
#[derive(Debug, Clone)]
pub struct BgpElem {
    pub timestamp: f64,
    pub elem_type: ElemType,
    pub peer_ip: IpAddr,
    pub peer_asn: Asn,
    pub prefix: NetworkPrefix,
    pub next_hop: Option<IpAddr>,
    pub as_path: Option<AsPath>,
    pub origin_asns: Option<Vec<Asn>>,
    pub origin: Option<Origin>,
    pub local_pref: Option<u32>,
    pub med: Option<u32>,
    pub communities: Option<Vec<Community>>,
    pub atomic: Option<AtomicAggregate>,
    pub aggr_asn: Option<Asn>,
    pub aggr_ip: Option<IpAddr>,
}

impl Default for BgpElem {
    fn default() -> Self {
        BgpElem {
            timestamp: 0.0,
            elem_type: ElemType::ANNOUNCE,
            peer_ip: IpAddr::from_str("0.0.0.0").unwrap(),
            peer_asn: 0,
            prefix: NetworkPrefix::from_str("0.0.0.0/0").unwrap(),
            next_hop: None,
            as_path: None,
            origin_asns: None,
            origin: None,
            local_pref: None,
            med: None,
            communities: None,
            atomic: None,
            aggr_asn: None,
            aggr_ip: None
        }
    }
}

#[inline(always)]
pub fn option_to_string<T>(o: &Option<T>) -> String
    where
        T: Display,
{
    if let Some(v) = o {
        v.to_string()
    } else {
        String::new()
    }
}

#[inline(always)]
pub fn option_to_string_communities(o: &Option<Vec<Community>>) -> String {
    if let Some(v) = o {
        v.iter()
            .join(" ")
    } else {
        String::new()
    }
}

impl Display for BgpElem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let t = match self.elem_type {
            ElemType::ANNOUNCE => "A",
            ElemType::WITHDRAW => "W",
        };
        let format = format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            t, &self.timestamp,
            &self.peer_ip,
            &self.peer_asn,
            &self.prefix,
            option_to_string(&self.as_path),
            option_to_string(&self.origin),
            option_to_string(&self.next_hop),
            option_to_string(&self.local_pref),
            option_to_string(&self.med),
            option_to_string_communities(&self.communities),
            option_to_string(&self.atomic),
            option_to_string(&self.aggr_asn),
            option_to_string(&self.aggr_ip),
        );
        write!(f, "{}", format)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::default::Default;
    use super::*;

    #[test]
    fn test_default() {
        let elem = BgpElem{
            timestamp: 0.0,
            elem_type: ElemType::ANNOUNCE,
            peer_ip: IpAddr::from_str("192.168.1.1").unwrap(),
            peer_asn: 0,
            prefix: NetworkPrefix::from_str("8.8.8.0/24").unwrap(),
            ..Default::default()
        };
        dbg!(elem);
    }
}