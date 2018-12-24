use std::io;
use std::io::{Read};
use std::fs::File;

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut f = File::open("day01.txt")?;

    f.read_to_string(&mut input)?;

    let p1 = part_one(&input)?;
    println!("Part 1: {}", p1);

    Ok(())
}

fn part_one(input: &str) -> io::Result<(i32)> {
    let res = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap()).sum();

    Ok(res)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let input1 = "+1\n+1\n+1";
    let input2 = "+1\n+1\n-2";
    let input3 = "-1\n-2\n-3";

    assert_eq!(part_one(&input1).unwrap(), 3);
    assert_eq!(part_one(&input2).unwrap(), 0);
    assert_eq!(part_one(&input3).unwrap(), -6);
  }
}
