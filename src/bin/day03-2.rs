use std::io::BufRead;

#[path = "../input_reader.rs"]
mod input_reader;

fn most_common(data: &Vec<u32>, mask: u32) -> bool {
    let num_one_bits = data.iter().filter(|x| (*x & mask != 0)).count();
    return 2 * num_one_bits >= data.len(); // 1 if tie
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = input_reader::get_lines()?;
    let mut b = reader.lines().peekable();

    let numbits = b
        .peek()
        .and_then(|v| v.as_ref().ok().and_then(|v| Some(v.len())))
        .expect("We need at least one value");

    let mut curbit = 1 << numbits - 1;

    let numbers: Vec<u32> = b
        .filter_map(|v| v.as_ref().ok().and_then(|v| u32::from_str_radix(v, 2).ok()))
        .collect();

    let mut most = numbers.clone();
    let mut least = numbers.clone();
    let mut most_final: Result<u32, _> = Err("Could not find most");
    let mut least_final: Result<u32, _> = Err("Could not find least");
    while curbit != 0 {
        let most_common_bit = most_common(&most, curbit);
        most.retain(|n| (*n & curbit != 0) == most_common_bit);

        let least_common_bit = !most_common(&least, curbit);
        least.retain(|n| (*n & curbit != 0) == least_common_bit);

        if least.len() == 1 {
            least_final = Ok(least[0])
        }

        if most.len() == 1 {
            most_final = Ok(most[0])
        }
        curbit = curbit >> 1;
    }
    println!(
        "{} = {} most * {} least",
        most_final? * least_final?,
        most_final?,
        least_final?
    );
    Ok(())
}
