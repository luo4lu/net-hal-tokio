use addr_hal::SocketAddressV4;
use addr_hal::SocketAddressV6;
use addr_hal::Ipv4Address;

use std::net::SocketAddrV4;
use std::net::Ipv4Addr;

#[derive(Clone, Copy, PartialOrd, PartialEq, Eq, Ord)]
pub struct IpAddressV4Inner {
    inner: Ipv4Addr,
}

impl Ipv4Address for IpAddressV4Inner {
    const LOCALHOST: Self = IpAddressV4Inner { inner: Ipv4Addr::new(127,0,0,1) };
    const UNSPECIFIED: Self = IpAddressV4Inner { inner: Ipv4Addr::new(0,0,0,0) };
    const BROADCAST: Self = IpAddressV4Inner { inner: Ipv4Addr::new(255,255,255,255) };

    fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self {
            inner: Ipv4Addr::new(a,b,c,d)
        }
    }

    fn octets(&self) -> [u8; 4] {
        self.inner.octets()
    }
}

pub struct SocketV4Inner {
    inner: SocketAddrV4,
}

impl SocketAddressV4 for SocketV4Inner {
    type IpAddress = IpAddressV4Inner;

    fn new(ip: addr_hal::Ipv4Addr<Self::IpAddress>, port: u16) -> Self {
        let oct = ip.octets();
        let std_ipv4 = Ipv4Addr::new(oct[0], oct[1], oct[2], oct[3]);
        Self {
            inner: SocketAddrV4::new(std_ipv4, port)
        }
    }

    fn ip(&self) -> &Ipv4Addr<Self::IpAddress> {

    }

    fn set_ip(&mut self, ip: Ipv4Addr<Self::IpAddress>) {

    }

    fn port(&self) -> u16 {

    }

    fn set_port(&mut self, port: u16) {

    }
}


