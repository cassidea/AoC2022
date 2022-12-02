use std::fs;

fn solve(content: &str) -> (i32, i32) {
    let mut sums = content
        .split_terminator("\n\n")
        .map(|s| s.split_terminator("\n").map(|e| e.parse::<i32>().unwrap()).sum())
        .collect::<Vec<i32>>();
    sums.sort_unstable();
    let top3 = sums[(sums.len() - 3)..sums.len()].to_vec();

    (top3[2], top3.iter().sum())
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("File not found!");
    let result_part1 = solve(&content);
    println!("Result part1: {}", result_part1.0);
    println!("Part1 66186 is correct");
    println!("Result part1: {}", result_part1.1);
    println!("Part2 196804 is correct");
}


#[test]
fn test_part1() {
    let content = String::from("1000\n\
2000\n\
3000\n\
\n\
4000\n\
\n\
5000\n\
6000\n\
\n\
7000\n\
8000\n\
9000\n\
\n\
10000\n\n");
    let result = solve(&content);
    assert_eq!(24000, result.0);
    assert_eq!(45000, result.1);
}
