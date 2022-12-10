use std::fs;

fn report(cycle: i32, x: i32) -> i32 {
    if (cycle - 20) % 40 == 0 {
        let ret = cycle * x;
        println!("{cycle}, {x}, {ret}");
        return ret;
    }
    0
}
fn main() {
    let data = fs::read_to_string("test1.txt").expect("read to string failed");
    let lines: Vec<&str> = data.split("\n").collect();
    let mut cycle = 0;
    let mut x = 1;
    let mut sum = 0;
    for line in lines {
        let s: Vec<_> = line.trim().split(" ").collect();
        if s[0] == "noop" {
            cycle += 1;
            sum += report(cycle, x);
        } else if s[0] == "addx" {
            let val: i32 = s[1].parse().unwrap();
            cycle += 1;
            sum += report(cycle, x);
            cycle += 1;
            sum += report(cycle, x);
            x += val;
        } else {
            unreachable!();
        }
    }

    println!("{sum}");
}
