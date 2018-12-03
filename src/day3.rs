use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{stdin, BufRead};
use std::iter::FromIterator;
use std::str::FromStr;

pub fn solve() {
    let stdin = stdin();
    let input = stdin
        .lock()
        .lines()
        .map(|line| line.expect("error reading line from input"))
        .map(|line| line.parse::<Claim>().unwrap())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Vec<Claim>) -> u32 {
    let mut fabric = HashMap::new();
    for claim in input {
        for x in claim.left..claim.left + claim.width {
            for y in claim.top..claim.top + claim.height {
                let counter = fabric.entry((x, y)).or_insert(0);
                *counter += 1;
            }
        }
    }
    fabric.into_iter().filter(|(_, c)| *c > 1).map(|_| 1).sum()
}

fn part2(input: &Vec<Claim>) -> u32 {
    let mut fabric = HashMap::new();
    let mut remaining: HashSet<u32> = HashSet::from_iter(input.iter().map(|c| c.id));
    for claim in input {
        for x in claim.left..claim.left + claim.width {
            for y in claim.top..claim.top + claim.height {
                let prev = fabric.insert((x, y), &claim.id);
                if let Some(prev) = prev {
                    remaining.remove(prev);
                    remaining.remove(&claim.id);
                }
            }
        }
    }

    let remaining = remaining.iter().collect::<Vec<_>>();
    assert_eq!(remaining.len(), 1);
    *remaining[0]
}

#[derive(Debug)]
struct Claim {
    pub id: u32,
    pub left: u32,
    pub top: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
struct ParseClaimError {}

impl FromStr for Claim {
    type Err = ParseClaimError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
        }

        let caps = RE.captures(&s).unwrap();

        Ok(Claim {
            id: caps.get(1).unwrap().as_str().parse().unwrap(),
            left: caps.get(2).unwrap().as_str().parse().unwrap(),
            top: caps.get(3).unwrap().as_str().parse().unwrap(),
            width: caps.get(4).unwrap().as_str().parse().unwrap(),
            height: caps.get(5).unwrap().as_str().parse().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_logic() {
        assert_eq!(
            part1(&vec![
                Claim {
                    id: 1,
                    left: 1,
                    top: 3,
                    width: 4,
                    height: 4
                },
                Claim {
                    id: 2,
                    left: 3,
                    top: 1,
                    width: 4,
                    height: 4
                },
                Claim {
                    id: 3,
                    left: 5,
                    top: 5,
                    width: 2,
                    height: 2
                },
            ]),
            4
        )
    }

    #[test]
    fn test_part2_logic() {
        assert_eq!(
            part2(&vec![
                Claim {
                    id: 1,
                    left: 1,
                    top: 3,
                    width: 4,
                    height: 4
                },
                Claim {
                    id: 2,
                    left: 3,
                    top: 1,
                    width: 4,
                    height: 4
                },
                Claim {
                    id: 3,
                    left: 5,
                    top: 5,
                    width: 2,
                    height: 2
                },
            ]),
            3
        )
    }
}
