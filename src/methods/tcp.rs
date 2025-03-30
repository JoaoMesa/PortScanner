use std::net::TcpStream;
use std::time:: Duration;

impl PortScanner {
    fn scan_port(&self, port: u16) -> bool {
        let addr = format!("{}:{}", self.target, port);
        let timeout = Duration::from_millis(self.timeout);


        match TcpStream::connect_timeout(&addr.parse().unwrap(), timeout){
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
