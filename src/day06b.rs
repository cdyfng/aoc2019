use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/day06.txt");



#[derive(Debug)]
struct Node {
    parent: String,
}

pub fn run() {
    let mut orbits: HashMap<String, Node> = HashMap::new();

    for line in INPUT.lines() {
        let x = &line[0..3];
        let y = &line[4..7];
        orbits.insert(y.into(), Node { parent: x.into() });
    }


    let mut you_map = HashMap::new();
    let mut k = "YOU";
    let mut hops = 0;
    while let Some(node) = orbits.get(k) {
        k = &node.parent;
        you_map.insert(k.clone(), hops);
        hops += 1;
    }

    k = "SAN";
    hops = 0;
    while let Some(node) = orbits.get(k) {
        k = &node.parent;
        if let Some(n) = you_map.get(k) {
            hops += *n;
            break;
        }
        println!("{:?}", k);
        hops += 1;
    }

    println!("{}", hops);
}



