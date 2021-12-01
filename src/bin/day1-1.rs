use std::io::{stdin, BufRead};

fn main() {
    let s = stdin();
    let mut lines = s.lock().lines();

    let mut cnt : u32 = 0;

    let mut prev : Option<u32> = None;
    while let Some(Ok(line)) = lines.next() {
        let v : Option<u32> = line.parse().ok();

        if let (Some(a), Some(b)) = (v,prev) {
            cnt += (a > b) as u32;
        }

        prev = v;
    }

    println!("{:?}", cnt);
}
