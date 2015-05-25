#![feature(lookup_host)]
#![feature(ip_addr)]
#![feature(scoped)]
use std::thread;

fn main() {
	println!("Hello, world!");
	let mut host_vec = Vec::new();
	for host in std::net::lookup_host("tfm.nu").unwrap() {
		let ip_addy = host.unwrap().ip();
		if !host_vec.contains(&ip_addy)  {
			host_vec.push(ip_addy);
			println!("found address: {}", ip_addy);
		}
	}
}
