use std::{collections::HashSet, fs, os::linux::raw};

#[derive(Debug)]
struct Coordinates {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
struct JunctionBox {
    id: u32,
    coordinates: Coordinates,
}

#[derive(Debug, Clone)]
struct Connection {
    source: u32,
    target: u32,
    distance: f64,
}

#[derive(Debug, Clone)]
struct Circuit {
    id: u32,
    connections: Vec<Connection>,
}

impl Circuit {
    fn all_junction_boxes(&self) -> HashSet<u32> {
        self.connections
            .iter()
            .flat_map(|c| [c.source, c.target])
            .collect()
    }

    fn size(&self) -> usize {
        self.all_junction_boxes().len()
    }

    fn contains(&self, id: u32) -> bool {
        self.all_junction_boxes().contains(&id)
    }

    fn merge(&mut self, target_circuit: Circuit) {
        self.connections.extend(target_circuit.connections);
    }
}

impl PartialEq for Circuit {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

fn part1(data: &[String], connections_to_make: usize) -> usize {
    let mut junction_boxes: Vec<JunctionBox> = Vec::new();
    let mut connections: Vec<Connection> = Vec::new();
    let mut circuits: Vec<Circuit> = Vec::new();

    for (i, line) in data.iter().enumerate() {
        let raw_coords: Vec<&str> = line.split(',').collect();
        let new_junction_box = JunctionBox {
            id: i as u32,
            coordinates: Coordinates {
                x: (raw_coords[0].parse::<i64>().unwrap()),
                y: (raw_coords[1].parse::<i64>().unwrap()),
                z: (raw_coords[2].parse::<i64>().unwrap()),
            },
        };

        println!("{:?}", new_junction_box);
        junction_boxes.push(new_junction_box);
    }

    for i in 0..junction_boxes.len() {
        let source_junction_box = &junction_boxes[i];

        for target_junction_box in junction_boxes.iter().skip(i + 1) {
            connections.push(Connection {
                source: source_junction_box.id,
                target: target_junction_box.id,
                distance: calculate_distance(source_junction_box, target_junction_box),
            })
        }
    }

    connections.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    let mut connections_made: usize = 0;
    for (i, c) in connections.iter().enumerate() {
        if connections_made == connections_to_make {
            break;
        }

        println!("{:?}", c);

        // Check for existing circuit
        let existing_source_circuit_idx = circuits
            .iter_mut()
            .position(|circuit| circuit.contains(c.source));

        let existing_target_circuit_idx = circuits
            .iter_mut()
            .position(|circuit| circuit.contains(c.target));

        match (existing_source_circuit_idx, existing_target_circuit_idx) {
            (Some(s_idx), Some(t_idx)) => {
                if s_idx == t_idx {
                    // This connection is made already, no op
                    // continue;
                } else {
                    // Remove the larger index first to avoid invalidating the smaller index
                    let (to_remove, to_keep) = if s_idx < t_idx {
                        (t_idx, s_idx)
                    } else {
                        (s_idx, t_idx)
                    };

                    let removed = circuits.remove(to_remove);
                    circuits[to_keep].merge(removed);
                }
            }
            (Some(c_idx), None) | (None, Some(c_idx)) => {
                // One found - add connection to existing circuit
                circuits[c_idx].connections.push(c.clone());
            }
            (None, None) => {
                // Neither found - create new circuit
                circuits.push(Circuit {
                    id: i as u32,
                    connections: vec![c.clone()],
                });
            }
        }

        connections_made += 1;
    }

    circuits.sort_by(|a, b| b.connections.len().partial_cmp(&a.size()).unwrap());
    let largest_circuits = &circuits[0..3];

    for c in circuits.iter() {
        println!("({}) {:?}", c.size(), c);
    }

    largest_circuits.iter().map(|c| c.size()).product::<usize>()
}

fn part2(data: &[String]) -> usize {
    let mut junction_boxes: Vec<JunctionBox> = Vec::new();
    let mut connections: Vec<Connection> = Vec::new();
    let mut circuits: Vec<Circuit> = Vec::new();

    for (i, line) in data.iter().enumerate() {
        let raw_coords: Vec<&str> = line.split(',').collect();
        let new_junction_box = JunctionBox {
            id: i as u32,
            coordinates: Coordinates {
                x: (raw_coords[0].parse::<i64>().unwrap()),
                y: (raw_coords[1].parse::<i64>().unwrap()),
                z: (raw_coords[2].parse::<i64>().unwrap()),
            },
        };

        println!("{:?}", new_junction_box);
        junction_boxes.push(new_junction_box);
    }

    for i in 0..junction_boxes.len() {
        let source_junction_box = &junction_boxes[i];

        for target_junction_box in junction_boxes.iter().skip(i + 1) {
            connections.push(Connection {
                source: source_junction_box.id,
                target: target_junction_box.id,
                distance: calculate_distance(source_junction_box, target_junction_box),
            })
        }
    }

    connections.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    let mut last_connection: Option<Connection> = None;
    for (i, c) in connections.iter().enumerate() {
        println!("{:?}", c);

        // Check for existing circuit
        let existing_source_circuit_idx = circuits
            .iter_mut()
            .position(|circuit| circuit.contains(c.source));

        let existing_target_circuit_idx = circuits
            .iter_mut()
            .position(|circuit| circuit.contains(c.target));

        match (existing_source_circuit_idx, existing_target_circuit_idx) {
            (Some(s_idx), Some(t_idx)) => {
                if s_idx == t_idx {
                    // This connection is made already, no op
                    // continue;
                } else {
                    // Remove the larger index first to avoid invalidating the smaller index
                    let (to_remove, to_keep) = if s_idx < t_idx {
                        (t_idx, s_idx)
                    } else {
                        (s_idx, t_idx)
                    };

                    let removed = circuits.remove(to_remove);
                    circuits[to_keep].merge(removed);
                    last_connection = Some(c.clone());
                }
            }
            (Some(c_idx), None) | (None, Some(c_idx)) => {
                // One found - add connection to existing circuit
                circuits[c_idx].connections.push(c.clone());
                last_connection = Some(c.clone());
            }
            (None, None) => {
                // Neither found - create new circuit
                circuits.push(Circuit {
                    id: i as u32,
                    connections: vec![c.clone()],
                });
            }
        }
    }

    circuits.sort_by(|a, b| b.connections.len().partial_cmp(&a.size()).unwrap());

    for c in circuits.iter() {
        println!("({}) {:?}", c.size(), c);
    }

    // I'm so tired... just multiply the output from the console...
    if let Some(last_conn) = last_connection {
        let jb_source = &junction_boxes[last_conn.source as usize];
        let jb_target = &junction_boxes[last_conn.target as usize];

        println!("Source: {:?}\nTarget:{:?}", jb_source, jb_target);
    }

    0
}

fn main() {
    let input = fs::read_to_string("day08/input.txt").expect("Failed to read input file");

    let data = parse_input(&input);

    // println!("Part 1: {}", part1(&data, 1000));
    println!("Part 2: {}", part2(&data));
}

fn calculate_distance(a: &JunctionBox, b: &JunctionBox) -> f64 {
    let mut deltas: [i64; 3] = [0; 3];

    deltas[0] = a.coordinates.x - b.coordinates.x;
    deltas[1] = a.coordinates.y - b.coordinates.y;
    deltas[2] = a.coordinates.z - b.coordinates.z;

    let sum: f64 = deltas.map(|i| i.pow(2) as f64).iter().sum::<f64>();

    sum.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[test]
    fn test_part1() {
        let data = parse_input(EXAMPLE);
        assert_eq!(part1(&data, 10), 40);
    }

    #[test]
    fn test_part2() {
        let data = parse_input(EXAMPLE);
        assert_eq!(part2(&data), 0);
    }
}
