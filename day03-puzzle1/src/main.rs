use std::collections::HashSet;

static INPUT: &str = include_str!("../resources/input.txt");

fn main() {
    let wires = INPUT.lines().map(parse_wire).collect::<Vec<_>>();
    let wire1 = wires[0].clone();
    let wire2 = wires[1].clone();
    let closest = closest_intersection(wire1, wire2);
    println!(
        "closest: {},{}; distance: {}",
        closest.0,
        closest.1,
        closest.0.abs() + closest.1.abs()
    )
}

fn closest_intersection(wire1: Wire, wire2: Wire) -> (i32, i32) {
    let intersections = wire1.intersections(&wire2);
    intersections
        .into_iter()
        .min_by(|one, two| (one.0.abs() + one.1.abs()).cmp(&(two.0.abs() + two.1.abs())))
        .unwrap()
}

fn parse_wire(wire: &str) -> Wire {
    let lines = wire
        .split(',')
        .map(|l| match l.chars().next().expect("empty line") {
            'U' => Line::Up(l.get(1..).unwrap().trim().parse().unwrap()),
            'D' => Line::Down(l.get(1..).unwrap().trim().parse().unwrap()),
            'L' => Line::Left(l.get(1..).unwrap().trim().parse().unwrap()),
            'R' => Line::Right(l.get(1..).unwrap().trim().parse().unwrap()),
            x => panic!("unexpected direction: {}", x),
        })
        .collect();
    Wire { lines }
}

#[derive(Debug, Clone)]
struct Wire {
    lines: Vec<Line>,
}

impl Wire {
    fn intersections(&self, other: &Wire) -> Vec<(i32, i32)> {
        let mut all = self.all_occupied_coordinates();
        let all_other = other
            .all_occupied_coordinates()
            .into_iter()
            .collect::<HashSet<_>>();
        all.retain(|coord| all_other.contains(coord));
        all.shrink_to_fit();
        all
    }

    fn all_occupied_coordinates(&self) -> Vec<(i32, i32)> {
        let mut coords: Vec<(i32, i32)> = Vec::new();
        let mut last_coord = (0, 0);
        for line in self.lines.iter() {
            match line {
                Line::Up(x) => {
                    for i in last_coord.0 + 1..=last_coord.0 + x {
                        let coord = (i, last_coord.1);
                        coords.push(coord);
                    }
                    last_coord.0 += x;
                }
                Line::Down(x) => {
                    for i in (last_coord.0 - x..last_coord.0).rev() {
                        let coord = (i, last_coord.1);
                        coords.push(coord);
                    }
                    last_coord.0 -= x;
                }
                Line::Right(x) => {
                    for i in last_coord.1 + 1..=last_coord.1 + x {
                        let coord = (last_coord.0, i);
                        coords.push(coord);
                    }
                    last_coord.1 += x;
                }
                Line::Left(x) => {
                    for i in (last_coord.1 - x..last_coord.1).rev() {
                        let coord = (last_coord.0, i);
                        coords.push(coord);
                    }
                    last_coord.1 -= x;
                }
            }
        }
        coords
    }
}

#[derive(Debug, Copy, Clone)]
enum Line {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_occupied() {
        let wire = "U2,R3,D4,L5";
        let wire = parse_wire(wire);
        let all = wire.all_occupied_coordinates();

        assert_eq!(
            all,
            vec![
                (1, 0),
                (2, 0),
                (2, 1),
                (2, 2),
                (2, 3),
                (1, 3),
                (0, 3),
                (-1, 3),
                (-2, 3),
                (-2, 2),
                (-2, 1),
                (-2, 0),
                (-2, -1),
                (-2, -2)
            ]
        )
    }

    #[test]
    fn test1() {
        let wire1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let wire2 = "U62,R66,U55,R34,D71,R55,D58,R83";
        let wire1 = parse_wire(wire1);
        let wire2 = parse_wire(wire2);
        let closest = closest_intersection(wire1, wire2);
        assert_eq!(closest.0.abs() + closest.1.abs(), 159);
    }

    #[test]
    fn test2() {
        let wire1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
        let wire2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let wire1 = parse_wire(wire1);
        let wire2 = parse_wire(wire2);
        let closest = closest_intersection(wire1, wire2);
        assert_eq!(closest.0.abs() + closest.1.abs(), 135);
    }
}
