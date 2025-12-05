use fancy_regex::Regex;
use std::fs;
use std::ops::RangeInclusive;

fn parse_input(input: &str) -> Vec<String> {
    input
        .lines()
        .next()
        .expect("No lines provided in input")
        .split(',')
        .map(|s| s.to_string())
        .collect()
}

fn part1(data: &[String]) -> usize {
    // TODO: Implement part 1
    let mut sum: usize = 0;
    for range_s in data {
        println!("Processing string {}", range_s);
        let mut range_list = range_s.split('-');
        let range: RangeInclusive<u64> = RangeInclusive::new(
            range_list
                .next()
                .expect("invalid range {range_s}")
                .parse()
                .unwrap(),
            range_list
                .next()
                .expect("invalid range {range_s}")
                .parse()
                .unwrap(),
        );

        for num in range {
            let num_s: String = num.to_string();
            let len = num_s.len();
            if len % 2 > 0 {
                continue;
            }
            let (half_1, half_2) = num_s.split_at(len / 2);
            if half_1 == half_2 {
                sum += num as usize;
            }
        }
    }

    sum
}

fn part2(data: &[String]) -> usize {
    // TODO: Implement part 2
    let re = Regex::new(r"\A(\d+)\1+\z").unwrap();
    let mut sum: usize = 0;
    for range_s in data {
        println!("Processing string {}", range_s);
        let mut range_list = range_s.split('-');
        let range: RangeInclusive<u64> = RangeInclusive::new(
            range_list
                .next()
                .expect("invalid range {range_s}")
                .parse()
                .unwrap(),
            range_list
                .next()
                .expect("invalid range {range_s}")
                .parse()
                .unwrap(),
        );

        for num in range {
            let num_s: String = num.to_string();

            if re.is_match(&num_s).unwrap() {
                sum += num as usize;
            }
        }
    }

    sum
}

fn main() {
    let input = fs::read_to_string("day02/input.txt").expect("Failed to read input file");

    let data = parse_input(&input);

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124\
";

    #[test]
    fn test_part1() {
        let data = parse_input(EXAMPLE);
        assert_eq!(part1(&data), 1227775554);
    }

    #[test]
    fn test_part2() {
        let data = parse_input(EXAMPLE);
        assert_eq!(part2(&data), 4174379265);
    }
}
