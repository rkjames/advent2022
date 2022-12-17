use std::{cmp, fs, collections::HashSet};

#[derive(Debug)]
struct Sensor {
    sx: i32,
    sy: i32,
    bx: i32,
    by: i32,
}

// inclusive range, ie. start and end are in the range.
// reminder to keep start<end
#[derive(Debug, Clone)]
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    // Returns a new range that is the intersection with other.
    // Returne None if intersection is empty.
    fn intersect(&self, other: &Range) -> Option<Range> {
        let maxstart = cmp::max(self.start, other.start);
        let minend = cmp::min(self.end, other.end);
        if maxstart <= minend {
            return Some(Range {
                start: maxstart,
                end: minend,
            });
        }
        None
    }

    fn union(&self, other: &Range) -> Range {
        // only support non-empty intersection unions.
        assert!(self.intersect(other).is_some());
        let minstart = cmp::min(self.start, other.start);
        let maxend = cmp::max(self.end, other.end);
        Range {
            start: minstart,
            end: maxend,
        }
    }
}

#[derive(Debug)]
struct RangeList {
    list: Vec<Range>,
}

impl RangeList {
    fn union(&mut self, r: &Range) {
        if self.list.len() == 0 {
            self.list.push(r.clone());
            return;
        }
        let mut tmp = r.clone();
        let mut i = 0;
        while i < self.list.len() {
            if self.list[i].intersect(&tmp).is_some() {
                // we overlap an existing entry, grow tmp to the union.
                tmp = self.list[i].union(&tmp);
                // remove the overlap, as its contained in tmp
                self.list.remove(i);
                // continue searching, as its possible more than 1 entry overlaps.
                continue;
            }
            i += 1;
        }
        self.list.push(tmp);
    }

    fn size(&self) -> usize {
        let mut count : usize = 0;
        for r in self.list.iter() {
            count += (r.start.abs_diff(r.end) + 1) as usize;
        }
        count
    }

    // returns true if intersection exists.
    fn intersects(&self, other : &Range) -> bool {
        for r in self.list.iter() {
            if r.intersect(other).is_some() {
                return true;
            }
        }
        false
    }
}

fn parse() -> Vec<Sensor> {
    let data = fs::read_to_string("test1.txt").expect("read to string failed");
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
    let mut r = RangeList { list: vec![] };
    //let rowy: i32 = 10;
    let rowy: i32 = 2000000;
    for s in data.iter() {
        let width = s.sx.abs_diff(s.bx);
        let height = s.sy.abs_diff(s.by);
        let hamilton = width + height;
        let space = rowy.abs_diff(s.sy);
        // todo: off by 1?
        if space < hamilton {
            let rwidth = (hamilton - space) as i32;
            let newrange = Range {
                start: s.sx - rwidth,
                end: s.sx + rwidth,
            };
            r.union(&newrange);
            //println!("after adding {:?} --> {:?}", newrange, r);
        }
    }
    //println!("{:?}", data);
    println!("{:?}", r);
    let mut size = r.size();
    println!("size= {}", size);
    // remove beacons in row
    let mut hash : HashSet<i32> = HashSet::new();
    for s in data.iter() {
        if s.by == rowy {
            hash.insert(s.bx);
        }
    }
    for x in hash {
        let range1 = Range {start:x, end:x};
        if r.intersects(&range1) {
            println!("intersection {:?}", x);
            size -= 1;
        }
    }
    println!("final size= {}", size);
}
