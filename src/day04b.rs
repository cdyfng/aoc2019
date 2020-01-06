const INPUT: (i32, i32) = (353096, 843212);

pub fn run() {
    let mut candidate = INPUT.0;
    let mut count = 0;
    while candidate < INPUT.1 {
        let mut divisor = 100000;
        let mut prev_digit = 0;
        let mut run_length = 1;
        let mut has_double = false;
        for _ in 0..6 {
            let mut digit = (candidate / divisor) % 10;
            while digit < prev_digit {
                candidate -= candidate % divisor;
                candidate += divisor;
                digit += 1;
            }
            
            if digit == prev_digit {
                run_length += 1;
            } else {
                if run_length == 2 {
                    has_double = true;
                }
                run_length = 1;
            }
            prev_digit = digit;
            divisor /= 10;
        }
        if run_length == 2 {
            has_double = true;
        }

        if has_double && candidate < INPUT.1 {
            count += 1;
            println!("{}", candidate);
        }
        candidate += 1;
    }
    println!("{}", count);
}