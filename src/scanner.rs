use std::net::TcpStream;
use std::time:: Duration;
use std::net::SocketAddr;

pub struct PortScanner {
    target: String,
    ports: Vec<u16>,
    timeout: u64,
}

impl PortScanner {
    pub fn new(target: &str, ports: Vec<u16>, timeout: u64) -> Self {
        Self {
            target: target.to_string(),
            ports,
            timeout,
        }
    }

    pub fn scan(&self) {
        for &port in &self.ports {
            if self.scan_port(port){
                println!("Port {} OPEN", port);
            } else{
                println!("Port {} CLOSED", port);
            }
            
        }
    }

    pub fn scan_port(&self, port: u16) -> bool {
        let addr = format!("{}:{}", self.target.trim(), port);
        let timeout = Duration::from_millis(self.timeout);


        /*match TcpStream::connect_timeout(&addr.parse().unwrap(), timeout){
            Ok(_) => true,
            Err(_) => false,
        }*/
        match addr.parse::<SocketAddr>() {
            Ok(socket_addr) => {
                TcpStream::connect_timeout(&socket_addr, timeout).is_ok()
            },
            Err(_) => {
                eprintln!("Invalid address format: {}", addr);
                false
            }
        }
    }
}
