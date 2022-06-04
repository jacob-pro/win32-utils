use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use windows::Win32::Networking::WinSock::{
    AF_INET, AF_INET6, SOCKADDR_IN, SOCKADDR_IN6, SOCKADDR_INET, SOCKET_ADDRESS,
};

pub trait ToStdIpAddr {
    fn to_ip_addr(&self) -> IpAddr;
}

impl ToStdIpAddr for SOCKET_ADDRESS {
    fn to_ip_addr(&self) -> IpAddr {
        unsafe {
            match (*self.lpSockaddr).sa_family {
                x if x == AF_INET.0 as u16 => {
                    let addr = &*(self.lpSockaddr as *const SOCKADDR_IN);
                    IpAddr::V4(Ipv4Addr::from(u32::from_be(addr.sin_addr.S_un.S_addr)))
                }
                x if x == AF_INET6.0 as u16 => {
                    let addr = &*(self.lpSockaddr as *const SOCKADDR_IN6);
                    IpAddr::V6(Ipv6Addr::from(addr.sin6_addr.u.Byte))
                }
                _ => panic!("Unknown sa_family"),
            }
        }
    }
}

impl ToStdIpAddr for SOCKADDR_INET {
    fn to_ip_addr(&self) -> IpAddr {
        unsafe {
            match self.si_family {
                x if x == AF_INET.0 as u16 => {
                    IpAddr::V4(Ipv4Addr::from(u32::from_be(self.Ipv4.sin_addr.S_un.S_addr)))
                }
                x if x == AF_INET6.0 as u16 => {
                    IpAddr::V6(Ipv6Addr::from(self.Ipv6.sin6_addr.u.Byte))
                }
                _ => panic!("Unknown si_family"),
            }
        }
    }
}
