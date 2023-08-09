use std::net::{Ipv4Addr, SocketAddrV4};

pub const HOSTNAME: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8081);
