fn parse_crates<T: AsRef<str>>(lines: &[T]) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = vec![];
    for line in lines {
        if line.as_ref().starts_with(" 1") {
            break;
        }

        line.as_ref().chars().collect::<Vec<char>>()
            .chunks(4)
            .filter(|c| { !c.is_empty() })
            .enumerate()
            .for_each(|(col, v)| {
                if result.len() <= col {
                    result.push(vec![]);
                }
                if !v[1].is_whitespace() {
                    result[col].insert(0, v[1]);
                }
            });
    }
    result
}

fn parse_move_command(line: &str) -> (u32, usize, usize) {
    let tokens: Vec<&str> = line.split(' ').collect();
    let amount = tokens[1].parse::<u32>().unwrap();
    let source = tokens[3].parse::<usize>().unwrap();
    let target = tokens[5].parse::<usize>().unwrap();

    (amount, source, target)
}

fn reorder_9000(line : &str, crates: &mut [Vec<char>]) {
    let (amount, source, target) = parse_move_command(line);

    for _ in 0..amount {
        let c = crates[source - 1].pop().unwrap();
        crates[target - 1].push(c);
    }
}

fn reorder_9001(line : &str, crates: &mut [Vec<char>]) {
    let (amount, source, target) = parse_move_command(line);
    let mut temp = vec![];
    for _ in 0..amount {
        temp.insert(0, crates[source - 1].pop().unwrap());
    }
    crates[target - 1].append(&mut temp);
}

fn reorder_creates<T: AsRef<str>>(crates: &mut [Vec<char>], lines: &[T], order_func : &dyn Fn(&str, &mut [Vec<char>])) {
    for line in lines {
        if !line.as_ref().starts_with("move") {
            continue;
        }
        order_func(line.as_ref(), crates);
    }
}

fn head_of_crates(crates: &[Vec<char>]) -> Vec<char> {
    crates.iter().map(|vec| *vec.last().unwrap()).collect()
}


fn solve_part_1<T: AsRef<str>>(lines: &[T]) -> String {
    let mut crates = parse_crates(lines);
    reorder_creates(&mut crates, lines, &reorder_9000);
    head_of_crates(&crates).into_iter().collect()
}

fn solve_part_2<T: AsRef<str>>(lines: &[T]) -> String {
    let mut crates = parse_crates(lines);
    reorder_creates(&mut crates, lines, &reorder_9001);
    head_of_crates(&crates).into_iter().collect()
}

fn main() {
    let lines = common::get_lines_from_file("day05/input.txt");
    let result1 = solve_part_1(&lines);
    println!("result part 1 is {:?}", result1);
    println!("Correct is TLNGFGMFN");
    let result2 = solve_part_2(&lines);
    println!("result part 2 is {:?}", result2);
    println!("Correct is FGLQJCMBD");
}

#[test]
fn test_part1() {
    let lines = common::get_lines_from_file("testinput.txt");
    let mut crates = parse_crates(&lines);
    reorder_creates(&mut crates, &lines, &reorder_9000);
    assert_eq!(vec![vec!['C'], vec!['M'], vec!['P', 'D', 'N', 'Z']], crates);
    assert_eq!(vec!['C', 'M', 'Z'], head_of_crates(&crates));
}

#[test]
fn test_part2() {
    let lines = common::get_lines_from_file("testinput.txt");
    let mut crates = parse_crates(&lines);
    reorder_creates(&mut crates, &lines, &reorder_9001);
    assert_eq!(vec![vec!['M'], vec!['C'], vec!['P', 'Z', 'N', 'D']], crates);
    assert_eq!(vec!['M', 'C', 'D'], head_of_crates(&crates));
}
