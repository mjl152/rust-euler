use std::collections::HashSet;
extern crate num;

// Find sum of all multiples of 3 or 5 below 1000
// 
fn main() {
	problem_1();
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
	// Odd + Odd = Even
	// Even + Odd = Odd
	// Odd + Even = Odd
	// Even + Even = Even
	// 0, 1, 2, 3, 5, 8, 13, 21 ...
	//    Odd Even Odd Odd Even Odd Odd ...
	// We know that every 3rd Fibonacci number is even
	// so calculate every 3rd Fibonacci number and sum
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