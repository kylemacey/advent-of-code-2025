use std::{
    collections::{btree_map::Values, HashMap},
    fs,
};

use itertools::Itertools;

fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

fn add(a: u64, b: u64) -> u64 {
    a + b
}

fn multiply(a: u64, b: u64) -> u64 {
    a * b
}

fn part1(data: &[String]) -> usize {
    let mut map: HashMap<usize, Vec<u64>> = HashMap::new();
    let mut column_outcomes: Vec<u64> = Vec::new();
    for lines in data {
        let row_entries = lines.split_whitespace().collect_vec();
        match row_entries[0].parse::<u64>() {
            Ok(_) => {
                // The line is numeric
                for (i, entry) in row_entries.iter().enumerate() {
                    map.entry(i).or_default().push(entry.parse().unwrap());
                }
            }
            Err(_) => {
                // This is likely the last line of operators
                for (i, entry) in row_entries.iter().enumerate() {
                    let operation = if entry.eq(&"+") { add } else { multiply };
                    let outcome = map
                        .get(&i)
                        .unwrap()
                        .iter()
                        .copied()
                        .reduce(operation)
                        .unwrap();
                    column_outcomes.push(outcome);
                }
            }
        }
    }

    column_outcomes.into_iter().sum::<u64>() as usize
}

fn part2(data: &[String]) -> usize {
    let (operators_line, rows) = data.split_last().unwrap();
    let mut column_outcomes: Vec<u64> = Vec::new();
    let operators = operators_line.split_whitespace().collect_vec();
    let values_map = get_vertical_values(rows.to_vec());

    for (i, entry) in operators.iter().enumerate() {
        let operation = if entry.eq(&"+") { add } else { multiply };
        let outcome = values_map
            .get(&i)
            .unwrap()
            .iter()
            .copied()
            .reduce(operation)
            .unwrap();
        column_outcomes.push(outcome);
    }

    column_outcomes.into_iter().sum::<u64>() as usize
}

fn rows_to_columns(rows: Vec<String>) -> Vec<String> {
    let mut columns: Vec<String> = Vec::new();

    for (r, row) in rows.iter().enumerate() {
        for (i, c) in row.chars().enumerate() {
            match columns.get_mut(i) {
                Some(col) => col.push(c),
                None => {
                    columns.push(" ".repeat(r));
                    columns[i].push(c);
                }
            }
        }
    }

    columns
}

fn get_vertical_values(rows: Vec<String>) -> HashMap<usize, Vec<u64>> {
    let mut problem_inputs: HashMap<usize, Vec<u64>> = HashMap::new();
    let mut i = 0;

    for column in rows_to_columns(rows) {
        if column.trim().is_empty() {
            i += 1;
            continue;
        }
        problem_inputs
            .entry(i)
            .or_default()
            .push(column.trim().parse().unwrap())
    }

    problem_inputs
}

fn main() {
    let input = fs::read_to_string("day06/input.txt").expect("Failed to read input file");

    let data = parse_input(&input);

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";

    #[test]
    fn test_part1() {
        let data = parse_input(EXAMPLE);
        assert_eq!(part1(&data), 4277556);
    }

    #[test]
    fn test_part2() {
        let data = parse_input(EXAMPLE);
        assert_eq!(part2(&data), 3263827);
    }

    #[test]
    fn test_rows_to_columns() {
        let expected: Vec<String> = vec![
            "1  ", "24 ", "356", "   ", "369", "248", "8  ", "   ", " 32", "581", "175", "   ",
            "623", "431", "  4",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let mut input = parse_input(EXAMPLE);
        input.pop();

        assert_eq!(rows_to_columns(input), expected);
    }

    #[test]
    fn test_get_vertical_values() {
        let mut expected: HashMap<usize, Vec<u64>> = HashMap::new();
        expected.insert(0, vec![1, 24, 356]);
        expected.insert(1, vec![369, 248, 8]);
        expected.insert(2, vec![32, 581, 175]);
        expected.insert(3, vec![623, 431, 4]);

        let mut input = parse_input(EXAMPLE);
        input.pop();

        assert_eq!(get_vertical_values(input), expected)
    }
}
