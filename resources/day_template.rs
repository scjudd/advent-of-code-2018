use std::io::{stdin, BufRead};

pub fn solve() {
    let stdin = stdin();
    let input = stdin
        .lock()
        .lines()
        .map(|line| line.expect("error reading line from input"))
        .collect::<Vec<_>>();

    //println!("Part 1: {}", part1(&input));
    //println!("Part 2: {}", part2(&input));

}

/*
fn part1(...) -> ... {
}
*/

/*
fn part2(...) -> ... {
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_logic() {
        unimplemented!();
    }

    #[test]
    fn test_part2_logic() {
        unimplemented!();
    }
}
