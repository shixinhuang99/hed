use std::{
	collections::{HashMap, HashSet},
	fs,
	net::IpAddr,
	path::PathBuf,
};

use anyhow::Result;

use super::ip_hosts::IpHosts;

const HED_COMMENT_MARK: &str = "#(hed)";

#[derive(Default, Debug, Clone)]
pub struct HostsInfo {
	pub content: String,
	pub list: Vec<IpHosts>,
	pub lines: Vec<LineKind>,
}

#[derive(Debug, Clone)]
pub enum LineKind {
	Valid(ValidLine),
	Comment(String),
	Empty,
	Other(String),
}

#[derive(Debug, Clone)]
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
		let list = lines_to_list(&lines);

		Ok(Self {
			content,
			list,
			lines,
		})
	}

	pub fn pretty(&mut self) {
		self.lines = new_lines_by_list(&self.lines, &self.list);
		self.content =
			lines_to_content(&self.lines, cfg!(target_os = "windows"));
	}

	pub fn update_by_content_change(&mut self) {
		self.lines = content_to_lines(&self.content);
		self.list = lines_to_list(&self.lines);
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
			LineKind::Empty => String::new(),
		};

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

fn lines_to_list(lines: &[LineKind]) -> Vec<IpHosts> {
	let mut list: Vec<IpHosts> = vec![];

	let mut ip_index_map: HashMap<&str, usize> = HashMap::new();

	for line in lines {
		if let LineKind::Valid(valid_line) = line {
			if let Some(idx) = ip_index_map.get(valid_line.ip.as_str()) {
				if let Some(ip_hosts) = list.get_mut(*idx) {
					ip_hosts.add(valid_line.hosts.clone(), valid_line.enabled);
				}
			} else {
				list.push(IpHosts::new(
					&valid_line.ip,
					valid_line.hosts.clone(),
					valid_line.enabled,
				));
				ip_index_map.insert(&valid_line.ip, list.len() - 1);
			}
		}
	}

	list
}

fn new_lines_by_list(lines: &[LineKind], list: &[IpHosts]) -> Vec<LineKind> {
	let mut ip_enabled_set: HashSet<String> = HashSet::new();
	let mut should_be_removed: HashSet<usize> = HashSet::new();
	let mut is_previous_line_empty = if !lines.is_empty() {
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

	for (i, line) in lines.iter().enumerate() {
		if !should_be_removed.contains(&i) {
			new_lines.push(line.clone());
		}
	}

	let mut ip_index_map: HashMap<String, usize> = HashMap::new();

	for (i, line) in new_lines.iter().enumerate() {
		if let LineKind::Valid(valid_line) = line {
			ip_index_map.insert(valid_line.ip.clone(), i);
		}
	}

	for ip_hosts in list {
		let enabled_hosts: Vec<String> = ip_hosts
			.hosts
			.iter()
			.filter_map(|h| {
				if h.enabled {
					Some(h.name.clone())
				} else {
					None
				}
			})
			.collect();

		let disabled_hosts: Vec<String> = ip_hosts
			.hosts
			.iter()
			.filter_map(|h| {
				if !h.enabled {
					Some(h.name.clone())
				} else {
					None
				}
			})
			.collect();

		if let Some(idx) = ip_index_map.get(&ip_hosts.ip) {
			if let Some(LineKind::Valid(valid_line)) = new_lines.get_mut(*idx) {
				valid_line.hosts = if valid_line.enabled {
					enabled_hosts
				} else {
					disabled_hosts
				}
			}
		} else {
			if !enabled_hosts.is_empty() {
				new_lines.push(LineKind::Valid(ValidLine {
					ip: ip_hosts.ip.clone(),
					hosts: enabled_hosts,
					comment: None,
					enabled: true,
				}));
			}
			if !disabled_hosts.is_empty() {
				new_lines.push(LineKind::Valid(ValidLine {
					ip: ip_hosts.ip.clone(),
					hosts: disabled_hosts,
					comment: None,
					enabled: false,
				}));
			}
		}
	}

	if new_lines
		.last()
		.is_some_and(|line| !matches!(line, LineKind::Empty))
	{
		new_lines.push(LineKind::Empty);
	}

	new_lines
}

#[cfg(test)]
mod tests {
	use std::{env, fs};

	use insta::{assert_debug_snapshot, assert_snapshot};
	use rstest::{fixture, rstest};

	use super::{
		content_to_lines, lines_to_content, lines_to_list, new_lines_by_list,
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
	fn test_lines_to_content_win(content: String) {
		let lines = content_to_lines(&content);
		let new_content_win = lines_to_content(&lines, true);

		assert_debug_snapshot!("lines_to_content_win", new_content_win);
	}

	#[rstest]
	fn test_lines_to_content_mac(content: String) {
		let lines = content_to_lines(&content);
		let new_content_mac = lines_to_content(&lines, false);

		assert_debug_snapshot!("lines_to_content_mac", new_content_mac);
	}

	#[rstest]
	fn test_lines_to_content_human_read(content: String) {
		let lines = content_to_lines(&content);
		let new_content = lines_to_content(&lines, false);

		assert_snapshot!("lines_to_content_human_read", new_content);
	}

	#[rstest]
	fn test_lines_to_list(content: String) {
		let lines = content_to_lines(&content);
		let list = lines_to_list(&lines);

		assert_debug_snapshot!("lines_to_list", list);
	}

	#[rstest]
	fn test_new_lines_by_list(content: String) {
		let mut lines = content_to_lines(&content);
		let mut list = lines_to_list(&lines);

		for (i, hosts) in list.iter_mut().enumerate() {
			hosts.add(vec![format!("foo{}.com", i)], false);
		}

		lines = new_lines_by_list(&lines, &list);

		assert_debug_snapshot!("new_lines_by_list", lines);
	}
}
