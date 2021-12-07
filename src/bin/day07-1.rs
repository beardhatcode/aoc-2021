use std::io::BufRead;

#[path = "../input_reader.rs"]
mod input_reader;

// I should have done this by just using the mean, but my brain was not working that early
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = input_reader::get_lines()?;
    let mut positions: Vec<u32> = reader
        .lines()
        .next()
        .and_then(|v| v.ok())
        .expect("Give a list of positions")
        .split(',')
        .map(|v| v.parse::<u32>().ok().expect("Could not parse"))
        .collect();

    // Sort the positions so we can calculate the cost dynamically
    positions.sort();
    let positions = positions;

    // Keep track of numbers higher lower and same
    let mut num_lower: u32 = 0;
    let mut num_higher: u32 = 0;
    let mut num_same: u32 = 0;

    // Calculate for first position
    let mut cost: u32 = 0;
    for i in 0..positions.len() {
        if positions[i] == positions[0] {
            num_same += 1
        } else {
            num_higher += 1;
            cost += positions[i] - positions[0];
        }
    }

    let mut best_pos = positions[0];
    let mut best_cost = cost;

    // Move to after the first (and all with the same value)
    num_lower += num_same;
    let start_i: usize = num_same as usize;
    num_same = 0;

    // Start with next
    for i in start_i..positions.len() {
        num_same += 1;
        num_higher -= 1;

        // Keep collecting while the next is the same
        if i + 1 < positions.len() && positions[i + 1] == positions[i] {
            continue; // collect all num_same
        }

        // Get the movement size
        let steps: u32 = positions[i] - positions[i - (num_same as usize)];

        // Add one for every lower, substract one for every higher
        cost = cost + num_lower * steps - (num_same + num_higher) * steps;

        if cost < best_cost {
            best_pos = positions[i];
            best_cost = cost;
        }

        num_lower += num_same;
        num_same = 0;
    }

    println!("Best move to {} = {}", best_pos, best_cost);

    Ok(())
}
