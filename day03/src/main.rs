use common;

fn get_prio(c: char) -> u32 {
    //a = 97, z = 122
    //A = 65, Z = 90
    let x = c as u32;

    if x > 96 {
        x - 96
    } else {
        x - 38
    }
}

// <T: AsRef<str>> allows to hand over &[str] and &[String]
fn solve_part_1<T: AsRef<str>>(lines: &[T]) -> u32 {
    let result = lines.iter().map(|line| {
        let (left, right) = line.as_ref().split_at(line.as_ref().len() / 2);
        left.chars().find(|c| right.contains(*c)).map(|c| get_prio(c)).unwrap()
    }).sum();
    result
}

#[allow(dead_code)] //for Alex
// <T: AsRef<str>> allows to hand over &[str] and &[String]
fn solve_part_2_long<T: AsRef<str>>(lines: &[T]) -> u32 {
    let mut result = 0;
    for i in (0..lines.as_ref().len()).step_by(3) {
        let l1 = lines.get(i).unwrap().as_ref();
        let l2 = lines.get(i + 1).unwrap().as_ref();
        let l3 = lines.get(i + 2).unwrap().as_ref();

        result += l1.chars()
            .find(|c| l2.contains(*c) && l3.contains(*c))
            .map(|c| {
                println!("Got duplicate {}", c);
                get_prio(c)
            }).unwrap();
    }
    result
}

// <T: AsRef<str>> allows to hand over &[str] and &[String]
fn solve_part_2<T: AsRef<str>>(lines: &[T]) -> u32 {
    lines.chunks(3)
        .map(|lll| lll[0].as_ref().chars()
            .find(|c| lll[1].as_ref().contains(*c) && lll[2].as_ref().contains(*c))
            .map(|c| get_prio(c)).unwrap())
        .sum()
}

fn main() {
    let lines = common::get_lines_from_file("day03/input.txt");
    let result1 = solve_part_1(&lines);
    println!("result part 1 is {:?}", result1);
    println!("correct is 8072");
    let result2 = solve_part_2(&lines);
    println!("result part 2 is {:?}", result2);
    println!("correct is 2567");
}

#[test]
fn test_prio() {
    assert_eq!(1, get_prio('a'));
    assert_eq!(26, get_prio('z'));
    assert_eq!(27, get_prio('A'));
    assert_eq!(52, get_prio('Z'));
}

#[test]
fn test_part1() {
    let lines = common::get_lines_from_file("testinput.txt");
    let result1 = solve_part_1(&lines);
    assert_eq!(157, result1);
}

#[test]
fn test_part2() {
    let lines = common::get_lines_from_file("testinput.txt");
    let result1 = solve_part_2(&lines);
    assert_eq!(70, result1);
}