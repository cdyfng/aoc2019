const INPUT: &str = include_str!("../inputs/day01b.txt");

fn fuel(mass: i64) -> i64 {
	(mass/3) -2

}

fn fuel_with_fuel(mass: i64) -> i64 {
	let mut total_fuel = 0;
	let mut fuel_mass = fuel(mass);
	while fuel_mass > 0 {
		total_fuel += fuel_mass;
		fuel_mass = fuel(fuel_mass);
	}
	total_fuel

}

pub fn run(){
	println!("Hello adoc 2019!");
	//println!("Hello adoc 2019! {}", INPUT);
	let mut fuel = 0;
	for line in INPUT.lines() {
		let mass: i64 = line.parse().unwrap();
		fuel += fuel_with_fuel(mass);
	}
	println!("{}", fuel);
}