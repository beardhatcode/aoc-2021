use std::io::{stdin, BufRead};

fn main() {
    let s = stdin();
    let lines = s.lock().lines();
    let mut b = lines.filter_map(|l| l.ok().and_then(|v| (v.parse::<u32>()).ok()));
    let first = b.next().unwrap();
    let (_, inc) = b.fold((first, 0), |(prev, inc), x| (x, ((x > prev) as u32) + inc));
    println!("{:?}", inc);
}
