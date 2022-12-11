use std::fs;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Operation {
    Add,
    Multiply,
    Square
}

#[derive(Debug, PartialEq, Clone)]
struct Monkey {
    items: Vec<i32>,
    op: Operation,
    op_val: i32,
    test: i32,
    test_true: usize,
    test_false: usize,
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
        let mut items: Vec<i32> = Vec::new();
        for i in s {
            let ite: i32 = i.trim().parse().unwrap();
            items.push(ite);
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
            op_val  = line[offset.unwrap()..].parse().unwrap();
        }

        // todo: move these repeated 3 lines to function.
        // Test: divisible by 23
        let line = it.next().unwrap().trim();
        let offset = line.find(char::is_numeric).unwrap();
        let test : i32 = line[offset..].parse().unwrap();

        //   If true: throw to monkey 2
        let line = it.next().unwrap().trim();
        let offset = line.find(char::is_numeric).unwrap();
        let test_true : usize = line[offset..].parse().unwrap();

        //   If false: throw to monkey 3
        let line = it.next().unwrap().trim();
        let offset = line.find(char::is_numeric).unwrap();
        let test_false : usize = line[offset..].parse().unwrap();
        
        let m = Monkey { items, op, op_val, test, test_true, test_false};
        monkeys.push(m);

        // blank line
        it.next();
    }

    monkeys
}

fn round(monkeys : &mut Vec<Monkey>) {
    for m in monkeys {
        for i in m.items {
            // inspect
            let mut worry = match m.op {
                Operation::Add => i + m.op_val,
                Operation::Multiply => i * m.op_val,
                Operation::Square => i * i,
            };

            // relief
            worry = worry / 3;

            // test 
            let mut throw: usize = m.test_false;
            if worry % m.test == 0 {
                throw = m.test_true;
            }
            println!("{i} -> {worry}: throw to {throw}");
        }
    }
}

fn main() {
    let mut m = parse();
    round(&mut m);
    println!("{:?}", m);
}
