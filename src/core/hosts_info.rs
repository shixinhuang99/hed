use std::{
	collections::{HashMap, HashSet},
	fs,
	path::PathBuf,
};

use anyhow::Result;
use indexmap::IndexMap;

use super::{item::Item, item_form::ItemForm};
use crate::util::{is_ip, StringExt};

const HED_COMMENT_MARK: &str = "#(hed)";

#[derive(Default, Debug, Clone)]
pub struct HostsInfo {
	pub content: String,
	pub list: Vec<Item>,
	lines: Vec<Line>,
}

#[derive(Debug, Clone)]
pub enum Line {
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

	pub fn update_content(&mut self) {
		self.lines = new_lines_by_list(&self.lines, &self.list);
		self.content =
			lines_to_content(&self.lines, cfg!(target_os = "windows"));
	}

	pub fn update_list(&mut self) {
		self.lines = content_to_lines(&self.content);
		self.list = lines_to_list(&self.lines);
	}

	pub fn add_item(&mut self, form: &ItemForm) {
		if let Some(item) = self.list.iter_mut().find(|item| item.ip == form.ip)
		{
			item.add_hosts(form.hosts.to_split_whitespace_vec(), true);
		} else {
			self.list.push(Item::new(
				&form.ip,
				form.hosts.to_split_whitespace_vec(),
				true,
			));
		}
	}

	pub fn get_item_mut(&mut self, item_id: usize) -> Option<&mut Item> {
		self.list.iter_mut().find(|item| item.id == item_id)
	}

	pub fn remove_item(&mut self, item_id: usize) {
		if let Some(idx) = self.list.iter().position(|item| item.id == item_id)
		{
			self.list.remove(idx);
		}
	}

	pub fn save_to_file(&self, hosts_path: PathBuf) -> Result<()> {
		let metadata = fs::metadata(&hosts_path)?;
		let mut permissions = metadata.permissions();
		if permissions.readonly() {
			#[allow(clippy::permissions_set_readonly_false)]
			permissions.set_readonly(false);
			fs::set_permissions(&hosts_path, permissions)?;
		}

		let tmp_file = std::env::temp_dir().join("hed_tmp");
		fs::write(&tmp_file, &self.content)?;
		fs::copy(&tmp_file, hosts_path)?;
		fs::remove_file(&tmp_file)?;

		Ok(())
	}
}

fn content_to_lines(s: &str) -> Vec<Line> {
	let mut lines = vec![];

	for l in s.lines() {
		let line = l.trim().to_string();

		if line.is_empty() {
			lines.push(Line::Empty);
			continue;
		}

		if line.starts_with('#') && !line.starts_with(HED_COMMENT_MARK) {
			lines.push(Line::Comment(line));
			continue;
		}

		if let Some(valid_line) = parse_valid_line(&line) {
			lines.push(Line::Valid(valid_line));
		} else {
			lines.push(Line::Other(line));
		}
	}

	lines
}

fn lines_to_content(lines: &[Line], is_win: bool) -> String {
	let mut text_lines: Vec<String> = vec![];

	for line in lines {
		let text = match line {
			Line::Valid(valid_line) => {
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
			Line::Comment(s) => s.clone(),
			Line::Other(s) => s.clone(),
			Line::Empty => String::new(),
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

fn lines_to_list(lines: &[Line]) -> Vec<Item> {
	let mut item_map: IndexMap<String, Item> = IndexMap::new();

	for line in lines {
		if let Line::Valid(valid_line) = line {
			if let Some(item) = item_map.get_mut(&valid_line.ip) {
				item.add_hosts(valid_line.hosts.clone(), valid_line.enabled);
			} else {
				item_map.insert(
					valid_line.ip.clone(),
					Item::new(
						&valid_line.ip,
						valid_line.hosts.clone(),
						valid_line.enabled,
					),
				);
			}
		}
	}

	item_map.into_values().collect()
}

fn gen_key(ip: &str, enabled: bool) -> String {
	format!("{}{}", ip, enabled)
}

fn remove_lines_by_indices(
	lines: Vec<Line>,
	indices: &mut HashSet<usize>,
) -> Vec<Line> {
	let mut new = vec![];
	for (i, line) in lines.into_iter().enumerate() {
		if !indices.contains(&i) {
			new.push(line);
		}
	}
	indices.clear();
	new
}

fn new_lines_by_list(lines: &[Line], list: &[Item]) -> Vec<Line> {
	let mut lines = lines.to_vec();

	let mut indices_to_removed: HashSet<usize> = HashSet::new();

	let mut ip_enabled_set: HashSet<String> = HashSet::new();

	for (i, line) in lines.iter().enumerate() {
		if let Line::Valid(valid_line) = line {
			let key = gen_key(&valid_line.ip, valid_line.enabled);
			if ip_enabled_set.contains(&key) {
				indices_to_removed.insert(i);
			} else {
				ip_enabled_set.insert(key);
			}
		}
	}

	lines = remove_lines_by_indices(lines, &mut indices_to_removed);

	let mut line_idx_map: HashMap<String, usize> = HashMap::new();

	for (i, line) in lines.iter().enumerate() {
		if let Line::Valid(valid_line) = line {
			line_idx_map.insert(gen_key(&valid_line.ip, valid_line.enabled), i);
		}
	}

	let mut list_ip_set: HashSet<&str> = HashSet::new();

	for item in list {
		let mut enabled_hosts = vec![];
		let mut disabled_hosts = vec![];

		for host in &item.hosts {
			if host.enabled {
				enabled_hosts.push(host.name.clone());
			} else {
				disabled_hosts.push(host.name.clone());
			}
		}

		if let Some(idx) = line_idx_map.get(&gen_key(&item.ip, true)) {
			if let Some(Line::Valid(valid_line)) = lines.get_mut(*idx) {
				valid_line.hosts = enabled_hosts;
			}
		} else {
			lines.push(Line::Valid(ValidLine {
				ip: item.ip.clone(),
				hosts: enabled_hosts,
				comment: None,
				enabled: true,
			}));
		}

		if let Some(idx) = line_idx_map.get(&gen_key(&item.ip, false)) {
			if let Some(Line::Valid(valid_line)) = lines.get_mut(*idx) {
				valid_line.hosts = disabled_hosts;
			}
		} else {
			lines.push(Line::Valid(ValidLine {
				ip: item.ip.clone(),
				hosts: disabled_hosts,
				comment: None,
				enabled: false,
			}));
		}

		list_ip_set.insert(&item.ip);
	}

	for (i, line) in lines.iter().enumerate() {
		if let Line::Valid(valid_line) = line {
			if valid_line.hosts.is_empty()
				|| !list_ip_set.contains(valid_line.ip.as_str())
			{
				indices_to_removed.insert(i);
			}
		}
	}

	lines = remove_lines_by_indices(lines, &mut indices_to_removed);

	let mut is_previous_line_empty = if lines.is_empty() {
		false
	} else {
		matches!(lines[0], Line::Empty)
	};

	for (i, line) in lines.iter().enumerate() {
		let is_line_empty = matches!(line, Line::Empty);
		if is_line_empty && is_previous_line_empty {
			indices_to_removed.insert(i);
		}
		is_previous_line_empty = is_line_empty;
	}

	lines = remove_lines_by_indices(lines, &mut indices_to_removed);

	if lines
		.last()
		.is_some_and(|line| !matches!(line, Line::Empty))
	{
		lines.push(Line::Empty);
	}

	lines
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
			hosts.add_hosts(vec![format!("foo{}.com", i)], false);
		}

		lines = new_lines_by_list(&lines, &list);

		assert_debug_snapshot!("new_lines_by_list", lines);
	}
}
