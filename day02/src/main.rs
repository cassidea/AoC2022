use common;

fn shape_value(shape: &str) -> i32 {
    match shape {
        "X" => 1, //rock
        "Y" => 2, //paper
        "Z" => 3, //scissor
        _ => panic!()
    }
}

fn get_result(input: (&str, &str)) -> i32 {
    match input.0 {
        "A" => { //rock
            match input.1 {
                "X" => 3 + shape_value(input.1),
                "Y" => 6 + shape_value(input.1),
                "Z" => 0 + shape_value(input.1),
                _ => panic!()
            }
        }
        "B" => { //paper
            match input.1 {
                "X" => 0 + shape_value(input.1),
                "Y" => 3 + shape_value(input.1),
                "Z" => 6 + shape_value(input.1),
                _ => panic!()
            }
        }
        "C" => { //scissor
            match input.1 {
                "X" => 6 + shape_value(input.1),
                "Y" => 0 + shape_value(input.1),
                "Z" => 3 + shape_value(input.1),
                _ => panic!()
            }
        }
        _ => panic!()
    }
}

fn get_mapping_for_part2(input: (&str, &str)) -> String {
    match input.0 {
        "A" => { //rock
            match input.1 {
                "X" => String::from("Z"),
                "Y" => String::from("X"),
                "Z" => String::from("Y"),
                _ => panic!()
            }
        }
        "B" => { //paper
            match input.1 {
                "X" => String::from("X"),
                "Y" => String::from("Y"),
                "Z" => String::from("Z"),
                _ => panic!()
            }
        }
        "C" => { //scissor
            match input.1 {
                "X" => String::from("Y"),
                "Y" => String::from("Z"),
                "Z" => String::from("X"),
                _ => panic!()
            }
        }
        _ => panic!()
    }
}

#[allow(dead_code)] //for Alex
// <T: AsRef<str>> allows to hand over &[str] and &[String]
fn solve_part_1_long<T: AsRef<str>>(lines: &[T]) -> i32 {
    let mut inputs = vec![];
    for line in lines {
        let input = line.as_ref().split_once(" ").unwrap();
        inputs.push(input);
    }

    println!("inputs: {:?}", inputs.len());
    inputs.iter().map(|(x, y)| get_result((x, y))).sum()
}

#[allow(dead_code)] //for Alex
// <T: AsRef<str>> allows to hand over &[str] and &[String]
fn solve_part_2_long<T: AsRef<str>>(lines: &[T]) -> i32 {
    let mut inputs = vec![];
    for line in lines {
        let input = line.as_ref().split_once(" ").unwrap();
        inputs.push(input);
    }

    println!("inputs: {:?}", inputs.len());
    inputs.iter()
        .map(|(x, y)| (x, get_mapping_for_part2((x, y))))
        .map(|(x, y)| get_result((x, y.as_str())))
        .sum()
}

// <T: AsRef<str>> allows to hand over &[str] and &[String]
fn solve_part_1<T: AsRef<str>>(lines: &[T]) -> i32 {
    lines.iter()
        .map(|line| line.as_ref().split_once(" ").unwrap())
        .map(|(x, y)| get_result((x, y)))
        .sum()
}

// <T: AsRef<str>> allows to hand over &[str] and &[String]
fn solve_part_2<T: AsRef<str>>(lines: &[T]) -> i32 {
    lines.iter().map(|line| line.as_ref().split_once(" ").unwrap())
        .map(|(x, y)| (x, get_mapping_for_part2((x, y))))
        .map(|(x, y)| get_result((x, y.as_str())))
        .sum()
}

fn main() {
    let lines = common::get_lines_from_file("day02/input.txt");
    let result1 = solve_part_1(&lines);
    println!("result part1 is: {}", result1);
    println!("correct is 14069");
    let result2 = solve_part_2(&lines);
    println!("result part2 is: {}", result2);
    println!("correct is 12411");
}

#[test]
fn test_part1() {
    let content = common::get_lines_from_file("testinput.txt");
    let result = solve_part_1(&content);
    println!("result part1 {:?}", result);
    assert_eq!(15, result);
    assert_eq!(8, solve_part_1(&vec!["A Y"]));
    assert_eq!(1, solve_part_1(&vec!["B X"]));
    assert_eq!(6, solve_part_1(&vec!["C Z"]));
}

#[test]
fn test_part2() {
    let content = common::get_lines_from_file("testinput.txt");
    let result = solve_part_2(&content);
    println!("result part2 {:?}", result);
    assert_eq!(12, result);
    assert_eq!(4, solve_part_2(&vec!["A Y"]));
    assert_eq!(1, solve_part_2(&vec!["B X"]));
    assert_eq!(7, solve_part_2(&vec!["C Z"]));
}
