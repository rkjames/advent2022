use std::{collections::HashSet, fs};

fn parse() -> Vec<(char, i32)> {
    let data = fs::read_to_string("test1.txt").expect("read to string failed");
    let lines: Vec<&str> = data.split("\n").collect();
    let mut v: Vec<(char, i32)> = Vec::new();
    for line in lines {
        let s: Vec<_> = line.trim().split(" ").collect();
        let i: i32 = s[1].parse().unwrap();
        let c = s[0].chars().nth(0).unwrap();
        v.push((c, i));
    }
    v
}

fn isclose(tx: i32, ty: i32, hx: i32, hy: i32) -> bool {
    hx >= tx - 1 && hx <= tx + 1 && hy >= ty - 1 && hy <= ty + 1
}

fn main() {
    let v = parse();
    println!("{:?}", v);

    let mut hx = 0;
    let mut hy = 0;
    let mut tx = 0;
    let mut ty = 0;
    let mut visit: HashSet<(i32, i32)> = HashSet::new();

    for (dir, val) in v {
        println!("{dir} {val}");
        for _i in 0..val {
            match dir {
                'R' => hx += 1,
                'L' => hx -= 1,
                'U' => hy += 1,
                'D' => hy -= 1,
                _ => unreachable!(),
            }

            if !isclose(tx, ty, hx, hy) {
                if hy > ty {
                    ty += 1
                } else if hy < ty {
                    ty -= 1
                }
                if hx > tx {
                    tx += 1
                } else if hx < tx {
                    tx -= 1
                }
            }

            println!("t({tx}, {ty}) -> h({hx}, {hy})");
            visit.insert((tx, ty));
        }
    }

    println!("visits = {}", visit.len());
}
