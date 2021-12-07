use std::fs;

const BOARD_SIZE : usize = 5;

fn build_board(rows : Vec<[BoardCell; BOARD_SIZE]>) -> Board {
    Board { rows: [rows[0], rows[1], rows[2], rows[3], rows[4]] }
}

fn build_row(rows : Vec<i32>) -> [BoardCell; BOARD_SIZE] {
    let marked = false;
    [BoardCell {
        number: rows[0],
        marked
     },
     BoardCell {
        number: rows[1],
        marked
     },
     BoardCell {
        number: rows[2],
        marked
     },
     BoardCell {
        number: rows[3],
        marked
     },
     BoardCell {
        number: rows[4],
        marked
     }]
}

struct BoardCell {
    number : i32,
    marked : bool,
}

impl Copy for BoardCell { }

impl Clone for BoardCell {
    fn clone(&self) -> BoardCell {
        *self
    }
}

struct Board {
    rows: [[BoardCell; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {

    pub fn insert_value(&mut self, value : i32) -> bool {
        // Inserts a value into a board and then checks if bingo has been achieved
        for column in 0..5 {
            for row in 0..5 {
                if value == self.rows[column][row].number {
                    self.rows[column][row].marked = true;
                }
            }
        }
        // Search columns for bingo
        for row in self.rows {
            let mut num_marked = 0;
            for num in 0..5 {
                if row[num].marked == true {
                    num_marked += 1;
                }
            }
            if num_marked >= 5 {
                println!("Column found");
                return true;
            }
        }
        // Search rows for bingo
        for num in 0..5 {
            let mut num_marked = 0;
            for row in self.rows {
                if row[num].marked == true {
                    num_marked += 1;
                }
            }
            if num_marked >= 5 {
                println!("Row found");
                return true;
            }
        }
        // No bingo :(
        false
    }

    pub fn calculate_score(&self, final_value : i32 ) -> i64 {
        let mut sum = 0;
        for row in self.rows {
            for num in 0..5 {
                if row[num].marked == false {
                    sum += row[num].number;    
                }
            }
        }
        let score : i64 = (sum * final_value).into();
        println!("sum={} final_value={} score={}", sum, final_value, score);
        score
    }

    pub fn print(&self) {
        for row in self.rows {
            for num in 0..5 {
                print!("({} {}) ", row[num].number, row[num].marked);
            }
            println!("");
        }
        println!("");
    }
}

fn main() {
    let filename = "/home/griffin/src/aoc/day4/part1/input";

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
                .expect("Something went wrong reading the file");

    let split : Vec<String> = contents.trim().split("\n").map(|s| s.to_string()).collect();
    let guesses : Vec<i32> = split[0].split(",").map(|s| s.to_string().trim().parse().unwrap()).collect();

    let mut boards : Vec<Board> = Vec::new();
    let mut board_rows : Vec<[BoardCell; BOARD_SIZE]> = Vec::new();

    let mut line_num = 0;

    for lines in &split[1..]{
        if line_num == 0 {
            // skip whitespace line
            line_num += 1;
            continue;
        }
        // parse out numbers building the board
        let numbers : Vec<i32> = lines.split_whitespace().map(|s| s.to_string().trim().parse().unwrap()).collect();
        board_rows.push(build_row(numbers));
        if line_num >= 5 {
            // Add 5 rows as a valid board to vector of boards
            boards.push(build_board(board_rows));
            board_rows = Vec::new();
            line_num = 0;
        } else {
            line_num += 1;
        }
    }

    for board in &boards {
        board.print();
    }

    let mut bingo = false;
    let mut answers : Vec<i64> = Vec::new();
    for guess in guesses {
        println!("Adding guess {}", guess);
        for board in &mut boards {
            let this_board_bingo = board.insert_value(guess);
            if this_board_bingo == true {
                bingo = true;
                println!("BINGO");
                answers.push(board.calculate_score(guess));
            }
        }
        if bingo {
            break;
        }
    }

    println!("max_value={}", answers.iter().max().unwrap());

}
