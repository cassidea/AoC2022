use std::collections::HashMap;
use common;
use std::time::Instant;

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

// ------- solve using Hashmap -----
fn solve1_with_hashmap<T: AsRef<str>>(lines: &[T]) -> u32 {
    lines.iter().map(|line| {
        let t = &line.as_ref().split_at(line.as_ref().len() / 2);
        get_prio_from_hashmap(&vec!(t.0, t.1))
    }).sum()
}

fn solve2_with_hashmap<T: AsRef<str>>(lines: &[T]) -> u32 {

    lines.chunks(3).map(|lll| {
        get_prio_from_hashmap(lll)
    }).sum()
}

fn get_prio_from_hashmap<T: AsRef<str>>(lll: &[T]) -> u32 {
    let mut map = HashMap::new();
    lll.iter().enumerate()
        .for_each(|(li, l)| l.as_ref().chars().for_each(|c|{map.entry(c).and_modify(|counter| if li*10 > *counter { *counter += 10*li+1 }).or_insert(10*li+1);}));

    let result = *(map.iter().find(|(_, v)| **v % 10 == lll.as_ref().len()).unwrap().0);
    get_prio(result)
}

// ----- solve using naive iteration -----

// <T: AsRef<str>> allows to hand over &[str] and &[String]
fn solve_part_1<T: AsRef<str>>(lines: &[T]) -> u32 {
    let result = lines.iter().map(|line| {
        let (left, right) = line.as_ref().split_at(line.as_ref().len() / 2);
        left.chars().find(|c| right.contains(*c)).map(|c| get_prio(c)).unwrap()
    }).sum();
    result
}

//for Alex
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
    let now = Instant::now();
    let result1 = solve_part_1(&lines);
    println!("result part 1 is {:?} in {}us", result1, (Instant::now() - now).as_micros());

    let now = Instant::now();
    let result1_hm = solve1_with_hashmap(&lines);
    println!("result part 1 using hashmap is {:?} in {}us", result1_hm, (Instant::now() - now).as_micros());

    println!("correct is 8072");

    let now = Instant::now();
    let result2 = solve_part_2(&lines);
    println!("result part 2 is {:?} in {}us", result2, (Instant::now() - now).as_micros());
    let now = Instant::now();
    let result2_l = solve_part_2_long(&lines);
    println!("result part 2 long variant is {:?} in {}us", result2_l, (Instant::now() - now).as_micros());
    let now = Instant::now();
    let result2_hm = solve2_with_hashmap(&lines);
    println!("result part 2 using hashmap is {:?} in {}us", result2_hm, (Instant::now() - now).as_micros());

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

#[test]
fn test_hashmap_1() {
    let lines = common::get_lines_from_file("testinput.txt");
    let result1 = solve1_with_hashmap(&lines);
    assert_eq!(157, result1);
}

#[test]
fn test_hashmap_2() {
    let lines = common::get_lines_from_file("testinput.txt");
    let result1 = solve2_with_hashmap(&lines);
    assert_eq!(70, result1);
}