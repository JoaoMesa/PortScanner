use std::net::IpAddr;
use std::time::Duration;

pub mod tcp;
pub mod udp;

// Trait comum para todos os métodos de scan
pub trait ScanMethod {
    fn scan(&self, target: IpAddr, port: u16) -> bool;
    fn set_timeout(&mut self, timeout: Duration);
}
