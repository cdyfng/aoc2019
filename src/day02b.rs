const INPUT: &str = include_str!("../inputs/day02.txt");

fn compute(program: &mut [u32]){
	let mut pc = 0; 
	loop {
		match program[pc] {
			1 => program[program[pc+3] as usize] = program[program[pc+1] as usize] + program[program[pc+2] as usize], 
			2 => program[program[pc+3] as usize] = program[program[pc+1] as usize] * program[program[pc+2] as usize],
			99 => break,
			_ => panic!("unkonw opcode {} at {}", program[pc], pc),
		}
		pc += 4;
	}
}

#[test]
fn example0() {
	let mut program = vec![1,0,0,0,99];
	compute(&mut program);
	assert_eq!(program, vec![2,0,0,0,99]);
}

#[test]
fn example1() {
	let mut program = vec![2,3,0,3,99];
	compute(&mut program);
	assert_eq!(program, vec![2,3,0,6,99]);
}

#[test]
fn example2() {
	let mut program = vec![2,4,4,5,99,0];
	compute(&mut program);
	assert_eq!(program, vec![2,4,4,5,99,9801]);
}

#[test]
fn example3() {
	let mut program = vec![1,1,1,4,99,5,6,0,99];
	compute(&mut program);
	assert_eq!(program, vec![30,1,1,4,2,5,6,0,99]);
}



pub fn run(){
	println!("Hello adoc 2019 day02!");

    let mut program = vec![];
    for number in INPUT.trim().split(",") {
            program.push(number.parse::<u32>().unwrap());
    }


    let original_program = program.to_vec();

    'outer: for noun in 0..100 {
        for verb in 0..100 {
            program = original_program.to_vec();
            program[1] = noun;
            program[2] = verb;
            compute(&mut program);
            if program[0] == 19690720 {
                println!("Found matching program. Noun = {}, verb = {}, answer is therefore: {}", noun, verb, 100 * noun + verb);
                break 'outer;
            } else {
            	println!("Noun = {}, verb = {}, answer is therefore: {}, {}", noun, verb, 100 * noun + verb, program[0]);
            }
        }
    }

}


