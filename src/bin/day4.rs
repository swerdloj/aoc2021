use std::collections::{HashMap, HashSet};

// The original grid isn't even needed
struct Board {
    locations: HashMap<u32, (usize, usize)>,
    marked: HashSet<(usize, usize)>,
    unmarked_sum: u32,
    is_solved: bool,
}

impl Board {
    fn new(locations: HashMap<u32, (usize, usize)>, unmarked_sum: u32) -> Self {
        Self {
            locations,
            marked: HashSet::new(),
            unmarked_sum,
            is_solved: false,
        }
    }

    // Returns `true` if solved
    fn update(&mut self, number: u32) -> bool {
        // Stop updating if already solved
        // FIXME: This is a hack for part 2 -- only doing this to avoid pushing multiple winning numbers after board is solved the first time 
        if self.is_solved {
            return false;
        }

        // Check if the board is solved
        if let Some((row, col)) = self.locations.get(&number) {
            self.marked.insert((*row, *col));
            self.unmarked_sum -= number;

            let mut solved = false;

            // Check if row is solved
            for i in 0..5 {
                if self.marked.contains(&(*row, i)) {
                    solved = true;
                } else {
                    solved = false;
                    break;
                }
            }
            
            if solved {
                self.is_solved = true;
                return true;
            }
            
            // Check if column is solved
            for i in 0..5 {
                if self.marked.contains(&(i, *col)) {
                    solved = true;
                } else {
                    solved = false;
                    break;
                }
            }

            self.is_solved = solved;
            solved
        } else {
            false
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("./inputs/day4.txt").unwrap();

    // Boards are a 5x5 grid
    let mut boards = Vec::new();

    let mut lines = input.lines();

    let numbers = lines.next().unwrap().split(',')
        .map(|number| number.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut row = 0;
    let mut locations = HashMap::new();
    let mut sum = 0;

    // Skip the first empty line
    lines.next();

    // Gather board data into a workable data structure
    for line in lines {
        let mut is_board = false;

        for (column, number) in line.split(' ').filter(|&number| number != " " && !number.is_empty()).enumerate() {
            is_board = true;
            let number = number.parse::<u32>().unwrap();

            locations.insert(number, (row, column));
            sum += number;
        }
        
        if is_board {
            row += 1;
        } else {
            boards.push(Board::new(locations, sum));
            row = 0;
            sum = 0;
            locations = HashMap::new();
        }
    }
    // Add the last board
    boards.push(Board::new(locations, sum));

    let mut winning_boards = Vec::new();
    let mut winning_numbers = Vec::new();

    // Run through the bingo game, tracking wins
    for number in numbers {
        for (i, board) in boards.iter_mut().enumerate() {
            if board.update(number) {
                winning_boards.push(i);
                winning_numbers.push(number);
            }
        }
    }

    println!("Part 1: {}", boards[winning_boards[0]].unmarked_sum * winning_numbers[0]);
    println!("Part 2: {}", boards[*winning_boards.last().unwrap()].unmarked_sum * winning_numbers.last().unwrap());
}