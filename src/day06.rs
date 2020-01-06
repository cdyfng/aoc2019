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

    let mut count = 0;
    for mut k in orbits.keys() {
        println!("k:{}", k);
        while let Some(node) = orbits.get(k) {
            println!("  node:{:?}", node);
            k = &node.parent;
            count += 1;
        }
    }
    println!("{}", count);
}



