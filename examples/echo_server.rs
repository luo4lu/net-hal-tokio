use net_hal::udp::{UdpSocket, UdpServer};
use addr_hal::SocketAddr;


async fn echo_recv<S: UdpSocket>(socket: &mut S) {
    let mut buf = [0;10];
        let _buf_size = match socket.recv(&mut buf).await {
            Ok(received) => println!("received {} bytes {:?}", received, &buf[..received]),
            Err(_e) => panic!("recv function failed:"),
        };
}

#[tokio::main]
async fn main() {
    use net_hal_tokio::udp::TokioUdpServer;
    
    let mut handle = match TokioUdpServer::bind(SocketAddr::from(([127, 0, 0, 1], 3400))).await{
        Ok(s) => s,
        Err(_error) => panic!("couldn't bind to address"),
    };
    echo_recv(&mut handle).await;
}