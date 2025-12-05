use std::fs;

fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

fn part1(data: &[String]) -> usize {
    let mut zeroes = 0;
    let mut position = 50;
    for line in data {
        let (dir, count) = line.split_at(1);
        let mut count_i: i32 = count.parse().unwrap();
        if dir == "L" {
            count_i = 100 - count_i;
        }

        position = (position + count_i) % 100;

        if position == 0 {
            zeroes += 1;
        }
    }

    zeroes.try_into().unwrap()
}

fn part2(data: &[String]) -> usize {
    let mut zeroes = 0;
    let mut position = 50;
    for line in data {
        let (dir, count) = line.split_at(1);
        let mut count_i: i32 = count.parse().unwrap();
        if dir == "L" {
            if count_i >= position {
                zeroes += (count_i - position) / 100;
                if position != 0 {
                    zeroes += 1
                }
            }
            count_i = 100 - (count_i % 100);
        } else if (position + count_i) >= 100 {
            zeroes += (position + count_i) / 100;
        }

        position = (position + count_i) % 100;
    }

    zeroes.try_into().unwrap()
}

fn main() {
    let input = fs::read_to_string("day01/input.txt").expect("Failed to read input file");

    let data = parse_input(&input);

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
L200
R200
";

    #[test]
    fn test_part1() {
        let data = parse_input(EXAMPLE);
        assert_eq!(part1(&data), 3);
    }

    #[test]
    fn test_part2() {
        let data = parse_input(EXAMPLE);
        assert_eq!(part2(&data), 10);
    }
}
