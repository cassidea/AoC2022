fn marker_index(line: &str, marker_size: usize) -> u32 {
    'outer: for i in 0..line.len() - marker_size {
        let mut temp: [u8; 26] = [0; 26];
        let current = line.get(i..(i + marker_size)).unwrap().as_bytes();
        for c in current {
            // 'a' is 97
            if temp[(c - 97) as usize] > 0 {
                continue 'outer;
            } else {
                temp[(c - 97) as usize] = 1;
            }
        }
        return (i + marker_size) as u32;
    }
    panic!();
}

fn solve_part_1(lines: &str) -> u32 {
    marker_index(lines, 4)
}


fn solve_part_2(lines: &str) -> u32 { marker_index(lines, 14) }

fn main() {
    let lines = common::get_lines_as_string("day06/input.txt");
    let result1 = solve_part_1(&lines);
    println!("result part 1 is {:?}", result1);
    println!("Correct is 1855");
    let result2 = solve_part_2(&lines);
    println!("result part 2 is {:?}", result2);
    println!("Correct is 3256");
}

#[test]
fn test_part1() {
    assert_eq!(7, marker_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4));
    assert_eq!(5, marker_index("bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
    assert_eq!(6, marker_index("nppdvjthqldpwncqszvftbrmjlhg", 4));
    assert_eq!(10, marker_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4));
    assert_eq!(11, marker_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4));
}

#[test]
fn test_part2() {
    assert_eq!(19, marker_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14));
    assert_eq!(23, marker_index("bvwbjplbgvbhsrlpgdmjqwftvncz", 14));
    assert_eq!(23, marker_index("nppdvjthqldpwncqszvftbrmjlhg", 14));
    assert_eq!(29, marker_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14));
    assert_eq!(26, marker_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14));
}