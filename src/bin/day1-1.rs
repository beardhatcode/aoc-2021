use std::env;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = env::args().collect();

    let file : Box<dyn std::io::Read> = match &args[..] {
        [_, filename] => Box::new(File::open(filename)?),
        _ => Box::new(std::io::stdin()),
    };

    let reader = BufReader::new(file);
    let mut lines = reader.lines();

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
    Ok(())
}
