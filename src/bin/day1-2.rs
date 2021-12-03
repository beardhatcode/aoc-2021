use std::io::BufRead;

#[path = "../input_reader.rs"]
mod input_reader;


const WIN : usize = 3;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = input_reader::get_lines()?;
    let mut b = reader
        .lines()
        .filter_map(|l| l.ok().and_then(|v| (v.parse::<u32>()).ok()));

    let mut meas_prev: u32 = b.next().unwrap();
    let mut hist: [u32; WIN] = [0; WIN];
    for i in 0..hist.len() {
        hist[i] = b.next().unwrap();
    }
    let mut meas_next: u32 = hist[..].iter().sum();
    meas_prev += meas_next - hist[WIN - 1];

    let mut cur = 0;
    let mut increases = 0;
    for n in b {
        if meas_next > meas_prev {
            increases += 1;
        }

        //Shift out value
        meas_prev = meas_next;
        meas_next -= hist[cur];
        hist[cur] = n;
        meas_next += hist[cur];

        cur = (cur + 1) % WIN;
    }

    if meas_next > meas_prev {
        increases += 1;
    }

    println!("{:?}", increases);
    Ok(())
}
