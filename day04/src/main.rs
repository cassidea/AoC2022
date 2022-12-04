use common;

fn to_u32(s: (&str, &str)) -> (u32, u32) {
    (s.0.parse::<u32>().unwrap(), s.1.parse::<u32>().unwrap())
}

fn full_overlap(ll:u32, lu:u32, rl:u32, ru:u32) -> u32 {
    if ll >= rl && lu <= ru {
        return 1;
    }

    if rl >= ll && ru <= lu {
        return 1;
    }
    0
}

fn partly_overlap(ll:u32, lu:u32, rl:u32, ru:u32) -> u32 {
    if ll >= rl && ll <= ru || lu >= rl && lu <= ru{
        return 1;
    }

    if rl >= ll && rl <= lu ||  ru >= ll && ru <= lu {
        return 1;
    }
    0
}

fn check_intervals(line: &str, func : &dyn Fn(u32, u32, u32, u32) -> u32) -> u32 {
    let (left, right) = line.split_once(",").unwrap_or_else(|| ("0-1", "2-3"));
    let (ll, lu) = left.split_once("-").map(|s| to_u32(s)).unwrap();
    let (rl, ru) = right.split_once("-").map(|s| to_u32(s)).unwrap();

    func(ll, lu, rl, ru)
}

fn solve_part_1<T: AsRef<str>>(lines: &[T]) -> u32 {
    lines.iter().map(|line| { check_intervals(line.as_ref(), &full_overlap) }).sum()
}

fn solve_part_2<T: AsRef<str>>(lines: &[T]) -> u32 {
    lines.iter().map(|line| { check_intervals(line.as_ref(), &partly_overlap) }).sum()
}

fn main() {
    let lines = common::get_lines_from_file("day04/input.txt");
    let result1 = solve_part_1(&lines);
    println!("result part 1 is {:?}", result1);
    println!("Correct is 477");
    let result2 = solve_part_2(&lines);
    println!("result part 2 is {:?}", result2);
    println!("Correct is 830");
}

#[test]
fn test_part1() {
    let lines = common::get_lines_from_file("testinput.txt");
    let result1 = solve_part_1(&lines);
    assert_eq!(2, result1);
}

#[test]
fn test_part2() {
    let lines = common::get_lines_from_file("testinput.txt");
    let result2 = solve_part_2(&lines);
    assert_eq!(4, result2);
}