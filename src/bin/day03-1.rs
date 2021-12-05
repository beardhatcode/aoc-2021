use std::io::BufRead;

#[path = "../input_reader.rs"]
mod input_reader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = input_reader::get_lines()?;
    let mut b = reader.lines();

    let mut counts: Vec<u32> = b
        .next()
        .and_then(|v| {
            v.ok().and_then(|c| {
                Some(
                    c.as_bytes()
                        .into_iter()
                        .map(|x| (*x == 49) as u32)
                        .collect(),
                )
            })
        })
        .expect("nead at least a number");

    let mut tot: u32 = 1;
    while let Some(Ok(l)) = b.next() {
        for (i, v) in l.as_bytes().into_iter().enumerate() {
            counts[i] += (*v == 49) as u32
        }
        tot += 1;
    }

    println!("{:?} {:?}", counts, tot);
    let tot = tot;
    let gamma = counts
        .iter()
        .fold(0, |acc, v| (2 * acc) + ((*v > tot / 2) as u32));
    let epsilon = counts
        .iter()
        .fold(0, |acc, v| (2 * acc) + ((*v <= tot / 2) as u32));
    println!("{:?}", gamma * epsilon);

    Ok(())
}
