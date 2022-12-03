use std::collections::HashSet;
use std::fs;

fn main() {
    println!("Hello, world!");
    let data = fs::read_to_string("test1.txt").expect("read to string failed");
    let lines: Vec<&str> = data.split("\n").collect();
    let mut extra: Vec<char> = Vec::new();

    let mut count = 0;
    let mut first = HashSet::new();
    let mut second = HashSet::new();
    for line in lines {
        if count == 0 {
            for c in line.chars() {
                first.insert(c);
            }
            count += 1;
        } else if count == 1 {
            for c in line.chars() {
                second.insert(c);
            }
            count += 1;
        } else {
            let intersect: HashSet<_> = first.intersection(&second).collect();
            for c in line.chars() {
                if intersect.contains(&c) {
                    extra.push(c);
                    break;
                }
            }
            count = 0;
            first.clear();
            second.clear();
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
