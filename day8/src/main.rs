use std::fs;

fn parse() -> Vec<Vec<i32>> {
    let data = fs::read_to_string("test1.txt").expect("read to string failed");
    let lines: Vec<&str> = data.split("\n").collect();
    let mut v: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        let mut row: Vec<_> = Vec::new();
        let chars: Vec<_> = line.trim().chars().collect();
        for c in chars {
            let i = c as i32 - '0' as i32;
            row.push(i);
        }
        v.push(row);
    }
    v
}

fn main() {
    let v = parse();
    let l = v.len();

    // track what we've seen
    let mut seen: Vec<Vec<bool>> = Vec::new();
    for _x in 0..l {
        let mut row = Vec::new();
        for _y in 0..l {
            row.push(false);
        }
        seen.push(row);
    }

    for r in 0..l {
        // todo: how to make array of iter + rev iter
        // todo: how to do a decreasing for loop variable?
        // current height
        let mut current: i32 = -1;
        for i in 0..l {
            if v[r][i] > current {
                current = v[r][i];
                seen[r][i] = true;
            }
        }

        let mut current: i32 = -1;
        for i in (0..l).rev() {
            if v[r][i] > current {
                current = v[r][i];
                seen[r][i] = true;
            }
        }

        let mut current: i32 = -1;
        for i in 0..l {
            if v[i][r] > current {
                current = v[i][r];
                seen[i][r] = true;
            }
        }

        let mut current: i32 = -1;
        for i in (0..l).rev() {
            if v[i][r] > current {
                current = v[i][r];
                seen[i][r] = true;
            }
        }
    }

    println!("{:?}", v);
    println!("{:?}", seen);
    let mut count = 0;
    for r in seen {
        count += r.iter().filter(|b| **b).count();
    }
    println!("count= {count}");
}
