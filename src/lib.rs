pub mod rust_euler {

	extern crate num;

	use std::collections::HashSet;

	pub fn problem_1() -> Option<u64> {
		let mut multiples = HashSet::new();
		for i in num::range_step(3, 1000, 3) {
			multiples.insert(i);
		}
		for i in num::range_step(5, 1000, 5) {
			multiples.insert(i);
		}
		let sum = multiples.iter().fold(0, |sum, x| sum + x);
		return Some (sum);
	}

	pub fn problem_2() -> Option<u64> {
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
		return Some (sum);
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

	pub fn problem_3() -> Option<u64> {
		 let a : u64 = 600851475143;
		 let mut d = 2;
		 while d < (a / 2) {
		 	// Check that the result is an integer
		 	if a % d == 0 {
		 		match is_prime(a / d) {
		 			true => return Some(a / d),
		 			false => {}
		 		}
		 	}
		 	d += 1;
		 }
		 return None;
	}

	pub fn problem_not_supported() -> Option<u64> {
		return None;
	}
}