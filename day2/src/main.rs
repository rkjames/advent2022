use std::fs;

fn failfast(msg :&str) {
    println!("failfast {msg}");
    std::process::abort();
}

// todo: learn enums
fn me_score(opp : i32, me : i32) -> i32 {
    if opp == me {
        return 3;
    }
    if opp == 1 && me == 2 {
        // rock, paper
        return 6;
    }
    if opp == 2 && me == 3 {
        // paper, scissors
        return 6;
    }
    if opp == 3 && me == 1 {
        // scissors, rock
        return 6;
    }
    0
}

fn me_choice(opp : i32, me : i32) -> i32 {
    if me == 2 {
        // draw
        return opp;
    }

    if me == 1 {
        // lose
        match opp {
            1 => return 3,
            2 => return 1,
            3 => return 2, 
            _ => failfast("me_choice lose") 
        }
    }

    // win
    match opp {
        1 => return 2,
        2 => return 3,
        3 => return 1,     
        // todo: why can't i failfast here?       
        _ => 999999
    }

}

fn main() {
    println!("Hello, world!");
    let data = fs::read_to_string("test1.txt").expect("read to string failed");
    let lines: Vec<&str> = data.split("\n").collect();
    let mut opp : Vec<i32> = Vec::new();
    let mut me : Vec<i32> = Vec::new();
    println!("{}", lines.len());
    for line in &lines {
        let s : Vec<&str> = line.trim().split(" ").collect();
        match s[0] {
            "A" => opp.push(1),
            "B" => opp.push(2),
            "C" => opp.push(3),
            _ => failfast("opponent")
        }
        match s[1] {
            "X" => me.push(1),
            "Y" => me.push(2),
            "Z" => me.push(3),
            _ => failfast("me")
        }
    }

    let mut me_total = 0;
    for i in 0..opp.len() {
        let choose = me_choice(opp[i], me[i]);
        let round = me_score(opp[i], choose) + choose;
        println!("{round}");
        me_total += round;
    }

    //println!("{:?}", opp);
    //println!("{:?}", me);
    println!("{me_total}");
}
