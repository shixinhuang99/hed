use std::{net::IpAddr, result::Result, str::FromStr};

use indexmap::IndexSet;

#[derive(Debug, PartialEq, Eq)]
pub struct IpHosts {
	pub ip: String,
	pub hosts: IndexSet<String>,
}

impl FromStr for IpHosts {
	type Err = ();

	fn from_str(line: &str) -> Result<Self, Self::Err> {
		let line = strip_comment(line);
		if let Some((may_ip, rest)) =
			line.split_once(|ch: char| ch.is_whitespace())
		{
			if may_ip.parse::<IpAddr>().is_ok() {
				return Ok(IpHosts {
					ip: may_ip.to_string(),
					hosts: rest
						.split_whitespace()
						.map(|s| s.to_string())
						.collect(),
				});
			}
		}
		if line.parse::<IpAddr>().is_ok() {
			return Ok(IpHosts {
				ip: line.to_string(),
				hosts: IndexSet::new(),
			});
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
	use test_case::test_case;

	use super::strip_comment;

	#[test_case("foo#bar", "foo"; "case 1")]
	#[test_case("foo #bar", "foo"; "case 2")]
	#[test_case("foo #     bar", "foo"; "case 3")]
	#[test_case("foo     #     bar", "foo"; "case 4")]
	#[test_case("foo foo   #     bar", "foo foo"; "case 5")]
	fn test_strip_comment(s: &str, expect: &str) {
		assert_eq!(strip_comment(s), expect);
	}
}
