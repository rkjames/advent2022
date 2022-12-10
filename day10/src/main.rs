use std::fs;

fn report(cycle: i32, x: i32, s: &mut String) {
    let pixel = (cycle - 1) % 40;
    let start = x - 1;
    let end = start + 3;

    if start <= pixel && pixel < end {
        s.push('#');
    } else {
        s.push('.');
    }
    //println!("{cycle}:{s}  x={x}, pixel={pixel}, start={start}, end={end}");
    if cycle % 40 == 0 {
        println!("{s}");
        s.clear();
    }
}
fn main() {
    let data = fs::read_to_string("test1.txt").expect("read to string failed");
    let lines: Vec<&str> = data.split("\n").collect();
    let mut cycle = 0;
    let mut x = 1;
    let mut row = String::new();
    for line in lines {
        let s: Vec<_> = line.trim().split(" ").collect();
        if s[0] == "noop" {
            cycle += 1;
            report(cycle, x, &mut row);
        } else if s[0] == "addx" {
            let val: i32 = s[1].parse().unwrap();
            cycle += 1;
            report(cycle, x, &mut row);
            cycle += 1;
            report(cycle, x, &mut row);
            x += val;
        } else {
            unreachable!();
        }
    }
}
