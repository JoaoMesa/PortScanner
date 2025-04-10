use std::net::{ToSocketAddrs, IpAddr};

pub fn resolve_target(target: &str) -> Result<IpAddr, String> {
    (target, 0)
        .to_socket_addrs()
        .map_err(|e| format!("Erro de DNS: {}", e))?
        .find(|addr| addr.is_ipv4())
        .map(|addr| addr.ip())
        .ok_or_else(|| "Nenhum endereço IPv4 encontrado".into())
}

