use anyhow::Error;
use socket2::{Domain, Protocol, Socket, Type};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub fn get_local_ip_address() -> Result<String, Error> {
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 254, 254, 254)), 1).into();
    socket.connect(&socket_addr)?;
    let addr = socket.local_addr()?;
    match addr.as_socket_ipv4() {
        Some(addr_ipv4) => Ok(addr_ipv4.ip().to_string()),
        None => Err(Error::msg("Unable to get local IP address")),
    }
}
