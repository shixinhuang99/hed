use std::env;

use hed_core::Hed;
use insta::assert_debug_snapshot;

#[tokio::test]
async fn test_hed_parse() {
	let cwd = env::current_dir().unwrap();
	let hosts_path = cwd.join("fixture/hosts");
	let mut hed = Hed::new(hosts_path);

	hed.parse().await.unwrap();

	assert_debug_snapshot!("ip_hosts_map", hed.ip_hosts_map);
}
