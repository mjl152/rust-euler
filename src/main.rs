extern crate rustc_serialize;
extern crate docopt;
extern crate num;

use std::collections::HashSet;
use docopt::Docopt;

const USAGE: &'static str = "
rust-euler

Rust implementations of the Project Euler problems.

Usage: rust-euler [options]

Options:
	-p, --problem=K    Which Project Euler problem to solve [default: 1]
	-h, --help         Display this help and exit
";

#[derive(Debug, RustcDecodable)]
struct Args {
	flag_problem: u8,
	flag_help: bool,
}

fn main() {
	let args: Args = Docopt::new(USAGE).and_then(|d| d.decode())
									   .unwrap_or_else(|e| e.exit());
	match args.flag_problem {
		1 => problem_1(),
		2 => problem_2(),
		3 => problem_3(),
		_ => println!("Problem number solution not supported.")
	}
}

fn problem_1() {
	let mut multiples = HashSet::new();
	for i in num::range_step(3, 1000, 3) {
		multiples.insert(i);
	}
	for i in num::range_step(5, 1000, 5) {
		multiples.insert(i);
	}
	let sum = multiples.iter().fold(0, |sum, x| sum + x);
	println!("{}", sum);	
}

fn problem_2() {
	let mut current = 2;
	let mut previous = 1;
	let mut n = 0;
	let mut sum = 2;
	while current < 4000000 {
		let temp = current;
		current = temp + previous;
		previous = temp;
		n = n + 1;
		if n == 3 {
			sum += current;
			n = 0;
		}
	}
	println!("{}", sum)
}

fn is_prime(x: u64) -> bool {
	let mut i = 2;
	while i < (x / 2) {
		if x % i == 0 {
			return false;
		}
		i += 1;
	}
	return true;
}

fn problem_3() {
	 let a : u64 = 600851475143;
	 let mut d = 2;
	 while d < (a / 2) {
	 	// Check that the result is an integer
	 	if a % d == 0 {
	 		match is_prime(a / d) {
	 			true => {println!("{}", a/d); return;},
	 			false => {}
	 		}
	 	}
	 	d += 1;
	 }
}