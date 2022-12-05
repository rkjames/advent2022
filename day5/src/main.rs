fn main() {
    println!("Hello, world!");
    let mut v : Vec<i32> = Vec::new();
    v.push(1);
    let mut v2 : Vec<i32> = Vec::new();
    v2.push(2);
    v2.push(3);
    let mut s : Vec<Vec<i32>> = Vec::new();
    s.push(v);
    s.push(v2);
    println!("{:?}", s[1][1]);
    
}
