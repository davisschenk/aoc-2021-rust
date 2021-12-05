const FILE: &str = include_str!("../data/p02");

enum Command {
    Forward(u64),
    Down(u64),
    Up(u64),
}

impl From<&str> for Command {
    fn from(data: &str) -> Self {
        let mut split = data.split(" ");
        let dir = split.next().unwrap();
        let amt: u64 = split.next().unwrap().parse().unwrap();
        match dir {
            "forward" => Self::Forward(amt),
            "down" => Self::Down(amt),
            "up" => Self::Up(amt),
            _ => panic!("Bad command"),
        }
    }
}

fn part_one() -> u64 {
    let mut depth: u64 = 0;
    let mut horizontal: u64 = 0;

    for line in FILE.lines() {
        match Command::from(line) {
            Command::Forward(amt) => horizontal += amt,
            Command::Down(amt) => depth += amt,
            Command::Up(amt) => depth -= amt,
        }
    }

    depth * horizontal
}

fn part_two() -> u64 {
    let mut depth: u64 = 0;
    let mut horizontal: u64 = 0;
    let mut aim: u64 = 0;

    for line in FILE.lines() {
        match Command::from(line) {
            Command::Forward(amt) => {
                horizontal += amt;
                depth += aim * amt;
            }
            Command::Down(amt) => aim += amt,
            Command::Up(amt) => aim -= amt,
        }
    }

    depth * horizontal
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
