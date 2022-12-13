use std::collections::VecDeque;

fn get_signal_strength<T: AsRef<str>>(lines: &[T]) -> Vec<i32> {
    let mut x = 1;
    let mut x_at_every_circle = vec![];

    for l in lines {
        let mut add = 0;
        if l.as_ref() == "noop" {
            x_at_every_circle.push(x);
        } else {
            let (_, s) = l.as_ref().split_once(' ').unwrap();
            add = s.parse::<i32>().unwrap();
            x_at_every_circle.push(x);
            x_at_every_circle.push(x);
        }
        x += add;
    };
    x_at_every_circle
}

fn solve<T: AsRef<str>>(lines: &[T]) -> (i32, Vec<Vec<char>>) {
    let relevant_circles = VecDeque::from(vec![20, 60, 100, 140, 180, 220]);
    let all_x = get_signal_strength(lines);

    let mut all_chars = vec![];
    (0..all_x.len()).for_each(|c| {
        let normalized_circle = (c % 40) as i32;
        let x = all_x.get(c).unwrap();
        if normalized_circle % 40 == 0 { all_chars.push(vec![]); }
        if normalized_circle >= *x - 1 && normalized_circle <= *x + 1 {
            all_chars.last_mut().unwrap().push('#');
        } else { all_chars.last_mut().unwrap().push('.'); }
    });

    (relevant_circles.iter().map(|c| *c * all_x.get((*c - 1) as usize).unwrap()).sum(), all_chars)
}

fn main() {
    let lines = common::get_lines_from_file("day10/input.txt");
    let (result1, result2) = solve(&lines);
    println!("result part 1 is {:?}", result1);
    println!("Correct is 17940");

    println!("result part 2 is");
    result2.iter().for_each(|v| println!("{:?}", String::from_iter(v)));
    println!("Correct is ZCBAJFJZ");
}

#[test]
fn test_part1() {
    let lines = vec!["noop", "addx 3", "addx -5"];
    let mut all_circles = get_signal_strength(&lines);
    assert_eq!(vec![1, 1, 1, 4, 4], all_circles);

    let lines = common::get_lines_from_file("testinput.txt");
    all_circles = get_signal_strength(&lines);
    assert_eq!(21, *all_circles.get(20 - 1).unwrap());
    assert_eq!(19, *all_circles.get(60 - 1).unwrap());
    assert_eq!(18, *all_circles.get(100 - 1).unwrap());
    assert_eq!(21, *all_circles.get(140 - 1).unwrap());
    assert_eq!(16, *all_circles.get(180 - 1).unwrap());
    assert_eq!(18, *all_circles.get(220 - 1).unwrap());
    let (result1, result2) = solve(&lines);
    assert_eq!(13140, result1);
    assert_eq!(common::get_lines_from_file("test2result.txt"), result2.iter().map(|c|String::from_iter(c)).collect::<Vec<String>>());
}

