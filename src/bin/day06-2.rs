use std::io::BufRead;

#[path = "../input_reader.rs"]
mod input_reader;

const MAX_AGE: usize = 8 + 1; // 0 included
type UType = u64;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = input_reader::get_lines()?;
    let mut b = reader.lines();
    let first_line = b
        .next()
        .and_then(|v| v.ok())
        .expect("We need a list of bingo numbers");
    let start_fish = first_line
        .split(',')
        .map(|v| v.parse::<UType>().ok().expect("Could not parse"));

    let mut ages: [UType; MAX_AGE] = [0; MAX_AGE];
    for fish in start_fish {
        ages[fish as usize] += 1
    }

    println!("initial {:?}", ages);

    let mut day_0_at = 0;
    for day in 1..=256 {
        // zero becomes a six
        let day_6_at = (day_0_at + 7) % MAX_AGE;
        ages[day_6_at] += ages[day_0_at];

        // Zero becomes an 8
        day_0_at = (day_0_at + 1) % MAX_AGE;

        print!("day {:<5}", day);
        let mut s = 0;
        for i in 0..MAX_AGE {
            print!(
                "\x1B[90m{}:\x1B[31m{:<12} \x1B[0m",
                i,
                ages[(day_0_at + i) % MAX_AGE]
            );
            s += ages[i];
        }
        print!(" = {:?} \n", s);
    }

    Ok(())
}
