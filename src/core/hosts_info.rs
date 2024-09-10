use std::{fs, net::IpAddr, path::PathBuf, str::FromStr};

use anyhow::Result;
use indexmap::{IndexMap, IndexSet};

#[derive(Default, Debug)]
pub struct HostsInfo {
	pub content: String,
	pub map: IndexMap<String, IpHosts>,
	pub ir: Vec<LineIR>,
}

impl HostsInfo {
	pub fn parse_from_file(hosts_path: PathBuf) -> Result<Self> {
		let content = fs::read_to_string(hosts_path)?;

		let mut parse_result = HostsInfo::default();

		for raw_line in content.lines() {
			let line = raw_line.trim();
			if line.starts_with('#') {
				continue;
			}
			if let Ok(new_ih) = line.parse::<IpHosts>() {
				if let Some(ih) = parse_result.map.get_mut(&new_ih.ip) {
					ih.hosts.extend(new_ih.hosts);
				} else {
					parse_result.map.insert(new_ih.ip.clone(), new_ih);
				}
			}
		}

		parse_result.content = content;

		Ok(parse_result)
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct IpHosts {
	pub ip: String,
	pub hosts: IndexSet<String>,
}

impl FromStr for IpHosts {
	type Err = ();

	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		let s = s.trim();
		if let Some((may_ip, rest)) =
			s.split_once(|ch: char| ch.is_whitespace())
		{
			if is_ip(may_ip) {
				return Ok(IpHosts {
					ip: may_ip.to_string(),
					hosts: rest
						.split_whitespace()
						.map(|s| s.to_string())
						.collect(),
				});
			}
		}
		if is_ip(s) {
			return Ok(IpHosts {
				ip: s.to_string(),
				hosts: IndexSet::new(),
			});
		}
		Err(())
	}
}

fn is_ip(s: &str) -> bool {
	s.parse::<IpAddr>().is_ok()
}

fn strip_comment(s: &str) -> &str {
	let s2 = if let Some(idx) = s.chars().position(|ch| ch == '#') {
		&s[0..idx]
	} else {
		s
	};
	s2.trim()
}

#[derive(Debug)]
pub struct LineIR {
	index: usize,
	content: String,
	kind: LineIRKind,
}

#[derive(Debug)]
pub enum LineIRKind {
	Normal,
	Comment,
	HedComment,
	Empty,
	Other,
}

fn content_to_ir(s: &str) -> Vec<LineIR> {
	let mut ir = vec![];

	for (i, line) in s.lines().enumerate() {
		let mut line_ir = LineIR {
			index: i,
			content: line.to_string(),
			kind: LineIRKind::Other,
		};

		let trimed_line = line.trim();

		if trimed_line.is_empty() {
			line_ir.kind = LineIRKind::Empty;
			ir.push(line_ir);
			continue;
		}

		if trimed_line.starts_with("#(hed)") {
			line_ir.kind = LineIRKind::HedComment;
			ir.push(line_ir);
			continue;
		}

		if trimed_line.starts_with('#') {
			line_ir.kind = LineIRKind::Comment;
			ir.push(line_ir);
			continue;
		}
	}

	ir
}

#[cfg(test)]
mod tests {
	use std::env;

	use insta::assert_debug_snapshot;
	use test_case::test_case;

	use super::{strip_comment, HostsInfo};

	#[test_case("foo#bar", "foo"; "case 1")]
	#[test_case("foo #bar", "foo"; "case 2")]
	#[test_case("foo #     bar", "foo"; "case 3")]
	#[test_case("foo     #     bar", "foo"; "case 4")]
	#[test_case("foo foo   #     bar", "foo foo"; "case 5")]
	fn test_strip_comment(s: &str, expect: &str) {
		assert_eq!(strip_comment(s), expect);
	}

	#[test]
	fn test_host_info_map() {
		let hosts_path = env::current_dir().unwrap().join("fixture/hosts");
		let hosts_info = HostsInfo::parse_from_file(hosts_path).unwrap();

		assert_debug_snapshot!("map", hosts_info.map);
	}
}
