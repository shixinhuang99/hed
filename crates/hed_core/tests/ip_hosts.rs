use hed_core::IpHosts;
use insta::assert_debug_snapshot;

#[test]
fn test_ip_hosts_parse() {
	let s = "172.16.254.1 a.com b.com b.com # comment";
	assert_debug_snapshot!("ipv4", s.parse::<IpHosts>().unwrap());
}

#[test]
fn test_ip_hosts_parse_ipv6() {
	let s =
		"2402:1200:4f00:1234:0000:5678:9abc:def0 a.com b.com b.com # comment";
	assert_debug_snapshot!("ipv6", s.parse::<IpHosts>().unwrap());
}
