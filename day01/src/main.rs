use std::io;
use std::io::{Read};
use std::fs::File;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut f = File::open("day01.txt")?;

    f.read_to_string(&mut input)?;

    let p1 = part_one(&input)?;
    println!("Part 1: {:?}", p1);

    let p2 = part_two(&input)?;
    println!("Part 2: {:?}", p2);

    Ok(())
}

fn part_one(input: &str) -> io::Result<(i32)> {
    let res = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .sum();

    Ok(res)
}

fn part_two(input: &str) -> io::Result<(i32)> {
    let mut freq = 0;
    let mut seen_freqs: HashSet<i32> = HashSet::new();
    seen_freqs.insert(freq);

    let freq_changes = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .cycle();

    for change in freq_changes {
        freq += change;
        if seen_freqs.contains(&freq) {
            break;
        }
        seen_freqs.insert(freq);
    }

    Ok(freq)
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

    #[test]
    fn test_part_two() {
        let input1 = "+1\n-1";
        let input2 = "+3\n+3\n+4\n-2\n-4";
        let input3 = "-6\n+3\n+8\n+5\n-6";
        let input4 = "+7\n+7\n-2\n-7\n-4";

        assert_eq!(part_two(&input1).unwrap(), 0);
        assert_eq!(part_two(&input2).unwrap(), 10);
        assert_eq!(part_two(&input3).unwrap(), 5);
        assert_eq!(part_two(&input4).unwrap(), 14);
    }
}
