use std::{fs, collections::HashSet};

fn main() {
    println!("Hello, world!");
    let data = fs::read_to_string("test1.txt").expect("read to string failed");
    let lines: Vec<&str> = data.split("\n").collect();
    let mut count = 0;
    for line in &lines {
        let section : Vec<&str> = line.trim().split(",").collect();
        let first = range_to_set(section[0]);
        let second = range_to_set(section[1]);
        let intersection : HashSet<_> = first.intersection(&second).collect();
        if intersection.len() > 0 {
            count +=1;
        }
    }
    println!("{count} intersections");
}

fn range_to_set(range: &str) -> HashSet<i32> {
    let first :Vec<&str> = range.split("-").collect();
    let first_start : i32 = first[0].parse().unwrap();
    let first_end : i32 = first[1].parse().unwrap();
    let mut first_set : HashSet<i32> = HashSet::new();
    for i in first_start..first_end+1 {
        first_set.insert(i);
    }
    first_set
}
