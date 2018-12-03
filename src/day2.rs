use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{stdin, BufRead};

pub fn solve() {
    let stdin = stdin();
    let input = stdin
        .lock()
        .lines()
        .map(|line| line.expect("error reading line from input"))
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input).unwrap());
}

fn part1(input: &Vec<String>) -> i32 {
    let mut sums = (0, 0);
    for chars in input.iter().map(|s| s.chars()) {
        let mut counts = HashMap::new();
        for ch in chars {
            let counter = counts.entry(ch).or_insert(0);
            *counter += 1;
        }
        let counts = counts.values().collect::<HashSet<_>>();
        if counts.contains(&2) {
            sums.0 += 1;
        }
        if counts.contains(&3) {
            sums.1 += 1;
        }
    }

    sums.0 * sums.1
}

fn part2(input: &Vec<String>) -> Option<String> {
    for (i, c1) in input.iter().map(|s| s.chars()).enumerate() {
        for c2 in input.iter().skip(i).map(|s| s.chars()) {
            let zipped = c1.clone().zip(c2);
            let diffs = zipped
                .filter(|(x, y)| x != y)
                .map(|p| p.0)
                .collect::<Vec<_>>();
            if diffs.len() == 1 {
                let letter = diffs.iter().nth(0).unwrap();
                let solution = c1.clone().filter(|ch| ch != letter).collect::<String>();
                return Some(solution);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_logic() {
        assert_eq!(
            part1(&vec![
                "abcdef".to_string(),
                "bababc".to_string(),
                "abbcde".to_string(),
                "abcccd".to_string(),
                "aabcdd".to_string(),
                "abcdee".to_string(),
                "ababab".to_string(),
            ]),
            12
        );
    }

    #[test]
    fn test_part2_logic() {
        assert_eq!(
            part2(&vec![
                "abcde".to_string(),
                "fghij".to_string(),
                "klmno".to_string(),
                "pqrst".to_string(),
                "fguij".to_string(),
                "axcye".to_string(),
                "wvxyz".to_string(),
            ]),
            Some("fgij".to_string())
        );
    }
}
