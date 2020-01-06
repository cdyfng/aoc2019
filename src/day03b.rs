use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/day03.txt");


fn parse(input: &str) -> HashMap<(i32, i32), usize>{
	let mut s = 0;
	let mut pos = (0, 0);
	let mut result = HashMap::new();
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
    		s += 1;
            result.insert(pos, s);
    	}
    }
	result
}



pub fn run(){
	println!("Hello adoc 2019 day03b");
	let lines: Vec<_> = INPUT.lines().collect();
	let w1 = lines[0];
	let w2 = lines[1];
	let hs1 = parse(w1);
	let hs2 = parse(w2);
	println!("intersection: {:?}",  hs1.iter().filter_map(|(k, v)| hs2.get(k).map(|u| u + v)).min());
}


