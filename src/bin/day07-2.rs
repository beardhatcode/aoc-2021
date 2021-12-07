use std::io::BufRead;

#[path = "../input_reader.rs"]
mod input_reader;

// Formula of gause (division posponed)
fn get_cost(v: &Vec<u32>, p: u32) -> u32 {
    let mut cost: u32 = 0;
    for pos in v {
        let m: u32 = ((p as i32) - (*pos as i32)).abs() as u32;
        cost += (m * (m + 1)) as u32
    }
    return cost / 2;
}

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

    // sort to get min and max (the rest is O(n²) anyways...)
    positions.sort();

    // no more modifications to the list
    let positions = positions;

    let mut best_pos = positions[0];
    let mut best_cost = get_cost(&positions, positions[0]);

    for i in positions[0]..=positions[positions.len() - 1] {
        let cur_cost = get_cost(&positions, i);
        if cur_cost < best_cost {
            best_cost = cur_cost;
            best_pos = i;
        }
    }

    println!("Best move to {} = {}", best_pos, best_cost);
    Ok(())
}
