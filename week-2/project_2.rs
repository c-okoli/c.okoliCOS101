fn main() {
	let qtoshiba:f64 = 450_000.00;
	let atoshiba:f64 = 2.0;
	let qmac:f64 = 1_500_000.00;
	let amac:f64 = 1.0;
	let qhp:f64 = 750_000.00;
	let ahp:f64 = 3.0;
	let qdell:f64 = 2_850_000.00;
	let adell:f64 = 3.0;
	let qacer:f64 = 250_000.00;
	let aacer:f64 = 1.0;

	// sum
	let s:f64 = (qtoshiba * atoshiba) + (qmac * amac) + (qhp * ahp) + (qdell * adell) + (qacer * aacer);
	println!("The Sum is {}", s);
	let a:f64 = s / 5.0;
	println!("The Average is {}", a);

}
