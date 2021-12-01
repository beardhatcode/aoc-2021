use std::io::{stdin, BufRead};

fn main() {
    let s = stdin();
    let lines = s.lock().lines();
    let mut b = lines.map(|l| {
        l.ok()
            .and_then(|v| (v.parse::<u32>().ok()))
            .expect("Could not line")
    });
    let first = b.next().expect("Need at least one value");

    let (_, inc) = b.fold((first, 0), |(prev, inc), x| (x, ((x > prev) as u32) + inc));
    println!("{:?}", inc);
}
