use std::io::BufRead;

#[path = "../input_reader.rs"]
mod input_reader;

fn grid_get(grid: &Vec<Vec<u32>>, i: i32, j: i32) -> Option<&u32> {
    if i >= 0 && j >= 0 {
        return grid.get(i as usize).and_then(|row| row.get(j as usize));
    } else {
        return None;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = input_reader::get_lines()?;
    let grid: Vec<Vec<u32>> = reader
        .lines()
        .filter_map(|l| {
            l.ok().map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10))
                    .filter_map(|x| x)
                    .collect::<Vec<u32>>()
            })
        })
        .collect();
    let w = grid.len();
    let h = grid[0].len();

    let mut risk_level = 0u32;
    for i in 0..(w as i32) {
        for j in 0..(h as i32) {
            if let Some(v) = grid_get(&grid, i, j) {
                if let Some(o) = grid_get(&grid, i - 1, j) {
                    if o <= v {
                        continue;
                    }
                }

                if let Some(o) = grid_get(&grid, i + 1, j) {
                    if o <= v {
                        continue;
                    }
                }

                if let Some(o) = grid_get(&grid, i, j - 1) {
                    if o <= v {
                        continue;
                    }
                }
                if let Some(o) = grid_get(&grid, i, j + 1) {
                    if o <= v {
                        continue;
                    }
                }

                risk_level += v + 1;
            }
        }
    }

    println!("{:?}", risk_level);

    Ok(())
}
