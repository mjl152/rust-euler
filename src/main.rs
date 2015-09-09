extern crate rustc_serialize;
extern crate docopt;
extern crate time;

extern crate rust_euler;

const USAGE: &'static str = "
rust_euler

Rust implementations of the Project Euler problems.

Usage: rust_euler [options]

Options:
	-p, --problem=K     Which Project Euler problem to solve [default: 1]
	-h, --help          Display this help and exit
	-b, --bench         Benchmark the specified problem
";

#[derive(Debug, RustcDecodable)]
struct Args {
	flag_problem: u8,
	flag_help: bool,
	flag_bench: bool,
}

fn main() {
	use rust_euler::rust_euler;
	let args: Args = docopt::Docopt::new(USAGE).and_then(|d| d.decode())
									   .unwrap_or_else(|e| e.exit());
	let mut problem : fn() -> (u64, &'static str);
	let mut iterations = 30;
	let mut function_result : (u64, &'static str);
	match args.flag_problem {
		1 => problem = rust_euler::problem_1,
		2 => problem = rust_euler::problem_2,
		3 => problem = rust_euler::problem_3,
		_ => problem = rust_euler::problem_not_supported,
	}
	if args.flag_bench == false {
		iterations = 1;
	}
	function_result = problem();
	let start = time::precise_time_ns();
	for i in 0..iterations {
		function_result = problem();
	}
	let stop = time::precise_time_ns();
	let time_taken : f64 = ((stop - start) as f64) / (iterations as f64) / 1000000000.0;
	println!("{}", function_result.1);
	println!("{}", function_result.0);
	if args.flag_bench {
		println!("Took {} s / iteration.", time_taken)
	}
}
