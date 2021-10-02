/*!
`bgp-models` is a library that defines the basic BGP and MRT message data structures.
This library aims to provide building blocks for downstreams libraries working with BGP and MRT
messages such as MRT parser or BGP table constructor.

Most of the structs defined in this library are named after the formal definitions in a number of
RFCs. Here is a list of them:

- [RFC 4271](https://datatracker.ietf.org/doc/html/rfc4271): A Border Gateway Protocol 4 (BGP-4)
- [RFC 6793](https://datatracker.ietf.org/doc/html/rfc6793): BGP Support for Four-Octet Autonomous System (AS) Number Space
- [RFC 6396](https://datatracker.ietf.org/doc/html/rfc6396): Multi-Threaded Routing Toolkit (MRT) Routing Information Export Format
- [RFC 8050](https://datatracker.ietf.org/doc/html/rfc8050): Multi-Threaded Routing Toolkit (MRT) Routing Information Export Format with BGP Additional Path Extensions
 */

#![allow(dead_code)]

pub mod bgp;
pub mod network;
pub mod mrt;

#[macro_use]
extern crate enum_primitive_derive;

