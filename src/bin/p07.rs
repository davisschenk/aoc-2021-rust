const FILE: &str = include_str!("../data/p07");

fn load_file() -> Vec<usize> {
    FILE.split(",").map(|f| f.parse().unwrap()).collect()
}

fn part_one() -> usize {
    let crabs = load_file();

    let max_crab = crabs.iter().max().unwrap();
    let min_crab = crabs.iter().min().unwrap();

    (*min_crab..*max_crab).map(|index| {
        crabs.iter().map(|c| (*c as isize - index as isize).abs()).sum::<isize>()
    }).min().unwrap() as usize
}

fn part_two() -> usize {
    let crabs = load_file();

    let max_crab = crabs.iter().max().unwrap();
    let min_crab = crabs.iter().min().unwrap();

    (*min_crab..*max_crab).map(|index| {
        crabs.iter().map(|c| {
            let a = (*c as isize - index as isize).abs();
            (a+1) * a / 2
        } ).sum::<isize>()
    }).min().unwrap() as usize
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
        assert_eq!(part_one(), 336040);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 94813675);
    }
}
