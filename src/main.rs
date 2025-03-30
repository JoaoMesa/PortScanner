mod scanner;

fn main() {
           let my_scanner = scanner::PortScanner::new("scanme.nmap.org", vec![22, 80, 443, 8000, 27000], 300);
    my_scanner.scan();
}


