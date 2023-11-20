use std::io;

fn main() {
	
	let mut input = String::new();
	println!("
	Enter the value of n:
	");
	io::stdin()
	.read_line(&mut input)
	.expect("Failed to read line");
	let n:u32 = input.trim().parse().expect("Please entera value");


	for a in 1..=n {
	    for b in 1..=12 {
	        let c = a * b;
	        println!("
	        {} x {} = {}", a, b, c);
	    }
	}
}