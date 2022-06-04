use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use windows::Win32::Networking::WinSock::{
    AF_INET, AF_INET6, SOCKADDR_IN, SOCKADDR_IN6, SOCKADDR_INET, SOCKET_ADDRESS,
};

pub trait ToStdSocket {
    fn to_std_socket_addr(&self) -> SocketAddr;
}

impl ToStdSocket for SOCKET_ADDRESS {
    fn to_std_socket_addr(&self) -> SocketAddr {
        unsafe {
            match (*self.lpSockaddr).sa_family {
                af if af == AF_INET.0 as u16 => {
                    let addr = &*(self.lpSockaddr as *const SOCKADDR_IN);
                    let ip = IpAddr::V4(Ipv4Addr::from(u32::from_be(addr.sin_addr.S_un.S_addr)));
                    SocketAddr::new(ip, addr.sin_port)
                }
                af if af == AF_INET6.0 as u16 => {
                    let addr = &*(self.lpSockaddr as *const SOCKADDR_IN6);
                    let ip = IpAddr::V6(Ipv6Addr::from(addr.sin6_addr.u.Byte));
                    SocketAddr::new(ip, addr.sin6_port)
                }
                af => panic!("Unknown sa_family: {:#06x}", af),
            }
        }
    }
}

impl ToStdSocket for SOCKADDR_INET {
    fn to_std_socket_addr(&self) -> SocketAddr {
        unsafe {
            match self.si_family {
                af if af == AF_INET.0 as u16 => {
                    let ip =
                        IpAddr::V4(Ipv4Addr::from(u32::from_be(self.Ipv4.sin_addr.S_un.S_addr)));
                    SocketAddr::new(ip, self.Ipv4.sin_port)
                }
                af if af == AF_INET6.0 as u16 => {
                    let ip = IpAddr::V6(Ipv6Addr::from(self.Ipv6.sin6_addr.u.Byte));
                    SocketAddr::new(ip, self.Ipv6.sin6_port)
                }
                af => panic!("Unknown si_family: {:#06x}", af),
            }
        }
    }
}
