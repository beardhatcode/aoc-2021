use std::io::{stdin, BufRead};

fn main() {
    let s = stdin();
    let lines = s.lock().lines();
    let mut b = lines.filter_map(|l| l.ok().and_then(|v| (v.parse::<u32>()).ok()));

    let win = 3;
    let mut meas_prev: u32 = b.next().unwrap();
    let mut hist: [u32; 3] = [0; 3]; // todo how to constant?
    for i in 0..hist.len() {
        hist[i] = b.next().unwrap();
    }
    let mut meas_next: u32 = hist[..].iter().sum();
    meas_prev += meas_next - hist[win - 1];

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

        cur = (cur + 1) % win
    }

    if meas_next > meas_prev {
        increases += 1;
    }

    println!("{:?}", increases);
}
