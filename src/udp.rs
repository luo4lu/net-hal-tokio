use addr_hal::ToSocketAddrs;
use async_trait::async_trait;
use net_hal::udp::{UdpSocket, UdpServer};
use addr_hal::SocketAddr;

//use tokio::net::UdpSocket;

use crate::addr;

//#[derive(PartialOrd, PartialEq, Debug)]
pub struct TokioUdpSocket {
    inner: tokio::net::UdpSocket,
}

#[async_trait]
impl UdpSocket for TokioUdpSocket {
    type SA4 = addr::SocketV4Inner;
    type SA6 = addr::SocketV6Inner;
    type Error = tokio::io::Error;
    async fn connect(&self, addr: SocketAddr<Self::SA4, Self::SA6>) -> Result<(), Self::Error>
    {
    //    let mut a = match addr.to_socket_addrs(){
    //        Ok(s) => s,
    //        Err(error) => panic!("to socket addrs return addr error:{:?}",error),
    //    };

       match addr {
            SocketAddr::V4(v) => self.inner.connect(v.inner.inner).await,
            SocketAddr::V6(v) => self.inner.connect(v.inner.inner).await,
        }
        
    } 

    async fn send(&mut self, buffer: &[u8]) -> Result<usize, Self::Error>
    {
        self.inner.send(buffer).await
    }

    async fn recv(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error>
    {
        self.inner.recv(buffer).await
    }
}

//#[derive(PartialOrd, PartialEq,Debug)]
pub struct UdpServerinner {}

#[async_trait]
impl UdpServer for UdpServerinner {
    type SA4 = addr::SocketV4Inner;
    type SA6 = addr::SocketV6Inner;
    type Error = tokio::io::Error;
    type BindResult = TokioUdpSocket;

    async fn bind(addr: SocketAddr<Self::SA4, Self::SA6>) -> Result<Self::BindResult, Self::Error>
    {
        let r = match addr{
            SocketAddr::V4(v) =>tokio::net::UdpSocket::bind(v.inner.inner).await,
            SocketAddr::V6(v) =>tokio::net::UdpSocket::bind(v.inner.inner).await,
        };
        let s = TokioUdpSocket {inner: r.unwrap()};
        Ok(s)
    }
}
