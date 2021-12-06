use std::fs;

fn main() {
	// Hardcoded so nobody can steal my answer >:(
    let filename = "/home/griffin/src/aoc/aoc3/input";

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
                .expect("Something went wrong reading the file");

	let split = contents.trim().split("\n");

	let mut depth = 0;
	let mut position = 0;

	for value in split {
	    let split_value: Vec<&str> = value.split(" ").collect();
		let distance: u32 = split_value[1].trim().parse().expect("Please type a number!");
		let command = split_value[0];
		match command {
		    "forward" => {
		        position += distance;
            },
            "down" => {
                depth += distance;
            },
            "up" => {
                depth -= distance;

            }
            _ => panic!("Unexpected value {}", command)
        }
	}
	println!("position={} depth={} total={}", position, depth, position * depth);
}
