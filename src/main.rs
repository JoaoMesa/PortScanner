use crate::scanner::PortScanner;
mod scanner;
mod methods;
mod utils;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version = "1.0", about = "Port scanner CLI tool")]

struct Args {
    #[clap(short, long)]
    target: String,

    #[clap(short, long, default_value = "21,22,23,25,53,80,110,139,143,443,445,3389")]
    ports: String,

    #[clap(short, long, default_value = "tcp-connect")]
    method: String,

    #[clap(short = 'T', long, default_value_t = 1000)]
    timeout: u64,
}

fn parse_ports(ports: &str) -> Result<Vec<u16>, String> {
    let mut result = Vec::new();
  

    for part in ports.split (',') {
        if part.contains('-') {
            let range: Vec<&str> = part.split('-').collect();
            if range.len() != 2 {
                return Err(format!("Intervalo inválido: {}", part));
            }

            let start = range[0].parse::<u16>().map_err(|e| e.to_string())?;
            let end = range[1].parse::<u16>().map_err(|e| e.to_string())?;

            result.extend(start..=end);
        } else {
                let port = part.parse::<u16>().map_err(|e| e.to_string())?;
                result.push(port);
        }
    }

    if result.is_empty() {
        return Err("Nenhuma porta válida".to_string())
    }

    Ok(result)
}

fn main() {

    let args = Args::parse();
    //dbg!(&args.ports);
    
    let parsed_ports = parse_ports(&args.ports)
        .unwrap_or_else(|e| {
            eprintln!("Erro ao converter portas: {}", e);
            std::process::exit(1);
        });

    //dbg!(&parsed_ports);
    
    let scanner = PortScanner::new(
	      &args.target,
	      args.timeout,
        parsed_ports,
        &args.method
    ).unwrap();
    
    scanner.scan();
    
}
