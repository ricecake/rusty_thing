#![feature(lookup_host)]
#![feature(ip_addr)]
#![feature(scoped)]
use std::thread;
use std::collections::BTreeSet;

fn main() {
	println!("Hello, world!");
	let mut host_vec = BTreeSet::new();
	for host in std::net::lookup_host("tfm.nu").unwrap() {
		host_vec.insert(host.unwrap().ip());
	}
	for it in host_vec {
		println!("found address: {}", it);
	}
}
