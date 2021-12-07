use std::cmp::{max, min};
use std::fmt::Display;

const FILE: &str = include_str!("../data/p05");

#[derive(Debug)]

struct Line {
    start: (u64, u64),
    end: (u64, u64),
}

impl Line {
    fn new(data: &str) -> Self {
        let (r, l) = data.split_once(" -> ").unwrap();

        let mut r_s = r.split(",");
        let mut l_s = l.split(",");

        Self {
            start: (
                r_s.next().unwrap().parse().unwrap(),
                r_s.next().unwrap().parse().unwrap(),
            ),
            end: (
                l_s.next().unwrap().parse().unwrap(),
                l_s.next().unwrap().parse().unwrap(),
            ),
        }
    }
}

#[derive(Debug)]
struct Field {
    field: Vec<Vec<u8>>,
}

impl Field {
    pub fn new(data: &str, diags: bool) -> Self {
        let lines = data.lines().map(|l| Line::new(l)).collect();
        let (max_x, max_y) = Self::find_max(&lines);
        let mut field = vec![];

        for _ in 0..max_y + 1 {
            field.push(vec![0; max_x as usize + 1])
        }

        for line in &lines {
            // println!("{},{} -> {},{}", line.start.0, line.start.1, line.end.0, line.end.1);
            if line.start.0 == line.end.0 {
                let start = min(line.start.1, line.end.1);
                let end = max(line.start.1, line.end.1);

                for y in start..=end {
                    field[y as usize][line.start.0 as usize] += 1;
                }
            } else if line.start.1 == line.end.1 {
                let start = min(line.start.0, line.end.0);
                let end = max(line.start.0, line.end.0);

                for x in start..=end {
                    field[line.start.1 as usize][x as usize] += 1;
                }
            } else if diags {
                let x_dir = if line.start.0 < line.end.0 { 1 } else { -1 };
                let y_dir = if line.start.1 < line.end.1 { 1 } else { -1 };

                for distance in 0..=(line.end.0 as isize - line.start.0 as isize).abs() {
                    let x = line.start.0 as isize + distance * x_dir;
                    let y = line.start.1 as isize + distance * y_dir;

                    field[y as usize][x as usize] += 1;
                }
            }
        }

        Self { field }
    }

    fn find_max(lines: &Vec<Line>) -> (u64, u64) {
        let (mut max_x, mut max_y) = (0, 0);
        for line in lines {
            max_x = std::cmp::max(max_x, line.end.0);
            max_y = std::cmp::max(max_y, line.end.1);
            max_x = std::cmp::max(max_x, line.start.0);
            max_y = std::cmp::max(max_y, line.start.1);
        }

        (max_x, max_y)
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.field {
            for amt in row {
                if *amt == 0 {
                    write!(f, ".").expect("Failed to write");
                } else {
                    write!(f, "{}", amt).expect("Failed to write");
                }
            }
            write!(f, "\n").expect("Failed to write")
        }

        Ok(())
    }
}
fn part_one() -> usize {
    let field = Field::new(FILE, false);
    let mut overlap = 0;

    for row in field.field {
        for point in row {
            if point >= 2 {
                overlap += 1;
            }
        }
    }

    overlap
}

fn part_two() -> usize {
    let field = Field::new(FILE, true);
    let mut overlap = 0;

    for row in field.field {
        for point in row {
            if point >= 2 {
                overlap += 1;
            }
        }
    }

    overlap
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
        assert_eq!(part_one(), 6189);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 19164);
    }
}
