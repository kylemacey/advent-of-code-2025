use std::fs;

fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

fn part1(data: &[String]) -> usize {
    let mut sum: usize = 0;

    for bank in data {
        let (mut tens, mut ones) = (0, 0);
        let mut batteries = bank.chars().peekable();
        while let Some(battery) = batteries.next() {
            let joltage: u32 = battery.to_digit(10).unwrap();

            if joltage > tens && batteries.peek().is_some() {
                tens = joltage;
                ones = 0;
            } else if joltage > ones {
                ones = joltage;
            }
        }
        println!("Bank top joltage is {tens}{ones}");
        sum += (tens * 10 + ones) as usize;
    }

    sum
}

fn part2(data: &[String]) -> usize {
    let mut sum: usize = 0;

    for bank in data {
        let mut digits: [u32; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let batteries: Vec<char> = bank.chars().collect();
        let batteries_count = &batteries.len();
        for (n, battery) in batteries.iter().enumerate() {
            let joltage: u32 = battery.to_digit(10).unwrap();
            let remaining_batteries_count = batteries_count - n - 1;

            for i in 0..digits.len() {
                let unchecked_digits_count = 11 - i;
                let has_sufficient_digits = remaining_batteries_count >= unchecked_digits_count;

                if joltage > digits[i] && has_sufficient_digits {
                    digits[i] = joltage;
                    for x in (i + 1)..digits.len() {
                        digits[x] = 0;
                    }
                    break;
                }
            }
        }
        let mut top_joltage: u64 = 0;
        for (i, digit) in digits.iter().enumerate() {
            let exp = 11 - i as u32;
            let base: u64 = 10;
            let digit_calc: u64 = u64::from(*digit) * base.pow(exp);
            top_joltage += digit_calc;
        }
        sum += top_joltage as usize;
    }

    sum
}

fn main() {
    let input = fs::read_to_string("day03/input.txt").expect("Failed to read input file");

    let data = parse_input(&input);

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

    #[test]
    fn test_part1() {
        let data = parse_input(EXAMPLE);
        assert_eq!(part1(&data), 357);
    }

    #[test]
    fn test_part2() {
        let data = parse_input(EXAMPLE);
        assert_eq!(part2(&data), 3121910778619);
    }
}
