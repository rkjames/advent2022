use std::fs;

fn main() {
    println!("Hello, world!");
    let data = fs::read_to_string("src\\test1.txt").expect("read to string failed");
    let lines: Vec<&str> = data.split("\n").collect();
    println!("{}", lines.len());
    let mut elf: Vec<(i32, i32)> = Vec::new();
    let mut current = 0;
    elf.push((0, 0));
    for line in &lines {
        if line.len() == 1 {
            elf.push((0, 0));
            current += 1;
        } else {
            let guess: i32 = match line.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            elf[current].1 += guess;
        }
    }
    println!("{:?}", elf);
    elf.sort_by_key(|k| k.1);
    let i = elf.len() - 1;
    println!(
        "top 3 elves carrying {}",
        elf[i].1 + elf[i - 1].1 + elf[i - 2].1
    );
}
