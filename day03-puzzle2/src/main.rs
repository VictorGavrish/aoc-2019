use std::collections::HashSet;

static INPUT: &str = include_str!("../resources/input.txt");

fn main() {
    let wires = INPUT.lines().map(Wire::from_str).collect::<Vec<_>>();
    let wire1 = wires[0].clone();
    let wire2 = wires[1].clone();
    let closest = closest_intersection(wire1, wire2).unwrap();
    println!(
        "Closest intersection: {},{}; steps: {}",
        closest.coord.0, closest.coord.1, closest.distance
    );
}

fn closest_intersection(wire1: Wire, wire2: Wire) -> Option<Intersection> {
    wire1
        .intersections(&wire2)
        .into_iter()
        .min_by_key(|w| w.distance)
}

#[derive(Debug, Copy, Clone)]
enum Line {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

#[derive(Debug, Clone)]
struct Wire {
    lines: Vec<Line>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct WirePiece {
    coord: (i32, i32),
    steps: u32,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Intersection {
    coord: (i32, i32),
    distance: u32,
}

impl Wire {
    fn from_str(wire: &str) -> Self {
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
        Self { lines }
    }

    fn intersections(&self, other: &Wire) -> HashSet<Intersection> {
        let mut intersections = HashSet::new();
        let all_self = self.all_wire_pieces();
        let all_other = other.all_wire_pieces();
        for piece_self in all_self {
            for piece_other in all_other.iter() {
                if piece_self.coord == piece_other.coord {
                    let intersection = Intersection {
                        coord: piece_self.coord,
                        distance: piece_self.steps + piece_other.steps,
                    };
                    intersections.insert(intersection);
                }
            }
        }
        intersections
    }

    fn all_wire_pieces(&self) -> Vec<WirePiece> {
        let mut wire_pieces: Vec<WirePiece> = Vec::new();
        let mut last_coord = (0, 0);
        let mut steps = 0;
        for line in self.lines.iter() {
            match line {
                Line::Up(x) => {
                    for i in last_coord.0 + 1..=last_coord.0 + x {
                        steps += 1;
                        let piece = WirePiece {
                            coord: (i, last_coord.1),
                            steps,
                        };
                        wire_pieces.push(piece);
                    }
                    last_coord.0 += x;
                }
                Line::Down(x) => {
                    for i in (last_coord.0 - x..last_coord.0).rev() {
                        steps += 1;
                        let piece = WirePiece {
                            coord: (i, last_coord.1),
                            steps,
                        };
                        wire_pieces.push(piece);
                    }
                    last_coord.0 -= x;
                }
                Line::Right(x) => {
                    for i in last_coord.1 + 1..=last_coord.1 + x {
                        steps += 1;
                        let piece = WirePiece {
                            coord: (last_coord.0, i),
                            steps,
                        };
                        wire_pieces.push(piece);
                    }
                    last_coord.1 += x;
                }
                Line::Left(x) => {
                    for i in (last_coord.1 - x..last_coord.1).rev() {
                        steps += 1;
                        let piece = WirePiece {
                            coord: (last_coord.0, i),
                            steps,
                        };
                        wire_pieces.push(piece);
                    }
                    last_coord.1 -= x;
                }
            }
        }
        wire_pieces
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_pieces() {
        let wire = Wire::from_str("U2,R3,D4,L5");
        let all = wire.all_wire_pieces();
        let expected = vec![
            WirePiece {
                coord: (1, 0),
                steps: 1,
            },
            WirePiece {
                coord: (2, 0),
                steps: 2,
            },
            WirePiece {
                coord: (2, 1),
                steps: 3,
            },
            WirePiece {
                coord: (2, 2),
                steps: 4,
            },
            WirePiece {
                coord: (2, 3),
                steps: 5,
            },
            WirePiece {
                coord: (1, 3),
                steps: 6,
            },
            WirePiece {
                coord: (0, 3),
                steps: 7,
            },
            WirePiece {
                coord: (-1, 3),
                steps: 8,
            },
            WirePiece {
                coord: (-2, 3),
                steps: 9,
            },
            WirePiece {
                coord: (-2, 2),
                steps: 10,
            },
            WirePiece {
                coord: (-2, 1),
                steps: 11,
            },
            WirePiece {
                coord: (-2, 0),
                steps: 12,
            },
            WirePiece {
                coord: (-2, -1),
                steps: 13,
            },
            WirePiece {
                coord: (-2, -2),
                steps: 14,
            },
        ];

        assert_eq!(all, expected);
    }

    #[test]
    fn test1() {
        let wire1 = Wire::from_str("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let wire2 = Wire::from_str("U62,R66,U55,R34,D71,R55,D58,R83");
        let closest = closest_intersection(wire1, wire2).unwrap();
        assert_eq!(closest.distance, 610);
    }

    #[test]
    fn test2() {
        let wire1 = Wire::from_str("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let wire2 = Wire::from_str("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        let closest = closest_intersection(wire1, wire2).unwrap();
        assert_eq!(closest.distance, 410);
    }
}
