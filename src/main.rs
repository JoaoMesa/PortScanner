use crate::scanner::PortScanner;
mod scanner;
mod methods;
mod utils;

fn main() {
    /*let udp_ports = vec![
        53,    // DNS
        123,   // NTP
        161,   // SNMP
        500,   // IPSec
        1900,  // UPnP
        5353   // mDNS
    ];*/

    let scanner = PortScanner::new(
	      "felix.nmap.org",
	      1000,
        vec![80, 443, 22, 8080],
        "tcp-connect"
    ).unwrap();
    
    scanner.scan();
}
