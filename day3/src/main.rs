use std::fs;
use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
    let data = fs::read_to_string("test1.txt").expect("read to string failed");
    let lines: Vec<&str> = data.split("\n").collect();
    let mut extra : Vec<char> = Vec::new();

    for line in lines {
        let (left,right) = line.split_at(line.len() / 2);
        let mut s = HashSet::new();

        for c in left.chars() {
            s.insert(c);
        }

        for c in right.chars() {
            if s.contains(&c) {
                extra.push(c);
                break;
            }
        }
    }

    println!("{:?}", extra);
    let mut sum = 0;
    for c in extra {
        let mut val = 0;
        if c.is_lowercase() {
            val = c as i32 - 'a' as i32;
        } else if c.is_uppercase() {
            val = c as i32 - 'A' as i32 + 26;
        }
        println!("--{}--{}--", c, val);
        // 0 based
        sum += val + 1;
    }
    println!("sum={}", sum);
}
