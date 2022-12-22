use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

fn operation_from_string(line: &str) -> (String, u64) {
    //Operation: new = old + 6
    let mut tokens = line.trim().split(' ').collect::<Vec<&str>>();
    let first = tokens[0];

    let operand = match tokens.last().unwrap().parse::<u64>() {
        Ok(x) => x,
        Err(_) => {
            tokens[4] = "**";
            2
        }
    };

    let operator = match first {
        "Operation:" => { tokens[4] }
        "Test:" => { "/" }
        _ => panic!("Unknown start token {:?}", first),
    };
    (String::from(operator), operand)
}

#[allow(dead_code)]
struct Monkey {
    id: i32,
    items: Vec<u64>,
    operation: (String, u64),
    test: (String, u64),
    target_true: i32,
    target_false: i32,
    counter: i32,
}

impl Monkey {
    fn from_string(lines: &[String]) -> Self {
        // Monkey 0:
        //     Starting items: 79, 98
        // Operation: new = old * 19
        // Test: divisible by 23
        // If true: throw to monkey 2
        // If false: throw to monkey 3
        println!("{:?}", lines);
        let id = lines.get(0).unwrap().split_once(' ').unwrap().1.strip_suffix(':').unwrap().parse::<i32>().unwrap();
        let items = lines.get(1).unwrap().split_once(": ").unwrap().1.split(", ").map(|c| c.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        let operation = operation_from_string(lines.get(2).unwrap());
        let test = operation_from_string(lines.get(3).unwrap());
        let target_true = lines.get(4).unwrap().chars().last().unwrap() as i32 - 48; //transform char to value
        let target_false = lines.get(5).unwrap().chars().last().unwrap() as i32 - 48; //transform char to value
        let counter = 0;

        Monkey { id, items, operation, test, target_true, target_false, counter }
    }

    fn cal_new_worry(&self, worry: u64) -> u64 {
        match self.operation.0.as_str() {
            "*" => worry * self.operation.1,
            "+" => worry + self.operation.1,
            "**" => worry.pow(self.operation.1 as u32),
            _ => panic!("Unknown operator '{:?}'", self.operation.0)
        }
    }

    fn test_worry(&self, worry: u64) -> (i32, u64) {
        match self.test.0.as_str() {
            "/" => {
                match worry % self.test.1 == 0 {
                    true => (self.target_true, worry),
                    false => (self.target_false, worry),
                }
            }
            _ => panic!("Unknown test operator {:?}", self.test.0)
        }
    }

    // -> monkey, worry
    fn next_monkey(&mut self, unworry : u64) -> (i32, u64) {
        let mut worry = self.items.remove(0);
        // panic
        worry = self.cal_new_worry(worry);
        if unworry == 0 {
            //calm down, nothing broke
            worry /= 3_u64;
        } else {
            worry %= unworry;
        }
        // monkey tests
        self.counter += 1;
        self.test_worry(worry)
    }
}


fn solve_part_1() -> i32 {
    let lines = common::get_lines_from_file("day11/input.txt");
    let mut monkeys = lines.chunks(7).map(|l| {
        let m = Monkey::from_string(l);
        Rc::new(RefCell::new(m))
    }).collect::<Vec<Rc<RefCell<Monkey>>>>();

    for _ in 0..20 {
        for (_, m_ref) in monkeys.iter().enumerate() {
            let m: &RefCell<Monkey> = m_ref.borrow();
            while !m.borrow().items.is_empty() {
                let (t_id, worry) = m.borrow_mut().next_monkey(0);
                let target: &RefCell<Monkey> = monkeys[t_id as usize].borrow();
                target.borrow_mut().items.push(worry);
            }
        }
    }

    monkeys.sort_by(|m1_ref, m2_ref| {
        let m1: &RefCell<Monkey> = m1_ref.borrow();
        let m2: &RefCell<Monkey> = m2_ref.borrow();
        m1.borrow().counter.cmp(&m2.borrow().counter).reverse()
    });

    let m0: &RefCell<Monkey> = monkeys[0].borrow();
    let c0 = m0.borrow().counter;
    let m1: &RefCell<Monkey> = monkeys[1].borrow();
    let c1 = m1.borrow().counter;
    c0 * c1
}

fn solve_part_2() -> u64 {
    let lines = common::get_lines_from_file("day11/input.txt");
    let mut monkeys = lines.chunks(7).map(|l| {
        let m = Monkey::from_string(l);
        Rc::new(RefCell::new(m))
    }).collect::<Vec<Rc<RefCell<Monkey>>>>();

    let unworry = monkeys.iter().map(|m| {
        let m_borrowed : &RefCell<Monkey> = m.borrow();
        m_borrowed.borrow().test.1
    }).reduce(|a, b| a*b).unwrap();
    for _ in 0..10000 {
        for (_, m_ref) in monkeys.iter().enumerate() {
            let m: &RefCell<Monkey> = m_ref.borrow();
            while !m.borrow().items.is_empty() {
                let (t_id, worry) = m.borrow_mut().next_monkey(unworry);
                let target: &RefCell<Monkey> = monkeys[t_id as usize].borrow();
                target.borrow_mut().items.push(worry);
            }
        }
    }

    monkeys.sort_by(|m1_ref, m2_ref| {
        let m1: &RefCell<Monkey> = m1_ref.borrow();
        let m2: &RefCell<Monkey> = m2_ref.borrow();
        m1.borrow().counter.cmp(&m2.borrow().counter).reverse()
    });

    let m0: &RefCell<Monkey> = monkeys[0].borrow();
    let c0 = m0.borrow().counter;
    let m1: &RefCell<Monkey> = monkeys[1].borrow();
    let c1 = m1.borrow().counter;
    c0 as u64 * c1 as u64
}

fn main() {
    let result1 = solve_part_1();
    println!("result part 1 is {:?}", result1);
    println!("Correct is 55458");
    let result2 = solve_part_2();
    println!("result part 2 is {:?}", result2);
    println!("Correct is 14508081294");
}

#[test]
fn test_part1_1() {
    let lines = common::get_lines_from_file("testinput.txt");
    let monkeys = lines.chunks(7).map(|l| {
        let m = Monkey::from_string(l);
        Rc::new(RefCell::new(m))
    }).collect::<Vec<Rc<RefCell<Monkey>>>>();
    assert_eq!(4, monkeys.len());
    let m0: &RefCell<Monkey> = monkeys[0].borrow();

    for (s_id, m_ref) in monkeys.iter().enumerate() {
        println!("Monkey {:?}", s_id);
        let m: &RefCell<Monkey> = m_ref.borrow();
        while !m.borrow().items.is_empty() {
            println!("-----");
            let (t_id, worry) = m.borrow_mut().next_monkey(0);
            println!("Sending {:?} from {:?} to {:?}", worry, s_id, t_id);
            let target: &RefCell<Monkey> = monkeys[t_id as usize].borrow();
            target.borrow_mut().items.push(worry);
        }
    }

    assert_eq!(vec![20, 23, 27, 26], m0.borrow().items);
    let m1: &RefCell<Monkey> = monkeys[1].borrow();
    assert_eq!(vec![2080, 25, 167, 207, 401, 1046], m1.borrow().items);
    let m2: &RefCell<Monkey> = monkeys[2].borrow();
    assert!(m2.borrow().items.is_empty());
    let m3: &RefCell<Monkey> = monkeys[3].borrow();
    assert!(m3.borrow().items.is_empty());
}

#[test]
fn test_part1_2() {
    let lines = common::get_lines_from_file("testinput.txt");
    let monkeys = lines.chunks(7).map(|l| {
        let m = Monkey::from_string(l);
        Rc::new(RefCell::new(m))
    }).collect::<Vec<Rc<RefCell<Monkey>>>>();
    assert_eq!(4, monkeys.len());
    let m0: &RefCell<Monkey> = monkeys[0].borrow();

    for _ in 0..20 {
        for (s_id, m_ref) in monkeys.iter().enumerate() {
            println!("Monkey {:?}", s_id);
            let m: &RefCell<Monkey> = m_ref.borrow();
            while !m.borrow().items.is_empty() {
                println!("-----");
                let (t_id, worry) = m.borrow_mut().next_monkey(0);
                println!("Sending {:?} from {:?} to {:?}", worry, s_id, t_id);
                let target: &RefCell<Monkey> = monkeys[t_id as usize].borrow();
                target.borrow_mut().items.push(worry);
            }
        }
    }

    assert_eq!(vec![10, 12, 14, 26, 34], m0.borrow().items);
    assert_eq!(101, m0.borrow().counter);
    let m1: &RefCell<Monkey> = monkeys[1].borrow();
    assert_eq!(95, m1.borrow().counter);
    assert_eq!(vec![245, 93, 53, 199, 115], m1.borrow().items);
    let m2: &RefCell<Monkey> = monkeys[2].borrow();
    assert_eq!(7, m2.borrow().counter);
    assert!(m2.borrow().items.is_empty());
    let m3: &RefCell<Monkey> = monkeys[3].borrow();
    assert_eq!(105, m3.borrow().counter);
    assert!(m3.borrow().items.is_empty());
    assert_eq!(10605, m3.borrow().counter * m0.borrow().counter);
}

#[test]
fn test_part2() {
    let lines = common::get_lines_from_file("testinput.txt");
    let monkeys = lines.chunks(7).map(|l| {
        let m = Monkey::from_string(l);
        Rc::new(RefCell::new(m))
    }).collect::<Vec<Rc<RefCell<Monkey>>>>();
    assert_eq!(4, monkeys.len());
    let m0: &RefCell<Monkey> = monkeys[0].borrow();
    let unworry = monkeys.iter().map(|m| {
        let m_borrowed : &RefCell<Monkey> = m.borrow();
        m_borrowed.borrow().test.1
    }).reduce(|a, b| a*b).unwrap();

    for _ in 0..10000 {
        for (_, m_ref) in monkeys.iter().enumerate() {
            // println!("Monkey {:?}", s_id);
            let m: &RefCell<Monkey> = m_ref.borrow();
            while !m.borrow().items.is_empty() {
                // println!("-----");
                let (t_id, worry) = m.borrow_mut().next_monkey(unworry);
                // println!("Sending {:?} from {:?} to {:?}", worry, s_id, t_id);
                let target: &RefCell<Monkey> = monkeys[t_id as usize].borrow();
                target.borrow_mut().items.push(worry);
            }
        }
    }

    assert_eq!(52166, m0.borrow().counter);
    let m1: &RefCell<Monkey> = monkeys[1].borrow();
    assert_eq!(47830, m1.borrow().counter);
    let m2: &RefCell<Monkey> = monkeys[2].borrow();
    assert_eq!(1938, m2.borrow().counter);
    let m3: &RefCell<Monkey> = monkeys[3].borrow();
    assert_eq!(52013, m3.borrow().counter);
}