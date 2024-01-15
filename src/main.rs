use std::{collections::VecDeque, io, process};

/// Small CLI representation of the puzzle called Tower of Hanoi which I run into in the book "Concrete Mathematics" by Donald E. Knuth
///
/// The puzzle itself looks like this:
/// - three pegs are given:
///
///         A           B           C
///
///         |           |           |
///        -|-          |           |
///       --|--         |           |
///      ---|---        |           |
///     ----|----       |           |
///
/// - each horizontal line on the A peg represents a disk, as we go lower the disks are larger in diameter forming a tower, pegs B and C are empty
/// - THE OBJECTIVE is to move the entier tower to one of the other pegs by:
/// 1. Moving only one disk at a time
/// 2. Never moving a larger one into a smaller
///
/// TODO:
/// - Implement a simple UNDO :: REDO mechanism

type Tower = VecDeque<usize>;

#[derive(Debug)]
struct Towers {
    A: Tower,
    B: Tower,
    C: Tower,
}

#[derive(Debug)]
struct TowerOfHanoi {
    towers: Towers,
    size: usize,
}

fn main() {
    let mut game: TowerOfHanoi = TowerOfHanoi::new(3);
    game.towers.print_towers(game.size);
    game.run();
}

impl Towers {
    fn new(size: usize) -> Self {
        let mut towers: Self = Towers {
            A: VecDeque::new(),
            B: VecDeque::new(),
            C: VecDeque::new(),
        };

        for i in 0..size {
            towers.A.push_front(i + 1);
        }

        towers
    }

    pub fn print_towers(&self, size: usize) {
        println!("  A   B   C");
        println!("-------------");
        for i in 0..size {
            println!(
                "{:3} {:3} {:3}",
                self.A.get(size - i - 1).unwrap_or(&(0 as usize)),
                self.B.get(size - i - 1).unwrap_or(&(0 as usize)),
                self.C.get(size - i - 1).unwrap_or(&(0 as usize))
            );
        }
    }
}

impl TowerOfHanoi {
    fn new(size: usize) -> Self {
        TowerOfHanoi {
            towers: Towers::new(size),
            size,
        }
    }

    pub fn run(&mut self) {
        loop {
            let mut input: String = String::new();
            if input.to_uppercase().eq(&"X") {
                break;
            }

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let parts: Vec<&str> = input.trim().split('>').collect();
            println!("{:?}", parts);

            let from = *parts.get(0).unwrap();
            let popped_element = match from {
                "A" => self.towers.A.pop_back().unwrap(),
                "B" => self.towers.B.pop_back().unwrap(),
                "C" => self.towers.C.pop_back().unwrap(),
                _ => {
                    eprintln!("Invalid Input");
                    continue;
                }
            };

            let to = *parts.get(1).unwrap();
            let to_top = match to {
                "A" => self.towers.A.pop_back().unwrap_or(0),
                "B" => self.towers.B.pop_back().unwrap_or(0),
                "C" => self.towers.C.pop_back().unwrap_or(0),
                _ => {
                    eprintln!("Invalid Input");
                    continue;
                }
            };

            if to_top != 0 && popped_element > to_top {
                eprintln!(
                    "Invalid Move, {}::{} is greater than {}::{}",
                    from, popped_element, to, to_top
                );
                process::exit(1);
            } else {
                match to {
                    "A" => self.towers.A.push_back(popped_element),
                    "B" => self.towers.B.push_back(popped_element),
                    "C" => self.towers.C.push_back(popped_element),
                    _ => {
                        eprintln!("Invalid Input");
                        continue;
                    }
                }
            }

            self.towers.print_towers(self.size);
        }
    }
}
