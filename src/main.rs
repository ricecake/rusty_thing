#![feature(lookup_host)]
#![feature(ip_addr)]
#![feature(scoped)]
use std::thread;

fn main() {
	println!("Hello, world!");
	for host in std::net::lookup_host("tfm.nu").unwrap() {
		let _ = thread::scoped(move || {
			println!("found address: {}", host.unwrap().ip());
		});
	}
}
