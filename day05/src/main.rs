use std::{fs, ops::RangeInclusive};

fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

fn part1(data: &[String]) -> usize {
    let mut sum = 0;
    let mut fresh_ranges: Vec<RangeInclusive<u64>> = Vec::new();
    let mut line_iter = data.iter();

    while let Some(line) = line_iter.next() {
        if line.is_empty() {
            break;
        }
        let mut bounds = line.split('-');
        let floor: u64 = bounds
            .next()
            .unwrap()
            .parse()
            .expect("invalid range {bounds}");
        let ceil: u64 = bounds
            .next()
            .unwrap()
            .parse()
            .expect("invalid range {bounds}");

        fresh_ranges.push(RangeInclusive::new(floor, ceil))
    }

    while let Some(line) = line_iter.next() {
        let ingredient: u64 = line.parse().expect("invalid ingredient id {line}");
        for fresh_range in fresh_ranges.iter().by_ref() {
            if fresh_range.contains(&ingredient) {
                sum += 1;
                break;
            }
        }
    }

    sum
}

fn part2(data: &[String]) -> usize {
    let mut fresh_ranges: Vec<RangeInclusive<u64>> = Vec::new();

    for line in data {
        if line.is_empty() {
            break;
        }
        let mut bounds = line.split('-');
        let floor: u64 = bounds
            .next()
            .unwrap()
            .parse()
            .expect("invalid range {bounds}");
        let ceil: u64 = bounds
            .next()
            .unwrap()
            .parse()
            .expect("invalid range {bounds}");

        marry_ranges(floor, ceil, &mut fresh_ranges);
    }

    let overlapping_ranges: Vec<RangeInclusive<u64>> = detect_overlapping_ranges(&fresh_ranges);
    if !overlapping_ranges.is_empty() {
        panic!("Overlapping ranges! {:?}", overlapping_ranges);
    }

    fresh_ranges
        .iter()
        .map(|range| (range.end() - range.start() + 1) as usize)
        .sum()
}

fn marry_ranges(proposed_lower: u64, proposed_upper: u64, ranges: &mut Vec<RangeInclusive<u64>>) {
    let (mut new_lower_bound, mut new_upper_bound): (u64, u64) = (proposed_lower, proposed_upper);
    ranges.sort_by(|a, b| a.start().cmp(b.start()));
    for i in (0..ranges.len()).rev() {
        let range = &ranges[i];
        let (range_lower, range_upper) = (*range.start(), *range.end());

        // The proposed range is useless, return early, no-op
        if range_lower <= new_lower_bound && range_upper >= new_upper_bound {
            return;
        }

        // The existing range is useless, delete it
        if range_lower >= new_lower_bound && range_upper <= new_upper_bound {
            ranges.remove(i);
            continue;
        }

        // Ensure no conflict with lower
        if new_lower_bound <= range_upper && new_upper_bound > range_upper {
            new_lower_bound = range_upper + 1;
        }

        // Ensure no conflict with upper
        if new_upper_bound >= range_lower && new_lower_bound < range_lower {
            new_upper_bound = range_lower - 1;
        }
    }

    // If we made it this far, we append a new range
    ranges.push(new_lower_bound..=new_upper_bound);
}

fn detect_overlapping_ranges(ranges: &Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    let mut overlapping_ranges: Vec<RangeInclusive<u64>> = Vec::new();

    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by(|a, b| a.start().cmp(b.start()));

    let mut iter = sorted_ranges.iter().peekable();
    while let Some(range) = iter.next() {
        match iter.peek() {
            Some(next_range) => {
                if range.end() >= next_range.start() {
                    overlapping_ranges.push(range.clone());
                    overlapping_ranges.push((*next_range).clone());
                }
            }
            None => {}
        }
    }

    overlapping_ranges
}

fn main() {
    let input = fs::read_to_string("day05/input.txt").expect("Failed to read input file");

    let data = parse_input(&input);

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part1() {
        let data = parse_input(EXAMPLE);
        assert_eq!(part1(&data), 3);
    }

    #[test]
    fn test_part2() {
        let data = parse_input(EXAMPLE);
        assert_eq!(part2(&data), 14);
    }

    #[test]
    fn test_marry_ranges_adjusts_upper() {
        let mut ranges: Vec<RangeInclusive<u64>> = vec![(4..=8)];
        let expected_ranges: Vec<RangeInclusive<u64>> = vec![(4..=8), (3..=3)];

        marry_ranges(3, 7, &mut ranges);
        assert_eq!(ranges, expected_ranges);
    }

    #[test]
    fn test_marry_ranges_adjusts_lower() {
        let mut ranges: Vec<RangeInclusive<u64>> = vec![(4..=8)];
        let expected_ranges: Vec<RangeInclusive<u64>> = vec![(4..=8), (9..=10)];

        marry_ranges(6, 10, &mut ranges);
        assert_eq!(ranges, expected_ranges);
    }

    #[test]
    fn test_marry_ranges_adjusts_from_both_sides() {
        let mut ranges: Vec<RangeInclusive<u64>> = vec![(4..=8), (11..=15)];
        let expected_ranges: Vec<RangeInclusive<u64>> = vec![(4..=8), (11..=15), (9..=10)];

        marry_ranges(6, 13, &mut ranges);
        assert_eq!(ranges, expected_ranges);
    }

    #[test]
    fn test_marry_ranges_does_not_adjust_no_conflict() {
        let mut ranges: Vec<RangeInclusive<u64>> = vec![(4..=8)];
        let expected_ranges: Vec<RangeInclusive<u64>> = vec![(4..=8), (10..=12)];

        marry_ranges(10, 12, &mut ranges);
        assert_eq!(ranges, expected_ranges);
    }

    #[test]
    fn test_marry_ranges_removes_existing_redundant_range() {
        let mut ranges: Vec<RangeInclusive<u64>> = vec![(4..=8)];
        let expected_ranges: Vec<RangeInclusive<u64>> = vec![(2..=8)];

        marry_ranges(2, 8, &mut ranges);
        assert_eq!(ranges, expected_ranges);
    }

    #[test]
    fn test_marry_ranges_does_not_add_redundant_range() {
        let mut ranges: Vec<RangeInclusive<u64>> = vec![(4..=8)];
        let expected_ranges: Vec<RangeInclusive<u64>> = vec![(4..=8)];

        marry_ranges(4, 6, &mut ranges);
        assert_eq!(ranges, expected_ranges);
    }

    #[test]
    fn test_marry_ranges_does_not_add_redundant_range_complex() {
        let mut ranges: Vec<RangeInclusive<u64>> = vec![(4..=8), (9..=15)];
        let expected_ranges: Vec<RangeInclusive<u64>> = vec![(4..=8), (9..=15)];

        marry_ranges(5, 12, &mut ranges);
        assert_eq!(ranges, expected_ranges);
    }

    #[test]
    fn test_marry_ranges_fix_bug() {
        let mut ranges: Vec<RangeInclusive<u64>> = vec![(272731198580506..=273246390457576)];
        let expected_ranges: Vec<RangeInclusive<u64>> = vec![(272731198580506..=273570669932717)];

        marry_ranges(272731198580506, 273570669932717, &mut ranges);
        assert_eq!(ranges, expected_ranges);
    }

    #[test]
    fn test_marry_ranges_fix_bug2() {
        let mut ranges: Vec<RangeInclusive<u64>> = vec![(272731198580506..=273570669932717)];
        let expected_ranges: Vec<RangeInclusive<u64>> = vec![(272731198580506..=273570669932717)];

        marry_ranges(272731198580506, 273246390457576, &mut ranges);
        assert_eq!(ranges, expected_ranges);
    }
}
