use std::collections::HashSet;
extern crate num;

// Find sum of all multiples of 3 or 5 below 1000
// 
fn main() {
	let mut multiples = HashSet::new();
	for i in num::range_step(3, 1000, 3) {
		multiples.insert(i);
	}
	for i in num::range_step(5, 1000, 5) {
		multiples.insert(i);
	}
	let mut sum = 0;
	for multiple in &multiples {
		sum = sum + multiple;
	}
	println!("{}", sum);
}