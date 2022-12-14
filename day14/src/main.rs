use std::fs;

enum Space {
    Air,
    Rock,
    Sand,
}

fn parse() -> Vec<Vec<Space>> {
    let data = fs::read_to_string("example1.txt").expect("read to string failed");
    let lines: Vec<&str> = data.split("\n").collect();
    let mut v: Vec<Vec<Space>> = Vec::new();
    for _x in 0..600 {
        let mut row : Vec<Space> = Vec::new();
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
            let (x1,y1) = path[i-1];
            let (x2,y2) = path[i];
            if x1 == x2 {
                for y in y1..=y2 {
                    v[x1][y] = Space::Rock;
                }
            } else {
                for x in x1..=x2 {
                    v[x][y1] = Space::Rock;
                }
            }
        }
    }
    v
}
fn main() {
    let map = parse();
    println!("{}", map.len());
}
