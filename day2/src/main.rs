use std::fs;

#[derive(Debug, PartialEq, Copy, Clone)] 
enum Move {
    Rock=1,
    Paper=2,
    Scissors=3
}

#[derive(Debug, PartialEq, Copy, Clone)] 
enum Goal {
    Lose,
    Draw,
    Win
}

fn failfast(msg :&str) {
    println!("failfast {msg}");
    std::process::abort();
}

fn me_score(opp : Move, me : Move) -> i32 {
    if opp == me {
        return 3;
    }
    if opp == Move::Rock && me == Move::Paper {
        return 6;
    }
    if opp == Move::Paper && me == Move::Scissors {
        return 6;
    }
    if opp == Move::Scissors && me == Move::Rock {
        return 6;
    }
    0
}

fn me_choice(opp : Move, me : Goal) -> Move {
    if me == Goal::Draw {
        return opp;
    }

    if me == Goal::Lose {
        match opp {
            Move::Rock => return Move::Scissors,
            Move::Paper => return Move::Rock,
            Move::Scissors => return Move::Paper,
        }
    }

    // win
    match opp {
        Move::Rock => return Move::Paper,
        Move::Paper => return Move::Scissors,
        Move::Scissors => return Move::Rock,
    }
}

fn main() {
    println!("Hello, world!");
    let data = fs::read_to_string("example1.txt").expect("read to string failed");
    let lines: Vec<&str> = data.split("\n").collect();
    let mut opp : Vec<Move> = Vec::new();
    let mut me : Vec<Goal> = Vec::new();
    println!("{}", lines.len());
    for line in &lines {
        let s : Vec<&str> = line.trim().split(" ").collect();
        match s[0] {
            "A" => opp.push(Move::Rock),
            "B" => opp.push(Move::Paper),
            "C" => opp.push(Move::Scissors),
            _ => failfast("opponent")
        }
        match s[1] {
            "X" => me.push(Goal::Lose),
            "Y" => me.push(Goal::Draw),
            "Z" => me.push(Goal::Win),
            _ => failfast("me")
        }
    }

    let mut me_total = 0;
    for i in 0..opp.len() {
        let choose = me_choice(opp[i], me[i]);
        let round = me_score(opp[i], choose) + (choose as i32);
        me_total += round;
    }

    println!("{me_total}");
}
