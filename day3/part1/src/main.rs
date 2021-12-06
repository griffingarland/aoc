use std::fs;
use num_traits::pow;

fn main() {
    // Hardcoded so nobody can steal my answer >:(
    let filename = "/home/griffin/src/aoc/day3/part1/input";

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
                .expect("Something went wrong reading the file");

    let split : Vec<String> = contents.trim().split("\n").map(|s| s.to_string()).collect();

    // Number of bits in each binary number
    let mut bit_map : i64 = pow(2i64, 11);
    let mut gamma : i64 = 0;
    let mut epsilon : i64 = 0;

    loop {
        let mut num_ones = 0;
        let mut num_zeros = 0;
        for value in &split {
            let number : i64 = i64::from_str_radix(value.trim(), 2).unwrap();
            if (number & bit_map) > 0 {
                num_ones += 1;
            } else {
                num_zeros += 1;
            }
        }
        if num_ones > num_zeros {
            gamma = gamma | bit_map;
        } else {
            epsilon = epsilon | bit_map;
        }
        bit_map = bit_map >> 1;
        if bit_map == 0 {
            break;
        }
    }
    println!("gamma={} epsilon={} multiplied={}", gamma, epsilon, gamma * epsilon);
}
