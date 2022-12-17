use std::fs;

#[derive(PartialEq)]
enum Space {
    Air,
    Rock,
    Sand,
}

fn parse() -> Vec<Vec<Space>> {
    let data = fs::read_to_string("test1.txt").expect("read to string failed");
    let lines: Vec<&str> = data.split("\n").collect();
    let mut v: Vec<Vec<Space>> = Vec::new();
    for _x in 0..600 {
        let mut row: Vec<Space> = Vec::new();
        for _y in 0..200 {
            row.push(Space::Air);
        }
        v.push(row);
    }
    for line in lines {
        let mut path: Vec<(usize, usize)> = Vec::new();
        let line_split: Vec<&str> = line.split("->").collect();
        for point in line_split {
            let point_split: Vec<&str> = point.trim().split(",").collect();
            let x: usize = point_split[0].parse().unwrap();
            let y: usize = point_split[1].parse().unwrap();
            path.push((x, y));
        }
        for i in 1..path.len() {
            let (x1, y1) = path[i - 1];
            let (x2, y2) = path[i];
            //println!("path {x1},{y1}->{x2},{y2}");
            if x1 == x2 {
                let miny = std::cmp::min(y1, y2);
                let maxy = std::cmp::max(y1, y2);
                for y in miny..=maxy {
                    v[x1][y] = Space::Rock;
                }
            } else {
                let minx = std::cmp::min(x1, x2);
                let maxx = std::cmp::max(x1, x2);
                for x in minx..=maxx {
                    v[x][y1] = Space::Rock;
                }
            }
        }
    }
    v
}

fn print(v: &Vec<Vec<Space>>) {
    for y in 0..50 {
        for x in 450..550 {
            if x == 500 && y == 0 {
                print!("+");
            } else {
                let c = match v[x][y] {
                    Space::Air => '.',
                    Space::Rock => '#',
                    Space::Sand => 'O',
                };
                print!("{c}");
            }
        }
        println!("");
    }
}
fn main() {
    let mut map = parse();
    //print(&map);

    // example should take 24 iterations
    'grains: loop {
        // The sand is pouring into the cave from point 500,0.
        let mut x = 500;
        let mut y = 0;

        loop {
            if y >= 199 {
                // off the bottom
                //println!("break bottom");
                break 'grains;
            }

            if map[x][y + 1] == Space::Air {
                // A unit of sand always falls down one step if possible.
                y += 1;
                continue;
            }
            if map[x - 1][y + 1] == Space::Air {
                // the unit of sand attempts to instead move diagonally one step down and to the left.
                y += 1;
                x -= 1;
                continue;
            }
            if map[x + 1][y + 1] == Space::Air {
                // the unit of sand attempts to instead move diagonally one step down and to the right.
                y += 1;
                x += 1;
                continue;
            }

            if map[x][y] == Space::Air {
                // at rest
                map[x][y] = Space::Sand;
                //println!("sand");
            } else {
                // out of sand spots
                //println!("break grains");
                break 'grains;
            }
            //println!("break classique");
            break;
        }
    }
    let mut count: i32 = 0;
    for r in map.iter() {
        for c in r {
            if *c == Space::Sand {
                count += 1;
            }
        }
    }
    println!("{count}");
    print(&map);
}
