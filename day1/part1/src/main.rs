use std::fs;

fn main() {
	// Hardcoded so nobody can steal my answer >:(
    let filename = "/home/griffin/src/aoc/day1/part1/input";

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
                .expect("Something went wrong reading the file");

	let split = contents.trim().split("\n");

	let mut is_first_value = true;
	let mut old_value = 0;
	let mut number_of_increases = 0;

	for value in split {
		match is_first_value {
			true => is_first_value = false,
			false => {
				let value_parsed: u32 = value.trim().parse().expect("Please type a number!");
				if value_parsed > old_value {
					number_of_increases += 1;
				}
				old_value = value_parsed;
			}
		};
	}
	println!("{}", number_of_increases);

}
