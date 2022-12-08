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

    // todo: how to make array of iter + rev iter
    // todo: how to do a decreasing for loop variable?

    let mut score = 0;
    let mut location = (0, 0);
    for x in 0..l {
        for y in 0..l {
            // current height
            let current: i32 = v[x][y];

            let mut up: i32 = 0;
            for i in (0..x).rev() {
                up += 1;
                if v[i][y] >= current {
                    break;
                }
            }

            let mut down: i32 = 0;
            for i in x+1..l {
                down += 1;
                if v[i][y] >= current {
                    break;
                }
            }

            let mut left: i32 = 0;
            for i in (0..y).rev() {
                left += 1;
                if v[x][i] >= current {
                    break;
                }
            }

            let mut right: i32 = 0;
            for i in y+1..l {
                right += 1;
                if v[x][i] >= current {
                    break;
                }
            }

            let current_score = left * right * up * down;
            if current_score > score {
                score = current_score;
                location = (x, y);
            }
        }
    }

    //println!("{:?}", v);
    println!("score= {score} at {:?}", location);
}
