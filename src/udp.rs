use addr_hal::{ToSocketAddrs, SocketAddressV4, SocketAddressV6};
use net_hal::UdpHandler;
use net_hal::UdpServer;
use async_trait::async_trait;

use std::net::UdpSocket;
use std::error::Error;

#[derive(Clone, Copy, PartialOrd, PartialEq, Eq, Ord,Debug)]
pub struct UdpHandlerinner{
    inner:UdpSocket,
}

#[async_trait]
impl UdpHandler for UdpHandlerinner{
    type SA4 = dyn SocketAddressV4;
    type SA6 = dyn SocketAddressV6;
    type Error = dyn Error;

    async fn connect<A>(addr: A) where A: ToSocketAddrs<Self::SA4, Self::SA6>{
        Self{
            inner: UdpSocket::connect(addr).await
        };
    }
}

#[derive(Clone, Copy, PartialOrd, PartialEq, Eq, Ord,Debug)]
pub struct UdpServerinner{
    inner:UdpServer,
}

#[async_trait]
impl UdpServer for UdpServerinner{
    type SA4 = dyn SocketAddressV4;
    type SA6 = dyn SocketAddressV6;
    type Error = dyn Error;

    async fn bind<A>(addr: A) where A: ToSocketAddrs<Self::SA4, Self::SA6>{
        Self{
            inner: UdpSocket::bind(addr).await
        };
    }
}