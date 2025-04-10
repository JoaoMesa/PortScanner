use std::net::{IpAddr, UdpSocket};
use std::time::Duration;
use super::ScanMethod;

pub struct UdpScanner {
    timeout: Duration,
}

impl UdpScanner {
    pub fn new(timeout: Duration) -> Self {
        Self { timeout }
    }
}

impl ScanMethod for UdpScanner {
    fn scan(&self, target: IpAddr, port: u16) -> bool {
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        socket.set_read_timeout(Some(self.timeout)).unwrap();

        let _ = socket.send_to(&[0; 1], (target, port));
        let mut buf = [0; 1];
        socket.recv_from(&mut buf).is_ok()
    }

    fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }
}
