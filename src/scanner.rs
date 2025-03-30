use std::net::{ToSocketAddrs, TcpStream, SocketAddr, IpAddr};
use std::time::Duration;

pub struct PortScanner {
    target_ip: IpAddr,
    timeout: Duration,
    ports: Vec<u16>,
}

impl PortScanner {
    pub fn new(target: &str, timeout_ms: u64, ports: Vec<u16>) -> Option<Self> {
        let addrs: Vec<_> = (target, 0).to_socket_addrs().ok()?.collect();
        let target_ip = addrs.iter()
            .find(|addr| addr.is_ipv4())
            .map(|addr| addr.ip())?;

        Some(Self {
            target_ip,
            timeout: Duration::from_millis(timeout_ms),
            ports,
        })
    }

    fn scan_port(&self, port: u16) -> bool {
        let addr = SocketAddr::new(self.target_ip, port);
        TcpStream::connect_timeout(&addr, self.timeout).is_ok()
    }

    pub fn scan(&self) {
        for &port in &self.ports {
            let status = if self.scan_port(port) { "OPEN" } else { "CLOSED" };
            println!("Port {}: {}", port, status);
        }
    }
}
