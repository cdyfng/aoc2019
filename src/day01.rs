const INPUT: &str = include_str!("../inputs/day01.txt");
pub fn run(){
	println!("Hello adoc 2019!");
	//println!("Hello adoc 2019! {}", INPUT);
	let mut fuel = 0;
	for line in INPUT.lines() {
		let mass: i64 = line.parse().unwrap();
		fuel += (mass/3) -2;
	}
	println!("{}", fuel);
}