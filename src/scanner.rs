use std::net::IpAddr;
use std::time::Duration;
use crate::methods::{ScanMethod, tcp::TcpConnectScanner, udp::UdpScanner};
use crate::utils::net;
use crate::methods::tcp::TcpSynScanner;

pub struct PortScanner {
    target_ip: IpAddr,
    scanner: Box<dyn ScanMethod>,
    ports: Vec<u16>,
}

impl PortScanner {
    pub fn new(
        target: &str, 
        timeout_ms: u64, 
        ports: Vec<u16>,
        scan_type: &str
        ) -> Result<Self, String> {
        let target_ip = net::resolve_target(target)?;
        let timeout = Duration::from_millis(timeout_ms);

        let scanner: Box<dyn ScanMethod> = match scan_type.to_lowercase().as_str(){
            "tcp-connect" => Box::new(TcpConnectScanner::new(timeout)),
            "tcp-syn" => Box::new(TcpSynScanner::new(timeout)),
            "udp" => Box::new(UdpScanner::new(timeout)),
            _ => return Err(format!("Tipo de scan inválido")),
        };
        Ok(Self {
            target_ip,
            scanner,
            ports,
        })
    }

    pub fn scan(&self) {
        println!("Scanning {}", self.target_ip);

        for &port in &self.ports{
            let status = if self.scanner.scan(self.target_ip, port){
                "OPEN"
            } else{
                "CLOSED"
            };
            println!("Porta {:5}: {}", port, status);
        }
    }
}
