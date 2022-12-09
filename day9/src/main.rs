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

#[derive(Hash, Debug, Eq, PartialEq, Clone, Copy)]
struct Knot {
    x: i32,
    y: i32,
}

fn main() {
    let v = parse();
    println!("{:?}", v);

    let mut rope: Vec<Knot> = Vec::new();
    for _i in 0..10 {
        rope.push(Knot { x: 0, y: 0 });
    }
    let mut visit: HashSet<Knot> = HashSet::new();

    for (dir, val) in v {
        println!("{dir} {val}");
        for _i in 0..val {
            {
                let head = &mut rope[0];
                match dir {
                    'R' => head.x += 1,
                    'L' => head.x -= 1,
                    'U' => head.y += 1,
                    'D' => head.y -= 1,
                    _ => unreachable!(),
                }
            }

            // todo: error[E0499]: cannot borrow `rope` as mutable more than once at a time
            // --> no references --> hard to read
            for i in 1..rope.len() {
                if !(rope[i - 1].x >= rope[i].x - 1
                    && rope[i - 1].x <= rope[i].x + 1
                    && rope[i - 1].y >= rope[i].y - 1
                    && rope[i - 1].y <= rope[i].y + 1)
                {
                    if rope[i - 1].y > rope[i].y {
                        rope[i].y += 1
                    } else if rope[i - 1].y < rope[i].y {
                        rope[i].y -= 1
                    }
                    if rope[i - 1].x > rope[i].x {
                        rope[i].x += 1
                    } else if rope[i - 1].x < rope[i].x {
                        rope[i].x -= 1
                    }
                }
            }

            //println!("{:?}", rope);
            let tail = rope[rope.len() - 1];
            visit.insert(tail);
        }
    }

    println!("visits = {}", visit.len());
}
