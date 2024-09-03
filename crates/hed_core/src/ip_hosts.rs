use std::{net::IpAddr, result::Result, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub struct IpHosts {
	pub ip: IpAddr,
	pub hosts: Vec<String>,
}

impl FromStr for IpHosts {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let s = strip_comment(s);
		if let Some((may_ip, rest)) =
			s.split_once(|ch: char| ch.is_whitespace())
		{
			if let Ok(ip) = may_ip.parse::<IpAddr>() {
				return Ok(IpHosts {
					ip,
					hosts: rest
						.split_whitespace()
						.map(|s| s.to_string())
						.collect(),
				});
			}
		}
		if let Ok(ip) = s.parse::<IpAddr>() {
			return Ok(IpHosts { ip, hosts: vec![] });
		}
		Err(())
	}
}

fn strip_comment(s: &str) -> &str {
	let s2 = if let Some(idx) = s.chars().position(|ch| ch == '#') {
		&s[0..idx]
	} else {
		s
	};
	s2.trim()
}

#[cfg(test)]
mod tests {
	use std::net::{IpAddr, Ipv4Addr};

	use test_case::test_case;

	use super::{strip_comment, IpHosts};

	#[test_case("foo#bar"; "case 1")]
	#[test_case("foo #bar"; "case 2")]
	#[test_case("foo #     bar"; "case 3")]
	#[test_case("foo     #     bar"; "case 4")]
	fn test_strip_comment(s: &str) {
		assert_eq!(strip_comment(s), "foo");
	}

	#[test_case("172.16.254.1 a.com b.com # comment"; "case 1")]
	#[test_case("172.16.254.1    a.com    b.com  #   comment  "; "case 2")]
	fn test_ip_hosts_parse(s: &str) {
		let expect = IpHosts {
			ip: IpAddr::V4(Ipv4Addr::new(172, 16, 254, 1)),
			hosts: vec!["a.com".to_string(), "b.com".to_string()],
		};
		assert_eq!(s.parse::<IpHosts>().unwrap(), expect);
	}
}
