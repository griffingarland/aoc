use std::fs;

fn main() {
	// Hardcoded so nobody can steal my answer >:(
    let filename = "/home/griffin/src/aoc/aoc1/input";

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
                .expect("Something went wrong reading the file");

	let split = contents.trim().split("\n");

	let mut number_of_seen_values = 0;
    let mut first_value = 0;
    let mut second_value = 0;
    let mut third_value = 0;
	let mut number_of_increases = 0;

	for value in split {
		let value_parsed: u32 = value.trim().parse().expect("Please type a number!");
		match number_of_seen_values {
			0 => { 
                first_value = value_parsed;
                number_of_seen_values += 1;
            },
		    1 => {
                second_value = value_parsed;
                number_of_seen_values += 1;
			},
			2 => {
                third_value = value_parsed;
                number_of_seen_values += 1;
            },
			_ => {
			    let old_value = first_value + second_value + third_value;
			    let new_value = second_value + third_value + value_parsed;

			    if new_value > old_value {
					number_of_increases += 1;
                }

			    first_value = second_value;
			    second_value = third_value;
			    third_value = value_parsed;
			}
		};
	}
	println!("{}", number_of_increases);

}
