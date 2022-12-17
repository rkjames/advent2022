use std::fs;

#[derive(Debug)]
struct Sensor {
    sx: i32,
    sy: i32,
    bx: i32,
    by: i32,
}

fn parse() -> Vec<Sensor> {
    let data = fs::read_to_string("example1.txt").expect("read to string failed");
    let lines: Vec<&str> = data.split("\n").collect();
    let mut v: Vec<Sensor> = Vec::new();
    for line in lines {
        let scolon: Vec<&str> = line.trim().split(":").collect();
        let numhunt = |c: char| c.is_numeric() || c == '-';

        let scomma: Vec<&str> = scolon[0].trim().split(",").collect();
        let i = scomma[0].find(numhunt).unwrap();
        let sx: i32 = scomma[0][i..].parse().unwrap();
        let i = scomma[1].find(numhunt).unwrap();
        let sy: i32 = scomma[1][i..].parse().unwrap();

        let scomma: Vec<&str> = scolon[1].trim().split(",").collect();
        let i = scomma[0].find(numhunt).unwrap();
        let bx: i32 = scomma[0][i..].parse().unwrap();
        let i = scomma[1].find(numhunt).unwrap();
        let by: i32 = scomma[1][i..].parse().unwrap();

        v.push(Sensor { sx, sy, bx, by });
    }
    v
}

fn main() {
    let data = parse();
    println!("{:?}", data);
}
