fn get_rows_and_cols<T: AsRef<str>>(lines: &[T]) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let rows = lines.iter().map(|l| l.as_ref().chars().map(|c| String::from(c).parse::<u8>().unwrap())
        .collect::<Vec<u8>>()).collect::<Vec<Vec<u8>>>();

    let mut cols = vec![];
    for i in 0..rows.len() {
        let mut col = vec![];
        for row in rows.iter() {
            col.push(*row.get(i).unwrap());
        }
        cols.push(col);
    }
    (rows, cols)
}

fn solve_part_1<T: AsRef<str>>(lines: &[T]) -> usize {
    let mut result = 0;
    let (rows, cols) = get_rows_and_cols(lines);
    // Array to store if tree has already been handled to prevent multiple counts of same tree
    let mut temp = vec![0; rows.len() * cols.len()];
    for r in 1..rows.len() - 1 {
        'cols: for c in 1..cols.len() - 1 {
            let row = rows.get(r).unwrap();
            let col = cols.get(c).unwrap();
            let tree_size = row.get(c).unwrap();
            let tree_id = r * row.len() + c;
            // check left
            if let Some(v) = row.get(0..c) {
                if temp[tree_id] == 0 && !v.is_empty() && v.iter().max().unwrap() < tree_size {
                    result += 1;
                    temp[tree_id] += 1;
                    continue 'cols;
                }
            }
            // check right
            if let Some(v) = row.get((c + 1)..row.len()) {
                if temp[r * row.len() + c] == 0 && !v.is_empty() && v.iter().max().unwrap() < tree_size {
                    result += 1;
                    temp[tree_id] += 1;
                    continue 'cols;
                }
            }
            // check top
            if let Some(v) = col.get(0..r) {
                if temp[tree_id] == 0 && !v.is_empty() && v.iter().max().unwrap() < tree_size {
                    result += 1;
                    temp[tree_id] += 1;
                    continue 'cols;
                }
            }
            // check bottom
            if let Some(v) = col.get((r + 1)..col.len()) {
                if temp[tree_id] == 0 && !v.is_empty() && v.iter().max().unwrap() < tree_size {
                    result += 1;
                    temp[tree_id] += 1;
                    continue 'cols;
                }
            }
        }
    }
    result + (2 * (rows.len() - 1)) + (2 * (cols.len() - 1))
}

fn solve_part_2<T: AsRef<str>>(lines: &[T]) -> usize {
    let (rows, cols) = get_rows_and_cols(lines);
    let mut temp = vec![1; rows.len() * cols.len()];
    for r in 0..rows.len() {
        for c in 0..cols.len() {
            let row = rows.get(r).unwrap();
            let col = cols.get(c).unwrap();
            let tree_size = row.get(c).unwrap();
            let tree_id = r * row.len() + c;
            // check left
            temp[tree_id] *= get_view_len_rev(row.get(0..c), tree_size);
            // check right
            temp[tree_id] *= get_view_len(row.get((c + 1)..row.len()), tree_size);
            // check top
            temp[tree_id] *= get_view_len_rev(col.get(0..r), tree_size);
            // check bottom
            temp[tree_id] *= get_view_len(col.get((r + 1)..col.len()), tree_size);
        }
    }

    *temp.iter().max().unwrap()
}

fn get_view_len_rev(vec: Option<&[u8]>, tree_size: &u8) -> usize {
    return match vec {
        Some(v) => if v.is_empty() { 0 } else {
            let mut s = v.iter().rev().take_while(|t| *t < tree_size).count();
            if s == 0 { return 1; } //immediately blocked, count blocking tree
            if s != v.len() { s += 1 }; //view is blocked, count blocking tree
            s
        },
        None => { 0 }
    };
}

fn get_view_len(vec: Option<&[u8]>, tree_size: &u8) -> usize {
    return match vec {
        Some(v) => if v.is_empty() { 0 } else {
            let mut s = v.iter().take_while(|t| *t < tree_size).count();
            if s == 0 { return 1; } //immediately blocked, count blocking tree
            if s != v.len() { s += 1 }; //view is blocked, count blocking tree
            s
        },
        None => { 0 }
    };
}

fn main() {
    let lines = common::get_lines_from_file("day08/input.txt");
    let result1 = solve_part_1(&lines);
    println!("result part 1 is {:?}", result1);
    println!("Correct is 1538");
    let result2 = solve_part_2(&lines);
    println!("result part 2 is {:?}", result2);
    println!("Correct is 496125");
}

#[test]
fn test_part1() {
    let lines = common::get_lines_from_file("testinput.txt");
    let result = solve_part_1(&lines);
    assert_eq!(5, result);
    println!("{:?}", result);
}

#[test]
fn test_part2() {
    let lines = common::get_lines_from_file("testinput.txt");
    let result = solve_part_2(&lines);
    assert_eq!(8, result);
    println!("{:?}", result);
}