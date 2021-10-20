use std::fmt::{Display, Formatter};
use crate::bgp::attributes::{*};
use crate::network::NextHopAddress;
use itertools::{Itertools};


impl Display for Origin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Origin::IGP => {"IGP"}
            Origin::EGP => {"EGP"}
            Origin::INCOMPLETE => {"INCOMPLETE"}
        };
        write!(f, "{}", s)
    }
}

impl Display for AtomicAggregate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            AtomicAggregate::NAG => {"NAG"}
            AtomicAggregate::AG => {"AG"}
        })
    }
}

const NOEXPORT: &str ="no-export";

impl Display for Community {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Community::NoExport => {
                "no-export".to_string()
            }
            Community::NoAdvertise => {
                "no-advertise".to_string()
            }
            Community::NoExportSubConfed => {
                "no-export-sub-confed".to_string()
            }
            Community::Custom(asn, value) => {
                format!("{}:{}", asn, value)
            }
        }
        )
    }
}

impl Display for NextHopAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
               match self {
                   NextHopAddress::Ipv4(v) => {v.to_string()}
                   NextHopAddress::Ipv6(v) => {v.to_string()}
                   NextHopAddress::Ipv6LinkLocal(v1, _v2) => {v1.to_string()}
               }
        )
    }
}

impl Display for AsPath {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}",
        self
            .segments()
            .iter()
            .map(|seg| match seg {
                AsPathSegment::AsSequence(v) | AsPathSegment::ConfedSequence(v) => v
                    .iter()
                    .join(" "),
                AsPathSegment::AsSet(v) | AsPathSegment::ConfedSet(v) => {
                    format!(
                        "{{{}}}",
                        v.iter()
                            .join(",")
                    )
                }
            })
            .join(" ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aspath_display() {
        let aspath = AsPath{
            segments: vec![AsPathSegment::AsSequence([1,2,3,5].to_vec())]
        };
        let mut paths = vec![];
        for _ in 0..1000000{
            paths.push(aspath.clone());
        }
        for p in paths{
            p.to_string();
        }
    }

}
