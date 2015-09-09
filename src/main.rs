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
	-b, --bench        Benchmark the specified problem
";

const DEFAULT_RETURN: &'static str = "The problem solution algorithm completed. The reported answer is: ";
const ERROR_PROBLEM_NOT_IMPL: &'static str = "Problem number solution not supported";

#[derive(Debug, RustcDecodable)]
struct Args {
	flag_problem: u8,
	flag_help: bool,
	flag_bench: bool,
}

fn main() {
	let args: Args = Docopt::new(USAGE).and_then(|d| d.decode())
									   .unwrap_or_else(|e| e.exit());
	let mut problem : fn() -> (u64, &'static str);
	match args.flag_problem {
		1 => problem = problem_1,
		2 => problem = problem_2,
		3 => problem = problem_3,
		_ => problem = problem_not_supported,
	}
	let (result, message) = problem();
	println!("{}", message);
	println!("{}", result);
}

fn problem_1() -> (u64, &'static str) {
	let mut multiples = HashSet::new();
	for i in num::range_step(3, 1000, 3) {
		multiples.insert(i);
	}
	for i in num::range_step(5, 1000, 5) {
		multiples.insert(i);
	}
	let sum = multiples.iter().fold(0, |sum, x| sum + x);
	return (sum, DEFAULT_RETURN);	
}

fn problem_2() -> (u64, &'static str) {
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
	return (sum, DEFAULT_RETURN);
}

/* A number is prime if it is divisble by no numbers other than 2 or itself.
   Our naive prime checking function performs this check.
   However we know that if a number x is not divisble by a number i
   it is not divisible by any multiples of i.
   So really the check to see if a number is prime is to divide by all primes
   less than that number.
*/

fn is_prime(x : u64) -> bool {
	let mut i = 2;
	while i < (x / 2) {
		if x % i == 0 {
			return false;
		}
		i += 1;
	}
	return true;
}

fn problem_3() -> (u64, &'static str) {
	 let a : u64 = 600851475143;
	 let mut d = 2;
	 while d < (a / 2) {
	 	// Check that the result is an integer
	 	if a % d == 0 {
	 		match is_prime(a / d) {
	 			true => {return (a / d, DEFAULT_RETURN);},
	 			false => {}
	 		}
	 	}
	 	d += 1;
	 }
	 return (a, DEFAULT_RETURN);
}

fn problem_not_supported() -> (u64, &'static str) {
	return (0, ERROR_PROBLEM_NOT_IMPL);
}