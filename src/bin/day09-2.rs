use std::collections::BinaryHeap;
use std::io::BufRead;

#[path = "../input_reader.rs"]
mod input_reader;

fn grid_get(grid: &Vec<Vec<i32>>, i: i32, j: i32) -> Option<&i32> {
    if i >= 0 && j >= 0 {
        return grid.get(i as usize).and_then(|row| row.get(j as usize));
    } else {
        return None;
    }
}

fn flood_fill(grid: &mut Vec<Vec<i32>>, i: i32, j: i32) -> u32 {
    if let Some(cur_val) = grid_get(grid, i, j) {
        if cur_val == &9 || cur_val < &0 {
            return 0;
        }

        let mut cnt = 1u32;
        grid[i as usize][j as usize] = -1;

        cnt += flood_fill(grid, i + 1, j);
        cnt += flood_fill(grid, i - 1, j);
        cnt += flood_fill(grid, i, j + 1);
        cnt += flood_fill(grid, i, j - 1);

        return cnt;
    }
    return 0;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = input_reader::get_lines()?;
    let mut grid: Vec<Vec<i32>> = reader
        .lines()
        .filter_map(|l| {
            l.ok().map(|l| {
                l.chars()
                    .filter_map(|c| c.to_digit(10))
                    .map(|x| x as i32)
                    .collect()
            })
        })
        .collect();
    let w = grid.len();
    let h = grid[0].len();

    let mut basin_sizes: BinaryHeap<u32> = BinaryHeap::new();
    for i in 0..(w as i32) {
        for j in 0..(h as i32) {
            let size = flood_fill(&mut grid, i, j);
            if size != 0 {
                basin_sizes.push(size);
            }
        }
    }

    let mut result = basin_sizes.pop().unwrap_or(0);
    result *= basin_sizes.pop().unwrap_or(1);
    result *= basin_sizes.pop().unwrap_or(1);

    println!("{}", result);
    Ok(())
}
