use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    #[allow(dead_code)] //used for debugging only
    name: String,
    children: Vec<Rc<RefCell<Node>>>,
    size: u32,
}

impl Node {
    fn new_dir(name: String) -> Self {
        Node { name, children: vec![], size: 0 }
    }

    fn new_file(name: String, value: u32) -> Self {
        Node { name, children: vec![], size: value }
    }


    fn add_child(&mut self, child: Rc<RefCell<Node>>) {
        self.children.push(child);
    }

    fn total(&self) -> u32 {
        self.size + self.children.iter().map(|c|c.borrow().total()).sum::<u32>()
    }
}

fn calc_dir_sizes<T: AsRef<str>>(lines: &[T]) -> (Rc<RefCell<Node>>, Vec<Rc<RefCell<Node>>>) {
    let root = Rc::new(RefCell::new(Node::new_dir(String::from("/"))));

    let mut location = vec![Rc::clone(&root)];
    let mut current = Rc::clone(&root);
    let mut dirs = vec![Rc::clone(&root)];

    let mut iter = lines.iter().peekable();
    iter.next(); //skip 'cd /'
    while let Some(l) = iter.next() {
        let line = l.as_ref();
        if line.starts_with("$ cd ..") {
            location.pop();
            current = Rc::clone(location.last().unwrap());
        } else if line.starts_with("$ cd") {
            let dir = line.split(' ').last().unwrap();
            let node = Rc::new(RefCell::new(Node::new_dir(String::from(dir))));
            let parent = Rc::clone(location.last().unwrap());
            parent.borrow_mut().add_child(Rc::clone(&node));
            location.push(Rc::clone(&node));
            current = Rc::clone(&node);
            dirs.push(Rc::clone(&node));
        } else if line == "$ ls" {
            while let Some(peek) = iter.peek() {
                if peek.as_ref().starts_with('$') {
                    break;
                }
                let f = iter.next().unwrap().as_ref();
                if !f.starts_with("dir") {
                    let (s, name) = f.split_once(' ').unwrap();
                    let size = s.parse::<u32>().unwrap();
                    let node = Node::new_file(String::from(name), size);
                    current.borrow_mut().add_child(Rc::new(RefCell::new(node)));
                }
            }
        }
    }

    (root, dirs)
}

fn solve_part_1<T: AsRef<str>>(lines: &[T]) -> u32 {
    let (_, dirs) = calc_dir_sizes(lines);
    let result: _ = dirs.iter().filter_map(|n| {
        let total = n.borrow().total();
        if total < 100_000 { Some(total) } else { None }
    }).sum();
    result
}

fn solve_part_2<T: AsRef<str>>(lines: &[T]) -> u32 {
    let (root, dirs) = calc_dir_sizes(lines);

    let used = root.borrow().total();
    let free = 70_000_000 - used;
    let to_free = 30_000_000 - free;

    dirs.iter().filter_map(|d| {
        let total = d.borrow().total();
        if total > to_free { Some(total) } else { None }
    }).min().unwrap()

}

fn main() {
    let lines = common::get_lines_from_file("day07/input.txt");
    let result1 = solve_part_1(&lines);
    println!("result part 1 is {:?}", result1);
    println!("Correct is 1443806");
    let result2 = solve_part_2(&lines);
    println!("result part 2 is {:?}", result2);
    println!("Correct is 942298");
}

#[test]
fn test_part1() {
    let lines = common::get_lines_from_file("testinput.txt");
    let (_, dirs) = calc_dir_sizes(&lines);
    assert_eq!(4, dirs.len());

    let result = solve_part_1(&lines);
    assert_eq!(95437, result);
    assert_eq!(94853, dirs[1].borrow().total());
    assert_eq!(584, dirs[2].borrow().total());
}