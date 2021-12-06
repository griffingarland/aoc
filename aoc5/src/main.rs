use std::fs;
use num_traits::pow;

fn main() {
	// Hardcoded so nobody can steal my answer >:(
    let filename = "/home/griffin/src/aoc/aoc5/input";

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
                .expect("Something went wrong reading the file");

	let split = contents.trim().split("\n");

    // Number of bits in each binary number
	let mut bit_shift : i64 = pow(2i64, 11);
	let mut gamma : i64 = 0;
	let mut epsilon : i64 = 0;

    loop {
        let split_clone = split.clone();
        let mut num_ones = 0;
        let mut num_zeros = 0;
        for value in split_clone {
            let number : i64 = i64::from_str_radix(value.trim(), 2).unwrap();
            if (number & bit_shift) > 0 {
                num_ones += 1;
            } else {
                num_zeros += 1;
            }
        }
        if num_ones > num_zeros {
            gamma = gamma | bit_shift;
        } else {
            epsilon = epsilon | bit_shift;
        }
        bit_shift = bit_shift >> 1;
        if bit_shift == 0 {
            break;
        }
    }
	println!("gamma={} epsilon={} multiplied={}", gamma, epsilon, gamma * epsilon);
}
