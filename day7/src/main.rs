use std::{collections::HashMap, fs};

fn parse() -> HashMap<String, i32> {
    let data = fs::read_to_string("test1.txt").expect("read to string failed");
    let lines: Vec<&str> = data.split("\n").collect();
    let mut dir: HashMap<String, i32> = HashMap::new();
    let mut cwd = String::from("/");
    dir.insert(cwd.clone(), 0);
    for line in lines {
        let a: Vec<&str> = line.trim().split(" ").collect();
        if a[0] == "$" {
            if a[1] == "cd" {
                if a[2] == "/" {
                    cwd = String::from("/");
                } else if a[2] == ".." {
                    parent(&mut cwd);
                } else {
                    if cwd.len() != 1 {
                        cwd.push_str("/");
                    }
                    cwd.push_str(a[2]);
                    if !dir.contains_key(&cwd) {
                        dir.insert(cwd.clone(), 0);
                    }
                }
                // println!("command cd {}, cwd={cwd}", a[2]);
            } else if a[1] == "ls" {
                //println!("command ls");
            } else {
                unreachable!();
            }
        } else {
            if a[0] != "dir" {
                let mut size: i32 = a[0].parse().unwrap();
                let key = cwd.clone();
                if dir.contains_key(&key) {
                    size += dir.get(&key).unwrap();
                }
                dir.insert(key, size);
            }
        }
    }
    dir
}

fn parent(cwd: &mut String) {
    let index = cwd.rfind("/").unwrap();
    cwd.truncate(index);
    if cwd.len() == 0 {
        *cwd = String::from("/");
    }
}

// todo: rust doesn't seem to like appending tuples to vectors
#[derive(Debug)]
struct Update {
    key: String,
    size: i32,
}
fn main() {
    let mut d = parse();
    for i in (1..10).rev() {
        println!("hello");
        // rust doesn't seem to allow updates while iterating, so 2 phase.
        let mut updates: Vec<Update> = Vec::new();
        for (cwd, size) in d.iter() {
            if cwd == "/" {
                continue;
            }
            let slash_count = cwd.matches("/").count();
            if slash_count == i {
                let mut pkey = cwd.clone();
                parent(&mut pkey);
                let u = Update {
                    key: pkey,
                    size: *size,
                };
                updates.push(u);
            }
        }

        //println!("{:?}", updates);

        for u in updates {
            let start = d.get(&u.key).unwrap();
            d.insert(u.key, start + u.size);
        }
    }
    //println!("{:?}", d);

    // todo: iterator adapter fanciness
    let mut sum: i32 = 0;
    for (_cwd, size) in d.iter() {
        if *size <= 100000 {
            sum += size;
        }
    }

    println!("sum of smaller dirs part1 = {sum}");

    let root_size = d.get("/").unwrap();
    println!("root size={}", root_size);

    let free = root_size - 40000000;
    println!("free={}", free);

    let mut candidate = 999999999;
    for (cwd, size) in d.iter() {
        if *size < candidate && *size >= free {
            println!("found {cwd}={size}");
            candidate = *size;
        }
    }
    println!("size part2={candidate}");
}
