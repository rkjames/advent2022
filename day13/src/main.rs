use std::{cmp::Ordering, fs};

enum Item {
    Number(i32),
    List(Vec<Item>),
}

fn parse(line: &str) -> Item {
    let mut stack: Vec<Item> = Vec::new();
    let mut ret = Item::Number(0);
    let mut i: usize = 0;
    let v: Vec<_> = line.chars().collect();
    while i < v.len() {
        let mut c = v[i];
        if c == '[' {
            stack.push(Item::List(Vec::new()));
        } else if c == ']' {
            if stack.len() == 0 {
                ret = stack.pop().unwrap();
            } else {
                let popped = stack.pop().unwrap();
                let item = stack.last_mut().unwrap();
                match item {
                    Item::Number(_n) => unreachable!(),
                    Item::List(list) => list.push(popped),
                }
            }
        } else if c.is_numeric() {
            let mut s = String::from("");
            while c.is_numeric() {
                s.push(c);
                i += 1;
                c = v[i];
            }
            // one past numeric, step back.
            i -= 1;
            // parse+push number.
            let num: i32 = s.parse().unwrap();
            let item = stack.last_mut().unwrap();
            match item {
                Item::Number(_n) => unreachable!(),
                Item::List(list) => list.push(Item::Number(num)),
            }
        }
        i += 1;
    }

    assert!(stack.len() == 0);
    ret
}
// returns true if ordered
fn ordered(left: &Item, right: &Item) -> Ordering {
    if let Item::Number(lnum) = left {
        if let Item::Number(rnum) = right {
            // left isa number, right isa number.
            return lnum.cmp(rnum);
        } else {
            // left isa number, right isa a list, make temp list and compare
            let ltmp = Item::List(vec![Item::Number(*lnum)]);
            return ordered(&ltmp, right);
        }
    } else {
        if let Item::List(llist) = left {
            if let Item::List(rlist) = right {
                // left isa list, right isa list
                for i in 0..llist.len() {
                    if i >= rlist.len() {
                        // "If the right list runs out of items first, the inputs are not in the right order."
                        return Ordering::Greater;
                    }
                    let result = ordered(&llist[i],&rlist[i]);
                    if result != Ordering::Equal {
                        return result;
                    }
                }
                // "If the left list runs out of items first, the inputs are in the right order."
                return Ordering::Less;
            }
            else {
                // left isa list, right isa number.
                if let Item::Number(rnum) = right {
                    let rtmp = Item::List(vec![Item::Number(*rnum)]);
                    return ordered(left, &rtmp);
                } else {
                    unreachable!();
                }
            }
        } else {
            // left must be a number or a list.
            unreachable!();
        }
    }
}

fn main() {
    let data = fs::read_to_string("example1.txt").expect("read to string failed");
    let lines: Vec<&str> = data.split("\n").collect();
    let mut it = lines.iter();
    let mut total = 0;
    let mut pair_count = 1;
    loop {
        let left = parse(it.next().unwrap().trim());
        let right = parse(it.next().unwrap().trim());
        if ordered(&left, &right) != Ordering::Greater {
            total += pair_count;
        }
        let blank = it.next();
        if blank.is_none() {
            break;
        }
        pair_count += 1;
    }

    println!("sum of ordered pair counts {total}");
}
