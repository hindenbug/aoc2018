use std::io;
use std::io::{Read};
use std::fs::File;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut f = File::open("day02.txt")?;

    f.read_to_string(&mut input)?;

    let p1 = part_one(&input)?;
    println!("Part 1: {:?}", p1);

    Ok(())
}

fn part_one(input: &str) -> io::Result<(u32)> {
    let (mut twos, mut threes) = (0, 0);

    for line in input.lines() {
       let mut freq = HashMap::new();
        for ch in line.chars() {
            *freq.entry(ch).or_insert(0u32) += 1;
        }

        if freq.values().any(|v| *v == 2u32) {
            twos += 1;
        }

        if freq.values().any(|v| *v == 3u32) {
            threes += 1;
        }
    }

    Ok(twos * threes)
}

fn part_two() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab";
        assert_eq!(part_one(input).unwrap(), 12);
    }

    fn test_part_two() {

    }
}
