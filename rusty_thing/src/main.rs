#![feature(lookup_host)]
#![feature(ip_addr)]
#![feature(scoped)]
use std::thread;
use std::collections::BTreeSet;
use std::collections::BTreeMap;

fn main() {
	println!("Hello, world!");
	let mut host_vec = BTreeSet::new();
	for host in std::net::lookup_host("tfm.nu").unwrap() {
		host_vec.insert(host.unwrap().ip());
	}
	for it in host_vec {
		println!("found address: {}", it);
	}

	let mut test_map = BTreeMap::new();
	//for i in 1..25 {
		test_map.insert(3, 3);
		test_map.insert(5, 5);
		test_map.insert(8, 8);
		test_map.insert(1, 1);
	//}
	println!("the number is: {}", test_map.values().next().unwrap());
}

structure Thing {
    name: String
}

impl Thing {
    pub fn new() -> Thing {
        Thing { name: "ear ball" };
    }

    pub fn frobnicate(&mut self) -> Thing {
    	self.name = "FROBNICATED";
    	return self;
    }
}
