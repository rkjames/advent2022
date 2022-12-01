use std::fs;

fn main() {
    println!("Hello, world!");
    let data = fs::read_to_string("src\\test1.txt").expect("read to string failed");
    let lines: Vec<&str> = data.split("\n").collect();
    println!("{}", lines.len());
    let mut elf: Vec<i32> = Vec::new();
    let mut current = 0;
    elf.push(0);
    for line in &lines {
        if line.len() == 1 {
            elf.push(0);
            current += 1;
        } else {
            let guess: i32 = match line.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            elf[current] += guess;
        }
    }
    println!("{:?}", elf);
    let mut max_cal = 0;
    let mut max_index = 0;
    for (pos, e) in elf.iter().enumerate() {
        if *e > max_cal {
            max_cal = *e;
            max_index = pos;
        }
    }
    println!("ask elf {}, carrying {}", max_index + 1, elf[max_index]);
}
