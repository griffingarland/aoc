use std::fs;
use num_traits::pow;

const BIT_SIZE : usize = 11;

fn sort_array(input : Vec<String> ) -> Vec<i64> {
    let mut sorted_array : Vec<i64> = input.iter().map(|s| i64::from_str_radix(s.trim(), 2).unwrap()).collect();
    sorted_array.sort();
    sorted_array
}

fn get_oxygen(sorted_array : &Vec<i64>) -> i64 {
    let mut start : usize = 0;
    let mut end : usize = sorted_array.len();

    // Number of bits in each binary number
    let mut bit_map : i64 = pow(2i64, BIT_SIZE);
    let mut oxygen = 0;

    // This is sorted so I could binary search, but I dont know how to do that easily.
    // So I will just use position to search through the array until it finds a 1
    loop {

        let index : usize = match &sorted_array[start..end].iter().position(|&r| (r & bit_map) != 0) {
            Some(v) => *v,
            None => {
                // did not find a 1
                bit_map = bit_map >> 1;
                if bit_map == 0 {
                    break;
                }
                continue;
            }
        };
        let index = index + start;
        let start_diff :i64 = (index - start) as i64;
        let end_diff :i64 = (end - index) as i64;
        if end_diff - start_diff >= 0 {
            // more 1s
            println!("More 1s");
            start = index;

        } else {
            // more 0s
            println!("More 0s");
            if index < end {
                end = index;
            }
        }
        bit_map = bit_map >> 1;
        if start == end - 1 {
            oxygen = sorted_array[start];
        }
    }
    println!("oxygen= {:#06b}", oxygen);

    oxygen
}

fn get_co2(sorted_array : &Vec<i64>) -> i64 {
    let mut start : usize = 0;
    let mut end : usize = sorted_array.len();

    // Number of bits in each binary number
    let mut bit_map : i64 = pow(2i64, BIT_SIZE);
    let mut co2 = 0;

    // This is sorted so I could binary search, but I dont know how to do that easily.
    // So I will just use position to search through the array until it finds a 1
    loop {
        for number in &sorted_array[start..end] {
            println!("number= {:#07b}", number);
        }
        let index : usize = match &sorted_array[start..end].iter().position(|&r| (r & bit_map) != 0) {
            Some(v) => *v,
            None => {
                // did not find a 1
                bit_map = bit_map >> 1;
                if bit_map == 0 {
                    break;
                }
                continue;
            }
        };
        let index = index + start;
        let start_diff :i64 = (index - start) as i64;
        let end_diff :i64 = (end - index) as i64;
        if end_diff - start_diff >= 0 {
            // more 1s
            println!("More 1s");
            if index < end {
                end = index;
            }

        } else {
            // more 0s
            start = index;
            println!("More 0s");
        }
        bit_map = bit_map >> 1;
        if start == end - 1 {
            co2 = sorted_array[start];
        }
    }
    println!("co2= {:#06b}", co2);

    co2 
}

fn main() {
    // Hardcoded so nobody can steal my answer >:(
    let filename = "/home/griffin/src/aoc/day3/part1/input";

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
                .expect("Something went wrong reading the file");

    let split : Vec<String> = contents.trim().split("\n").map(|s| s.to_string()).collect();
    let sorted_array = sort_array(split);

    let oxygen = get_oxygen(&sorted_array);
    let co2 = get_co2(&sorted_array);

    println!("oxygen={} co2={} multipled={}", oxygen, co2, oxygen*co2);
}
