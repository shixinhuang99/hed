use std::net::IpAddr;

pub fn is_ip(s: &str) -> bool {
	s.parse::<IpAddr>().is_ok()
}
