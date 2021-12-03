use std::io::{BufRead, BufReader};

#[path = "../input_reader.rs"]
mod input_reader;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader : BufReader<Box<dyn std::io::Read>>= input_reader::get_lines()?;
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
