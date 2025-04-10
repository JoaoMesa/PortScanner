use std::net::{IpAddr, TcpStream, SocketAddr};
use std::time::Duration;
use super::ScanMethod;

pub struct TcpConnectScanner {
    timeout: Duration,
}

impl TcpConnectScanner {
    pub fn new(timeout: Duration) -> Self {
        Self { timeout }
    }
}

impl ScanMethod for TcpConnectScanner {
    fn scan(&self, target: IpAddr, port: u16) -> bool {
        let addr = SocketAddr::new(target, port);
        TcpStream::connect_timeout(&addr, self.timeout).is_ok()

    } 
    fn set_timeout (&mut self, timeout: Duration) {
        self.timeout = timeout;
    }
}


pub struct TcpSynScanner {
    timeout: Duration,
}

impl TcpSynScanner {
    pub fn new(timeout: Duration) -> Self {
        Self { timeout }
    }
}

impl ScanMethod for TcpSynScanner {
    fn scan(&self, target: IpAddr, port: u16) -> bool {
        unimplemented!()
    }

    fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }
}


