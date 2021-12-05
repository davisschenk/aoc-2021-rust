const FILE: &str = include_str!("../data/p01");

fn part_one() -> usize {
    FILE.split_whitespace()
        .map(|f| f.parse::<i16>().unwrap())
        .collect::<Vec<i16>>()
        .windows(2)
        .filter(|f| f[0] < f[1])
        .count()
}

fn part_two() -> usize {
    FILE.split_whitespace()
        .map(|f| f.parse::<i16>().unwrap())
        .collect::<Vec<i16>>()
        .windows(4)
        .filter(|f| f[0] + f[1] + f[2] < f[1] + f[2] + f[3])
        .count()
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
        assert_eq!(part_one(), 1266);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 1217);
    }
}
