#![feature(lookup_host)]
fn main() {
    println!("Hello, world!");
    for host in std::net::lookup_host("tfm.nu").unwrap() {
        println!("found address: {}", host.unwrap());
    }
}
