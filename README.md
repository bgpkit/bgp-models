# bgp-models

[![Rust](https://github.com/bgpkit/bgp-models/actions/workflows/rust.yml/badge.svg)](https://github.com/bgpkit/bgp-models/actions/workflows/rust.yml)

`bgp-models` is a library that defines the basic BGP and MRT message data structures.
This library aims to provide building blocks for downstreams libraries working with BGP and MRT
messages such as MRT parser or BGP table constructor.

## Supported RFCs

Most of the structs defined in this library are named after the formal definitions in a number of
RFCs. Here is a list of them:

### BGP
- [X] [RFC 4271](https://datatracker.ietf.org/doc/html/rfc4271): A Border Gateway Protocol 4 (BGP-4)
- [X] [RFC 6793](https://datatracker.ietf.org/doc/html/rfc6793): BGP Support for Four-Octet Autonomous System (AS) Number Space

### MRT

- [X] [RFC 6396](https://datatracker.ietf.org/doc/html/rfc6396): Multi-Threaded Routing Toolkit (MRT) Routing Information Export Format
- [ ] [RFC 6397](https://datatracker.ietf.org/doc/html/rfc6397): Multi-Threaded Routing Toolkit (MRT) Border Gateway Protocol (BGP) Routing Information Export Format with Geo-Location Extensions
- [X] [RFC 8050](https://datatracker.ietf.org/doc/html/rfc8050): Multi-Threaded Routing Toolkit (MRT) Routing Information Export Format with BGP Additional Path Extensions

### Communities

#### Communities

- [X] [RFC 1977](https://datatracker.ietf.org/doc/html/rfc1977): BGP Communities Attribute

#### Extended Communities

- [X] [RFC 4360](https://datatracker.ietf.org/doc/html/rfc4360): BGP Extended Communities Attribute
- [X] [RFC 5668](https://datatracker.ietf.org/doc/html/rfc5668): 4-Octet AS Specific BGP Extended Community
- [X] [RFC 5701](https://datatracker.ietf.org/doc/html/rfc5701): IPv6 Address Specific BGP Extended Community Attribute
- [X] [RFC 7153](https://datatracker.ietf.org/doc/html/rfc7153): IANA Registries for BGP Extended Communities Updates 4360, 5701
- [X] [RFC 8097](https://datatracker.ietf.org/doc/html/rfc8097): BGP Prefix Origin Validation State Extended Community

#### Large Communities

- [X] [RFC 8092](https://datatracker.ietf.org/doc/html/rfc8092): BGP Large Communities

#### Other Informational

- [RFC 4384](https://datatracker.ietf.org/doc/html/rfc4384): BGP Communities for Data Collection BCP 114
- [RFC 8195](https://datatracker.ietf.org/doc/html/rfc8195): Use of BGP Large Communities (informational)
- [RFC 8642](https://datatracker.ietf.org/doc/html/rfc8642): Policy Behavior for Well-Known BGP Communities

## Used By

- [bgpkit-parser](https://github.com/bgpkit/bgpkit-parser)
- [ris-live-rs](https://github.com/bgpkit/ris-live-rs)

## Built with ❤️ by BGPKIT Team

BGPKIT is a small-team start-up that focus on building the best tooling for BGP data in Rust. We have 10 years of 
experience working with BGP data and believe that our work can enable more companies to start keeping tracks of BGP data
on their own turf. Learn more about what services we provide at https://bgpkit.com.

<a href="https://bgpkit.com"><img src="https://bgpkit.com/Original%20Logo%20Cropped.png" alt="https://bgpkit.com/favicon.ico" width="200"/></a>
