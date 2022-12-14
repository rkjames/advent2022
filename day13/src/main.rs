use std::{cmp::Ordering, fs};

#[derive(Debug)]
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
            if stack.len() == 1 {
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
        } else if c == ',' {
            // nothing
        } else {
            unreachable!();
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
                    let result = ordered(&llist[i], &rlist[i]);
                    if result != Ordering::Equal {
                        return result;
                    }
                }
                if llist.len() < rlist.len() {
                    // "If the left list runs out of items first, the inputs are in the right order."
                    return Ordering::Less;
                } else {
                    return Ordering::Equal;
                }
            } else {
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
    let data = fs::read_to_string("test1.txt").expect("read to string failed");
    let lines: Vec<&str> = data.split("\n").collect();
    let mut it = lines.iter();
    let mut parsed_lines: Vec<&str> = Vec::new();
    loop {
        parsed_lines.push(it.next().unwrap().trim());
        parsed_lines.push(it.next().unwrap().trim());
        let blank = it.next();
        if blank.is_none() {
            break;
        }
    }
    parsed_lines.push("[[2]]");
    parsed_lines.push("[[6]]");

    parsed_lines.sort_by(|a, b| {
        let aitem = parse(a);
        let bitem = parse(b);
        return ordered(&aitem, &bitem);
    });

    let mut first = 0;
    let mut second = 0;
    for i in 0..parsed_lines.len() {
        if parsed_lines[i] == "[[2]]" {
            first = i + 1;
        }
        if parsed_lines[i] == "[[6]]" {
            second = i + 1;
        }
    }
    println!("{} * {} = {}", first, second, first * second);
}
