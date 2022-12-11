extern crate core;

use std::collections::HashSet;
use std::fmt;

#[derive(Debug)]
enum Direction {
    L,
    R,
    U,
    D,
}

impl Direction {
    fn of(s: &str) -> Self {
        match s {
            "L" => { Direction::L }
            "R" => { Direction::R }
            "U" => { Direction::U }
            "D" => { Direction::D }
            _ => { panic!("Unknown direction {:?}", s) }
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn next_pos(&self, dir: &Direction) -> Self {
        match dir {
            Direction::L => Self { x: self.x - 1, ..*self },
            Direction::R => Self { x: self.x + 1, ..*self },
            Direction::U => Self { y: self.y + 1, ..*self },
            Direction::D => Self { y: self.y - 1, ..*self },
        }
    }

    fn follow(&self, head: &Self) -> Self {
        let diff_x = head.x - self.x;
        let diff_y = head.y - self.y;
        let mut new_x = self.x;
        let mut new_y = self.y;


        let sig_y = match diff_y.signum() {
            -1 => -1,
            _ => 1
        };
        let sig_x = match diff_x.signum() {
            -1 => -1,
            _ => 1
        };
        if diff_x.abs() >= 2 {  //moved left or right
            new_x = head.x - sig_x;
            if diff_y.abs() == 1 {
                new_y = head.y;
            } else if diff_y.abs() == 2 {
                new_y = head.y - sig_y;
            }
        }

        if diff_y.abs() >= 2 { // moved up or down
            new_y = head.y - sig_y;
            if diff_x.abs() == 1 {
                new_x = head.x;
            } else if diff_x.abs() == 2 {
                new_x = head.x - sig_x;
            }
        }

        Self { x: new_x, y: new_y }
    }
}

impl fmt::Debug for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}

fn solve_part_1<T: AsRef<str>>(lines: &[T]) -> usize {
    let mut positions = HashSet::new();
    let mut head = Pos { x: 0, y: 0 };
    let mut tail = Pos { x: 0, y: 0 };
    positions.insert(tail.clone());

    lines.iter().map(|l| l.as_ref().split_once(' ').unwrap())
        .map(|(d, s)| (Direction::of(d), s.parse::<i32>().unwrap()))
        .for_each(|(d, steps)|
            {
                for _ in 0..steps {
                    head = head.next_pos(&d);
                    tail = tail.follow(&head);
                    positions.insert(tail.clone());
                }
            });

    positions.len()
}

fn solve_part_2<T: AsRef<str>>(lines: &[T]) -> usize {
    let mut knots = vec![Pos { x: 0, y: 0 }; 10];
    let mut positions = HashSet::new();
    positions.insert(Pos { x: 0, y: 0 });

    lines.iter().map(|l| l.as_ref().split_once(' ').unwrap())
        .map(|(d, s)| (Direction::of(d), s.parse::<i32>().unwrap()))
        .for_each(|(d, steps)|
            {
                for _ in 0..steps {
                    knots[0] = knots.get(0).unwrap().next_pos(&d);
                    for k in 1..knots.len() {
                        knots[k] = knots.get(k).unwrap().follow(knots.get(k - 1).unwrap());
                    }
                    positions.insert(knots.last().unwrap().clone());
                }
            });

    positions.len()
}

fn main() {
    let lines = common::get_lines_from_file("day09/input.txt");
    let result1 = solve_part_1(&lines);
    println!("result part 1 is {:?}", result1);
    println!("Correct is 5513");
    let result2 = solve_part_2(&lines);
    println!("result part 2 is {:?}", result2);
    println!("Correct is 2427");
}

#[test]
fn test_part1() {
    let lines = common::get_lines_from_file("testinput.txt");
    let result = solve_part_1(&lines);
    assert_eq!(13, result);
}

#[test]
fn test_part2() {
    let lines = common::get_lines_from_file("testinput2.txt");
    let result = solve_part_2(&lines);
    assert_eq!(36, result);
}

#[test]
fn test_follow2() {
    let tail = Pos { x: 0, y: 0 };
    assert_eq!(Pos { x: 1, y: 0 }, tail.follow(&Pos { x: 2, y: 0 }));
    assert_eq!(Pos { x: 3, y: 0 }, tail.follow(&Pos { x: 4, y: 0 }));
    assert_eq!(Pos { x: 1, y: 1 }, tail.follow(&Pos { x: 2, y: 2 }));
    assert_eq!(Pos { x: 3, y: 3 }, tail.follow(&Pos { x: 4, y: 4 }));
    assert_eq!(Pos { x: 0, y: 1 }, tail.follow(&Pos { x: 0, y: 2 }));
    assert_eq!(Pos { x: 0, y: 3 }, tail.follow(&Pos { x: 0, y: 4 }));
    assert_eq!(Pos { x: 0, y: 0 }, tail.follow(&Pos { x: 0, y: 0 }));
    assert_eq!(Pos { x: 0, y: 0 }, tail.follow(&Pos { x: 1, y: 0 }));
    assert_eq!(Pos { x: 0, y: 0 }, tail.follow(&Pos { x: 0, y: 1 }));
}