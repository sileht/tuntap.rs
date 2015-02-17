#![allow(dead_code)]
#![feature(core)]
#![feature(io)]
#![feature(libc)]
#![feature(std_misc)]
#![feature(macro_rules)]
#[macro_use] extern crate bitflags;
#[macro_use] extern crate log;
extern crate libc;

pub use tuntap::{TunTap,IFF_TUN,IFF_TAP,IFF_NO_PI};
use std::thread::Thread;

pub mod tuntap;

#[test]
fn it_works() {
	let flags = IFF_TUN | IFF_NO_PI;
	let (tun1, (tx1,rx1)) = TunTap::new("tap12", flags).unwrap();
	let (tun2, (tx2,rx2)) = TunTap::new("tap14", flags).unwrap();

	for (i, tun) in [tun1, tun2].iter().enumerate() {
		tun.set_owner(1001).unwrap();
		tun.set_group(1001).unwrap();
		//tun.set_mac([11,22,33,44,55,66+i as u8]).unwrap();
		if i == 0 {
			tun.set_ipv4("10.0.0.1").unwrap();
			tun.set_ipv6("fe80::0db8:1234:1211").unwrap();
		} else {
			tun.set_ipv4("10.0.0.2").unwrap();
			tun.set_ipv6("fe80::0db8:1234:1212").unwrap();
		}
		tun.set_mtu(1400).unwrap();
		tun.set_up().unwrap();
	}

	Thread::spawn(move || {
		for packet in rx1.iter() {
			tx2.send(packet).unwrap();
		}
	});
	for packet in rx2.iter() {
		tx1.send(packet).unwrap();
	}
}

