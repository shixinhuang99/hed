use std::{fs, net::IpAddr, path::PathBuf};

use anyhow::Result;
use indexmap::{IndexMap, IndexSet};

type IpHostsMap = IndexMap<String, Hosts>;

const HED_COMMENT_MARK: &str = "#(hed)";

#[derive(Default, Debug)]
pub struct HostsInfo {
	pub content: String,
	pub map: IpHostsMap,
	pub lines: Vec<LineKind>,
}

#[derive(Debug)]
pub struct Hosts {
	pub enabled: IndexSet<String>,
	pub disabled: IndexSet<String>,
}

#[derive(Debug)]
pub enum LineKind {
	Valid(ValidLine),
	Comment(String),
	Empty,
	Other(String),
}

#[derive(Debug)]
pub struct ValidLine {
	ip: String,
	hosts: Vec<String>,
	comment: Option<String>,
	enabled: bool,
}

impl HostsInfo {
	pub fn parse_from_file(hosts_path: PathBuf) -> Result<Self> {
		let content = fs::read_to_string(hosts_path)?;
		let lines = content_to_lines(&content);
		let map = lines_to_map(&lines);

		Ok(Self {
			content,
			map,
			lines,
		})
	}
}

fn content_to_lines(s: &str) -> Vec<LineKind> {
	let mut lines = vec![];

	for l in s.lines() {
		let line = l.trim().to_string();

		if line.is_empty() {
			lines.push(LineKind::Empty);
			continue;
		}

		if line.starts_with('#') && !line.starts_with(HED_COMMENT_MARK) {
			lines.push(LineKind::Comment(line));
			continue;
		}

		if let Some(valid_line) = parse_valid_line(&line) {
			lines.push(LineKind::Valid(valid_line));
		} else {
			lines.push(LineKind::Other(line));
		}
	}

	lines
}

fn lines_to_content(lines: &[LineKind], is_win: bool) -> String {
	let mut text_lines: Vec<String> = vec![];

	let mut is_previous_line_empty = if lines.len() > 1 {
		matches!(lines[0], LineKind::Empty)
	} else {
		false
	};

	for line in lines {
		let text = match line {
			LineKind::Valid(valid_line) => {
				let mut vs = vec![];
				if !valid_line.enabled {
					vs.push(HED_COMMENT_MARK.to_string());
				}
				vs.push(valid_line.ip.clone());
				vs.append(&mut valid_line.hosts.clone());
				if let Some(comment) = &valid_line.comment {
					vs.push(format!("# {}", comment));
				}
				vs.join(" ")
			}
			LineKind::Comment(s) => s.clone(),
			LineKind::Other(s) => s.clone(),
			LineKind::Empty => {
				if is_previous_line_empty {
					continue;
				}
				String::new()
			}
		};

		is_previous_line_empty = matches!(line, LineKind::Empty);

		text_lines.push(text);
	}

	let eol = if is_win {
		"\r\n"
	} else {
		"\n"
	};

	text_lines.join(eol)
}

fn parse_valid_line(s: &str) -> Option<ValidLine> {
	let (striped_s, enabled) = strip_hed_comment(s);
	let (ip_hosts, comment) = split_ip_hosts_comment(striped_s);
	if let Some((ip, hosts)) = split_ip_hosts(ip_hosts) {
		Some(ValidLine {
			ip,
			hosts,
			comment: comment.map(|c| c.trim().to_string()),
			enabled,
		})
	} else {
		None
	}
}

fn strip_hed_comment(s: &str) -> (&str, bool) {
	if let Some(striped) = s.strip_prefix(HED_COMMENT_MARK) {
		(striped, false)
	} else {
		(s, true)
	}
}

fn split_ip_hosts_comment(s: &str) -> (&str, Option<&str>) {
	if let Some((may_ip_hosts, comment)) = s.split_once('#') {
		(may_ip_hosts, Some(comment))
	} else {
		(s, None)
	}
}

fn split_ip_hosts(s: &str) -> Option<(String, Vec<String>)> {
	let chunks = s.split_whitespace().collect::<Vec<&str>>();

	if chunks.len() > 1 {
		let may_ip = chunks[0];
		if is_ip(may_ip) {
			return Some((
				may_ip.to_string(),
				chunks[1..].iter().map(|i| i.to_string()).collect(),
			));
		}
	}

	None
}

fn is_ip(s: &str) -> bool {
	s.parse::<IpAddr>().is_ok()
}

impl Hosts {
	pub fn new(hosts: Vec<String>, enabled: bool) -> Self {
		if enabled {
			Self {
				enabled: IndexSet::from_iter(hosts),
				disabled: IndexSet::new(),
			}
		} else {
			Self {
				enabled: IndexSet::new(),
				disabled: IndexSet::from_iter(hosts),
			}
		}
	}

	pub fn add(&mut self, hosts: Vec<String>, enabled: bool) {
		if enabled {
			for host in hosts {
				self.enabled.insert(host);
			}
		} else {
			for host in hosts {
				self.disabled.insert(host);
			}
		}
	}
}

fn lines_to_map(lines: &[LineKind]) -> IpHostsMap {
	let mut map: IpHostsMap = IndexMap::new();

	for line in lines {
		if let LineKind::Valid(valid_line) = line {
			if let Some(hosts) = map.get_mut(&valid_line.ip) {
				hosts.add(valid_line.hosts.clone(), valid_line.enabled);
			} else {
				map.insert(
					valid_line.ip.clone(),
					Hosts::new(valid_line.hosts.clone(), valid_line.enabled),
				);
			}
		}
	}

	map
}

fn new_lines_by_map(
	lines: Vec<LineKind>,
	ip_hosts_map: &IpHostsMap,
) -> Vec<LineKind> {
	use std::collections::{HashMap, HashSet};

	let mut ip_enabled_set: HashSet<String> = HashSet::new();
	let mut should_be_removed: HashSet<usize> = HashSet::new();
	let mut is_previous_line_empty = if lines.len() > 1 {
		matches!(lines[0], LineKind::Empty)
	} else {
		false
	};

	for (i, line) in lines.iter().enumerate() {
		match line {
			LineKind::Valid(valid_line) => {
				let key = format!("{}{}", valid_line.ip, valid_line.enabled);
				if ip_enabled_set.contains(&key) {
					should_be_removed.insert(i);
				} else {
					ip_enabled_set.insert(key);
				}
			}
			LineKind::Empty => {
				if is_previous_line_empty {
					should_be_removed.insert(i);
				}
			}
			_ => (),
		}

		if !should_be_removed.contains(&i) {
			is_previous_line_empty = matches!(line, LineKind::Empty);
		}
	}

	let mut new_lines: Vec<LineKind> = vec![];

	for (i, line) in lines.into_iter().enumerate() {
		if !should_be_removed.contains(&i) {
			new_lines.push(line);
		}
	}

	let mut ip_index_map: HashMap<String, usize> = HashMap::new();

	for (i, line) in new_lines.iter().enumerate() {
		if let LineKind::Valid(valid_line) = line {
			ip_index_map.insert(valid_line.ip.clone(), i);
		}
	}

	for (ip, hosts) in ip_hosts_map {
		if let Some(line_idx) = ip_index_map.get(ip) {
			if let Some(LineKind::Valid(valid_line)) =
				new_lines.get_mut(*line_idx)
			{
				valid_line.hosts = if valid_line.enabled {
					Vec::from_iter(hosts.enabled.clone())
				} else {
					Vec::from_iter(hosts.disabled.clone())
				}
			}
		} else {
			if !hosts.enabled.is_empty() {
				new_lines.push(LineKind::Valid(ValidLine {
					ip: ip.clone(),
					hosts: Vec::from_iter(hosts.enabled.clone()),
					comment: None,
					enabled: true,
				}));
			}
			if !hosts.disabled.is_empty() {
				new_lines.push(LineKind::Valid(ValidLine {
					ip: ip.clone(),
					hosts: Vec::from_iter(hosts.disabled.clone()),
					comment: None,
					enabled: false,
				}));
			}
		}
	}

	new_lines
}

#[cfg(test)]
mod tests {
	use std::{env, fs};

	use insta::{assert_debug_snapshot, assert_snapshot};
	use rstest::{fixture, rstest};

	use super::{
		content_to_lines, lines_to_content, lines_to_map, new_lines_by_map,
	};

	#[fixture]
	fn content() -> String {
		let hosts_path = env::current_dir().unwrap().join("fixture/hosts");
		fs::read_to_string(hosts_path).unwrap()
	}

	#[rstest]
	fn test_content_to_lines(content: String) {
		let lines = content_to_lines(&content);

		assert_debug_snapshot!("content_to_lines", lines);
	}

	#[rstest]
	fn test_lines_to_map(content: String) {
		let lines = content_to_lines(&content);
		let map = lines_to_map(&lines);

		assert_debug_snapshot!("lines_to_map", map);
	}

	#[rstest]
	fn test_lines_to_content(content: String) {
		let lines = content_to_lines(&content);
		let new_content_win = lines_to_content(&lines, true);
		let new_content_mac = lines_to_content(&lines, false);

		assert_debug_snapshot!("lines_to_content_win", new_content_win);
		assert_debug_snapshot!("lines_to_content_mac", new_content_mac);
		assert_snapshot!("lines_to_content_human_read", new_content_mac);
	}

	#[rstest]
	fn test_new_lines_by_map(content: String) {
		let mut lines = content_to_lines(&content);
		let mut map = lines_to_map(&lines);

		for (i, hosts) in map.values_mut().enumerate() {
			hosts.add(vec![format!("foo{}.com", i)], false);
		}

		lines = new_lines_by_map(lines, &map);

		assert_debug_snapshot!("new_lines_by_map", lines);
	}
}
