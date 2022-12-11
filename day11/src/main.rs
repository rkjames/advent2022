use std::{collections::VecDeque, fs};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Operation {
    Add,
    Multiply,
    Square,
}

#[derive(Debug, PartialEq, Clone)]
struct Monkey {
    items: VecDeque<i32>,
    op: Operation,
    op_val: i32,
    test: i32,
    test_true: usize,
    test_false: usize,
    inspect: usize,
}

fn parse() -> Vec<Monkey> {
    let data = fs::read_to_string("test1.txt").expect("read to string failed");
    let lines: Vec<&str> = data.split("\n").collect();
    let mut monkeys = Vec::new();

    let mut it = lines.iter();
    loop {
        // Monkey 0:
        let line_op = it.next();
        if line_op.is_none() {
            break;
        }

        // Starting items: 79, 98
        let line = it.next().unwrap().trim();
        let offset = line.find(char::is_numeric).unwrap();
        let s: Vec<_> = line[offset..].split(",").collect();
        let mut items: VecDeque<i32> = VecDeque::new();
        for i in s {
            let ite: i32 = i.trim().parse().unwrap();
            items.push_back(ite);
        }

        // Operation: new = old * 19
        let line = it.next().unwrap().trim();
        let mut op = Operation::Multiply;
        if line.find("*").is_none() {
            op = Operation::Add
        }
        let mut op_val = 0;
        let offset = line.find(char::is_numeric);
        if offset.is_none() {
            assert_eq!(op, Operation::Multiply);
            op = Operation::Square;
        } else {
            op_val = line[offset.unwrap()..].parse().unwrap();
        }

        // todo: move these repeated 3 lines to function.
        // Test: divisible by 23
        let line = it.next().unwrap().trim();
        let offset = line.find(char::is_numeric).unwrap();
        let test: i32 = line[offset..].parse().unwrap();

        //   If true: throw to monkey 2
        let line = it.next().unwrap().trim();
        let offset = line.find(char::is_numeric).unwrap();
        let test_true: usize = line[offset..].parse().unwrap();

        //   If false: throw to monkey 3
        let line = it.next().unwrap().trim();
        let offset = line.find(char::is_numeric).unwrap();
        let test_false: usize = line[offset..].parse().unwrap();

        let m = Monkey {
            items,
            op,
            op_val,
            test,
            test_true,
            test_false,
            inspect: 0,
        };
        monkeys.push(m);

        // blank line
        it.next();
    }

    monkeys
}

fn round(monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        monkeys[i].inspect += monkeys[i].items.len();
        while !monkeys[i].items.is_empty() {
            let item = monkeys[i].items.pop_front().unwrap();
            let m = &monkeys[i];
            // inspect
            let mut worry = match m.op {
                Operation::Add => item + m.op_val,
                Operation::Multiply => item * m.op_val,
                Operation::Square => item * item,
            };

            // relief
            worry = worry / 3;

            // test
            let mut throw: usize = m.test_false;
            if worry % m.test == 0 {
                throw = m.test_true;
            }
            // println!("{i} -> {worry}: throw to {throw}");
            monkeys[throw].items.push_back(worry);
        }
    }
}

fn main() {
    let mut m = parse();
    for _r in 0..20 {
        round(&mut m);
        //println!("round {r}: {:?}", m);
    }

    let mut v: Vec<usize> = m.iter().map(|m| m.inspect).collect();
    v.sort();
    v.reverse();
    let business = v[0] * v[1];
    println!("{} * {} = business {business}", v[0], v[1]);
}
