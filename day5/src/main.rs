use std::fs;

// counts the number of stacks
fn stack_count(_s: &Vec<&str>) -> usize {
    // todo
    9
}
fn parse() -> Vec<Vec<char>> {
    let data = fs::read_to_string("test1.txt").expect("read to string failed");
    let lines: Vec<&str> = data.split("\n").collect();

    // count stacks
    let stacks = stack_count(&lines);
    let mut v: Vec<Vec<char>> = Vec::new();
    (0..stacks).for_each(|_i| {
        let vc: Vec<char> = Vec::new();
        v.push(vc);
    });

    // loads stacks
    let mut parse_stacks = true;
    for line in lines {
        //println!("{}", line);
        if line.len() == 1 {
            // empty line seperating stacks from instructions.
            parse_stacks = false;
            (0..stacks).for_each(|i| {
                v[i].reverse();
            });

            continue;
        }
        if parse_stacks {
            let linechars: Vec<char> = line.chars().collect();
            if linechars[1] == '1' {
                // stack identifier
                continue;
            }
            for i in 0..stacks {
                let index: usize = 1 + (i * 4);
                let c = linechars[index];
                if c != ' ' {
                    //println!("pushing {c}");
                    v[i].push(c);
                }
            }
        } else {
            let s: Vec<&str> = line.trim().split(" ").collect();
            let count: usize = s[1].parse().unwrap();
            let source: usize = s[3].parse().unwrap();
            let dest: usize = s[5].parse().unwrap();
            // learnt here: https://git.jessewalling.com/jessolm/advent_of_code/src/branch/master/day05/src/main.rs
            let stack = &mut v[source - 1];
            let items = stack.split_off(stack.len() - count);
            let stack = &mut v[dest - 1];
            stack.extend(items);
        }
    }

    v
}

fn main() {
    let v = parse();
    for inner in &v {
        print!("{}", inner[inner.len() - 1]);
    }
}
