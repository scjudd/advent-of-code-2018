use std::io::{stdin, BufRead};
use std::collections::HashSet;

pub fn solve() {
    let stdin = stdin();
    let input = stdin
        .lock()
        .lines()
        .map(|line| line.expect("error reading line from input"))
        .map(|line| line.parse::<i32>().expect("invalid input"))
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

fn part2(input: &[i32]) -> i32 {
    let mut sum = 0;
    let mut set = HashSet::new();
    set.insert(sum);
    for drift in input.iter().cycle() {
        sum += drift;
        if set.contains(&sum) {
            break;
        } else {
            set.insert(sum);
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_logic() {
        assert_eq!(part1(&vec![1, -2, 3, 1]), 3);
    }

    #[test]
    fn test_part2_logic() {
        assert_eq!(part2(&vec![1, -1]), 0);
        assert_eq!(part2(&vec![3, 3, 4, -2, -4]), 10);
        assert_eq!(part2(&vec![-6, 3, 8, 5, -6]), 5);
        assert_eq!(part2(&vec![7, 7, -2, -7, -4]), 14);
    }
}
