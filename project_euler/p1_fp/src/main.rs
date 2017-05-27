extern crate time;

use time::PreciseTime;
use std::env;
use std::process;

fn iter_sum() -> u32 {
    let mut n = 0;
	for i in 1..1001 {
		match i {
			i if i % 3 == 0 => n += i,
			i if i % 5 == 0 => n += i,
			_ => ()
		}
	}
	return n;
}

fn fp_sum() -> u32 {
    (1..1001).fold(0, |sum, x| sum + match x {
		x if x % 3 == 0 => x,
		x if x % 5 == 0 => x,
		_ => 0
	})
}

fn perf_test() {
	let start = PreciseTime::now();
	iter_sum();
	let end = PreciseTime::now();
	println!("Iter took {} seconds", start.to(end));
	
	let start = PreciseTime::now();
	fp_sum();
	let end = PreciseTime::now();
	println!("FP took {} seconds", start.to(end));

}

fn main() {
	// Get args
	let args: Vec<_> = env::args().collect();
	
	if args.len() == 2 {
		// let mode = String::from(args[1]);
		let mode = &args[1].to_string();
		match mode.as_ref() {
			"run" => println!("{}", fp_sum()),
			"perf" => perf_test(),
			_ => println!("Illegal arguments.")
		};
	}
	else {
		println!("Illegal arguments: please supply one argument.");
		process::exit(0);
	}
}
