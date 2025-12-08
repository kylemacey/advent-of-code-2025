use std::{collections::HashMap, fs, iter::Sum};

fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

fn part1(data: &[String]) -> usize {
    let mut sum: usize = 0;
    let mut beams: Vec<usize> = Vec::new();

    for line in data {
        let mut next_beams: Vec<usize> = Vec::new();
        for (i, c) in line.chars().enumerate() {
            if c == 'S' {
                next_beams = vec![i];
                continue;
            }

            if c == '^' && beams.contains(&i) {
                next_beams.push(i - 1);
                next_beams.push(i + 1);
                sum += 1;
                continue;
            }

            if beams.contains(&i) {
                next_beams.push(i);
            }
        }
        beams = next_beams;
    }

    sum
}

fn part2(data: &[String]) -> usize {
    let mut beams: HashMap<usize, usize> = HashMap::new();

    for (n, line) in data.iter().enumerate() {
        let line_length = line.chars().count();
        if n % 2 == 1 {
            let mut line_render = String::new();

            for i in 0..line_length {
                if beams.contains_key(&i) {
                    line_render.push('|');
                } else {
                    line_render.push('.');
                }
            }

            println!(
                "{line_render} {} : {:?}",
                beams.values().sum::<usize>(),
                beams
            );
            continue;
        }

        let mut next_beams: HashMap<usize, usize> = HashMap::new();
        for (i, c) in line.chars().enumerate() {
            if c == 'S' {
                next_beams.insert(i, 1);
                continue;
            }

            if c == '^' && beams.contains_key(&i) {
                if i != 0 {
                    *next_beams.entry(i - 1).or_insert(0) += beams[&i];
                }

                if i != line_length {
                    *next_beams.entry(i + 1).or_insert(0) += beams[&i];
                }

                continue;
            }

            if beams.contains_key(&i) {
                *next_beams.entry(i).or_insert(0) += beams[&i];
            }
        }
        beams = next_beams;
    }

    beams.values().sum()
}

fn count_beams_for(i: &usize, beams: &Vec<usize>) -> usize {
    beams.iter().filter(|&x| *x == *i).count()
}

fn main() {
    let input = fs::read_to_string("day07/input.txt").expect("Failed to read input file");

    let data = parse_input(&input);

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn test_part1() {
        let data = parse_input(EXAMPLE);
        assert_eq!(part1(&data), 21);
    }

    #[test]
    fn test_part2() {
        let data = parse_input(EXAMPLE);
        assert_eq!(part2(&data), 40);
    }
}
