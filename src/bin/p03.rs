use core::num;

const FILE: &str = include_str!("../data/p03");
const LENGTH: usize = 12;

type Count = (u64, u64);
type Counts = Vec<Count>;

fn find_counts(numbers: &str) -> Counts {
    let mut counts = vec![(0, 0); LENGTH];

    for number in numbers.lines() {
        for (index, chr) in number.chars().enumerate() {
            match chr {
                '0' => counts[index].0 += 1,
                '1' => counts[index].1 += 1,
                _ => panic!("Unrecognized character"),
            }
        }
    }
    counts
}

fn part_one() -> u64 {
    let counts = find_counts(FILE);
    let mut gamma = String::new();
    let mut epsilon = String::new();

    for (ones, zeroes) in counts {
        if ones > zeroes {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    u64::from_str_radix(&gamma, 2).unwrap() * u64::from_str_radix(&epsilon, 2).unwrap()
}

fn part_two() -> u64 {
    let mut o2: Vec<&str> = FILE.lines().collect();
    let mut co2: Vec<&str> = FILE.lines().collect();

    for index in 0..LENGTH {
        let (mut ones, mut zeroes) = (0, 0);
        for number in &o2 {
            match number.chars().nth(index).unwrap() {
                '0' => zeroes += 1,
                '1' => ones += 1,
                _ => panic!("Unrecognized character"),
            }
        }

        if ones >= zeroes {
            o2.retain(|f| f.chars().nth(index).unwrap() == '1');
        } else {
            o2.retain(|f| f.chars().nth(index).unwrap() == '0');
        }
    }

    for index in 0..LENGTH {
        let (mut ones, mut zeroes) = (0, 0);
        for number in &co2 {
            match number.chars().nth(index).unwrap() {
                '0' => zeroes += 1,
                '1' => ones += 1,
                _ => panic!("Unrecognized character"),
            }
        }

        if ones >= zeroes {
            co2.retain(|f| f.chars().nth(index).unwrap() == '0');
        } else {
            co2.retain(|f| f.chars().nth(index).unwrap() == '1');
        }

        if co2.len() == 1 {
            break;
        }
    }

    u64::from_str_radix(co2[0], 2).unwrap() * u64::from_str_radix(o2[0], 2).unwrap()
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
        assert_eq!(part_one(), 4001724);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 2120734350);
    }
}
