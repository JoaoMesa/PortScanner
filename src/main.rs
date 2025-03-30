mod scanner;

use scanner::PortScanner;

fn main(){

    let scanner = PortScanner::new("scanme.nmap.org", 1000, vec![15, 16,17, 1, 22, 80, 443]).unwrap();

    scanner.scan();  // Agora as portas já fazem parte do scanner
}
