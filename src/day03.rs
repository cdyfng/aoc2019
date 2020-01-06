use std::collections::HashSet;

const INPUT: &str = include_str!("../inputs/day03.txt");


fn parse(input: &str) -> HashSet<(i32, i32)>{
	let mut pos = (0, 0);
	let mut result = HashSet::new();
    for step in input.trim().split(",") {
    	let (dir, dist_str) = step.split_at(1);
    	let dist: usize = dist_str.parse().unwrap();
    	for _ in 0..dist {
    		match dir {
    			"U" => pos.1 -=1,
    			"D" => pos.1 += 1,
                "L" => pos.0 -= 1,
                "R" => pos.0 += 1,
                _ => panic!("{}", dir),
    		}
            result.insert(pos);
    	}
    }
	result
}

fn distance(h1: HashSet<(i32, i32)>, h2: HashSet<(i32, i32)>) -> i32 {
	h1.intersection(&h2).map(|p| p.0.abs() + p.1.abs()).min().unwrap()
}



#[test]
fn example0() {
	let w1 = "R8,U5,L5,D3";
	let w2 = "U7,R6,D4,L4";
	let hs1 = parse(w1);
	let hs2 = parse(w2);
	let d = distance(hs1, hs2);	//hs1.intersection(&hs2).map(|p| p.0.abs() + p.1.abs()).min(); //
	assert_eq!(d, 6);
}

#[test]
fn example1() {
	let w1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
	let w2 = "U62,R66,U55,R34,D71,R55,D58,R83";
	let hs1 = parse(w1);
	let hs2 = parse(w2);
	println!("{:?} \n {:?}", hs1, hs2);
	let d = distance(hs1, hs2);	 
	assert_eq!(d, 159);
}

#[test]
fn example2() {
	let w1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
	let w2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
	let hs1 = parse(w1);
	let hs2 = parse(w2);
	println!("{:?} \n {:?}", hs1, hs2);
	let d = distance(hs1, hs2); 
	assert_eq!(d, 135);
}



pub fn run(){
	println!("Hello adoc 2019 day03!");
	let lines: Vec<_> = INPUT.lines().collect();
	let w1 = lines[0];
	let w2 = lines[1];
	let hs1 = parse(w1);
	let hs2 = parse(w2);
	//let hs1_1:HashSet<(i32, i32)>= hs1.iter().cloned().collect();
	//let hs2_1:HashSet<(i32, i32)>= hs2.iter().cloned().collect();
	println!("wire1: {:?} \n wire2: {:?}", hs1, hs2);
	let d = distance(hs1.iter().cloned().collect(), hs2.iter().cloned().collect());
	println!("intersection: {:?}", hs1.intersection(&hs2));
	println!("distance: {:?}", d);
}


