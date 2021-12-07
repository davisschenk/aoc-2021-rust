const FILE: &str = include_str!("../data/p06");

fn load_file() -> Vec<u8> {
    FILE.split(",").map(|f| f.parse().unwrap()).collect()
}

fn simulate_fishes(fishes: Vec<u8>, days: usize) -> [usize ; 9] {
    let mut better_fishes = [0usize ; 9];
    for fish in fishes {
        better_fishes[fish as usize] += 1;
    }
    
    for _day in 0..days {
        better_fishes.rotate_left(1);
        better_fishes[6] += better_fishes[8];
    }

    better_fishes

}

fn part_one() -> usize {
    let fishes = load_file();

   let fishes = simulate_fishes(fishes, 80);

    fishes.iter().sum()
}

fn part_two() -> usize {
    let fishes = load_file();

    let fishes = simulate_fishes(fishes, 256);
 
     fishes.iter().sum()
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
        assert_eq!(part_one(), 380612);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 1710166656900);
    }
}
