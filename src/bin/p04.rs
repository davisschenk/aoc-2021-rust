use core::num;

const FILE: &str = include_str!("../data/p04");

type Number = (u8, bool);
type Board = Vec<Vec<Number>>;

#[derive(Debug)]
struct Game {
    numbers: Vec<u8>,
    boards: Vec<Board>,
}

impl Game {
    fn new(input: &str) -> Game {
        let mut lines = input.split("\n\n");

        let numbers: Vec<u8> = lines
            .next()
            .expect("Bad input file")
            .split(",")
            .map(|s| s.parse::<u8>().unwrap())
            .collect();

        let mut boards = vec![];
        for raw_board in lines {
            let mut board: Board = raw_board
                .split("\n")
                .map(|r| {
                    r.split_whitespace()
                        .map(|f| (f.parse().unwrap(), false))
                        .collect()
                })
                .collect();

            board.retain(|f| !f.is_empty());
            boards.push(board);
        }

        Game { numbers, boards }
    }
}

fn board_winning(board: &Board) -> bool {
    for row in board {
        if row.iter().all(|r| r.1) {
            return true;
        }
    }

    for ci in 0..board[0].len() {
        let mut all = true;
        for row in 0..board.len() {
            if !board[row][ci].1 {
                all = false;
            }
        }

        if all {
            return true;
        }
    }

    false
}

fn mark_number(board: &mut Board, number: u8) {
    for row in board {
        for n in row {
            let (spot, marked) = n;
            if *spot == number {
                *marked = true;
            }
        }
    }
}

fn print_board(board: &Board) {
    for row in board {
        for n in row {
            let (num, marked) = n;

            if *marked {
                print!("\x1b[93m{:02}\x1b[0m ", num)
            } else {
                print!("{:02} ", num)
            }
        }

        println!()
    }
    println!()
}

fn part_one() -> u64 {
    let mut game = Game::new(FILE);

    for number in game.numbers {
        for board in &mut game.boards {
            mark_number(board, number);

            if board_winning(&board) {
                let mut sum: u64 = 0;
                for row in board {
                    for n in row {
                        if let (amt, false) = n {
                            sum += *amt as u64;
                        }
                    }
                }

                return sum * number as u64;
            }
        }
    }

    panic!("No game won")
}

fn part_two() -> u64 {
    let mut game = Game::new(FILE);

    for number in game.numbers {
        for board in &mut game.boards {
            mark_number(board, number);
        }

        if game.boards.len() <= 1 {
            if board_winning(&game.boards[0]) {
                print_board(&game.boards[0]);
                let mut sum: u64 = 0;
                for row in &game.boards[0] {
                    for n in row {
                        if let (amt, false) = n {
                            sum += *amt as u64;
                        }
                    }
                }

                return sum * number as u64;
            }
        }
        game.boards.retain(|f| !board_winning(f));
    }

    panic!("Failed to find last game")
}

fn main() {
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 1893605);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 2120734350);
    }
}
