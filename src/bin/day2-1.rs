use std::io::BufRead;

#[path = "../input_reader.rs"]
mod input_reader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = input_reader::get_lines()?;
    let b = reader.lines().filter_map(|l| {
        l.ok().and_then(|v: String| {
            if let Some((dir, num)) = v.split_once(' ') {
                if let Ok(v) = num.parse::<i32>() {
                    match dir {
                        "forward" => Some((v, 0)),
                        "up" => Some((0, -v)),
                        "down" => Some((0, v)),
                        _ => None,
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
    });

    let mut cur = (0i32, 0i32);
    for (x, y) in b {
        cur.0 += x;
        cur.1 += y;
        println!("{:?}", cur);
    }

    println!("{:?}", cur.0 * cur.1);

    Ok(())
}
