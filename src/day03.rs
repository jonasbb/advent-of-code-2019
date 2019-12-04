use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::repeat;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Route {
    directions: Vec<(Direction, u32)>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<Route> {
    input
        .split('\n')
        .map(|x| Route {
            directions: x
                .split(',')
                .map(|d| {
                    let direction = match d.as_bytes()[0] {
                        b'U' => Direction::Up,
                        b'R' => Direction::Right,
                        b'D' => Direction::Down,
                        b'L' => Direction::Left,
                        _ => unreachable!("Unknown direction"),
                    };
                    let repeat = d[1..].parse().unwrap();
                    (direction, repeat)
                })
                .collect(),
        })
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &[Route]) -> usize {
    let mut points_per_cable = input.iter().map(|route| {
        let mut point: (isize, isize) = (0, 0);
        route
            .directions
            .iter()
            .flat_map(|(direction, repeat_count)| repeat(direction).take(*repeat_count as usize))
            .map(|direction| {
                match direction {
                    Direction::Up => point.1 += 1,
                    Direction::Right => point.0 += 1,
                    Direction::Down => point.1 -= 1,
                    Direction::Left => point.0 -= 1,
                };
                point
            })
            .collect::<HashSet<_>>()
    });

    let cable1 = points_per_cable.next().unwrap();
    let cable2 = points_per_cable.next().unwrap();
    let overlapping_points = cable1.intersection(&cable2);

    let mut shortest_manhatten_distance = usize::max_value();
    for point in overlapping_points {
        let cur_dist = point.0.abs() + point.1.abs();
        shortest_manhatten_distance = shortest_manhatten_distance.min(cur_dist as usize);
    }
    shortest_manhatten_distance
}

#[aoc(day3, part2)]
fn part2(input: &[Route]) -> usize {
    let mut points_per_cable = input.iter().map(|route| {
        let mut point: (isize, isize) = (0, 0);
        route
            .directions
            .iter()
            .flat_map(|(direction, repeat_count)| repeat(direction).take(*repeat_count as usize))
            .enumerate()
            .map(|(step_count, direction)| {
                match direction {
                    Direction::Up => point.1 += 1,
                    Direction::Right => point.0 += 1,
                    Direction::Down => point.1 -= 1,
                    Direction::Left => point.0 -= 1,
                };
                (point, step_count+1)
            })
            .collect::<HashMap<_, _>>()
    });

    let mut cable1 = points_per_cable.next().unwrap();
    let mut cable2 = points_per_cable.next().unwrap();
    if cable2.len() > cable1.len() {
        std::mem::swap(&mut cable1, &mut cable2);
    }

    let mut minimal_steps = usize::max_value();
    for (point, steps) in cable1 {
        if let Some(steps2) = cable2.get(&point) {
        let cur_steps = steps + steps2;
        minimal_steps = minimal_steps.min(cur_steps);
        }
    }
    minimal_steps
}

#[test]
fn test_part1_cables_0() {
    let input_str = r#"R8,U5,L5,D3
U7,R6,D4,L4"#;
    let input = input_generator(input_str);
    let output = part1(&input);
    assert_eq!(6, output);
}

#[test]
fn test_part1_cables_1() {
    let input_str = r#"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"#;
    let input = input_generator(input_str);
    let output = part1(&input);
    assert_eq!(159, output);
}

#[test]
fn test_part1_cables_2() {
    let input_str = r#"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"#;
    let input = input_generator(input_str);
    let output = part1(&input);
    assert_eq!(135, output);
}

#[test]
fn test_part2_cables_0() {
    let input_str = r#"R8,U5,L5,D3
U7,R6,D4,L4"#;
    let input = input_generator(input_str);
    let output = part2(&input);
    assert_eq!(30, output);
}

#[test]
fn test_part2_cables_1() {
    let input_str = r#"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"#;
    let input = input_generator(input_str);
    let output = part2(&input);
    assert_eq!(610, output);
}

#[test]
fn test_part2_cables_2() {
    let input_str = r#"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"#;
    let input = input_generator(input_str);
    let output = part2(&input);
    assert_eq!(410, output);
}
