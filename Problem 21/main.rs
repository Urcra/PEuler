fn main() {
	let mut amicsum = 0;

	for i in 1..10000 {
		let sumdiv = sumofdiv(i);
		if i == sumofdiv(sumdiv) && sumdiv != i {
			amicsum += i;
		}
	}

	println!("{:}", amicsum);

}

fn sumofdiv(num: u32) -> u32 {
	let mut sum = 0;
	for i in 1..(num/2 + 1) {
		if num % i == 0 {
			sum += i;
		}
	}
	sum
}