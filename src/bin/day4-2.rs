use std::collections::LinkedList;
use std::io::BufRead;

#[path = "../input_reader.rs"]
mod input_reader;

#[derive(Debug)]
struct Board {
    values: [i8; 25],
    rows: [u8; 5],
    cols: [u8; 5],
}

impl Board {
    fn stripe(&mut self, n: i8) -> bool {
        for (i, x) in self.values.into_iter().enumerate() {
            if x == n {
                let row = i / 5;
                let col = i % 5;
                self.rows[row] += 1;
                self.cols[col] += 1;
                self.values[i] = -1;

                if self.rows[row] == 5 || self.cols[col] == 5 {
                    return true;
                }
            }
        }
        false
    }

    fn sum(&self) -> u32 {
        let mut sum = 0;
        for v in self.values {
            if v > 0 {
                sum += v as u32;
            }
        }
        return sum;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = input_reader::get_lines()?;
    let mut b = reader.lines();
    let first_line = b
        .next()
        .and_then(|v| v.ok())
        .expect("We need a list of bingo numbers");
    let numbers: Vec<i8> = first_line
        .split(',')
        .map(|v| v.parse().ok().expect("Could not parse"))
        .collect();

    let mut boards: Vec<Board> = Vec::new();
    loop {
        let mut rows: Vec<i8> = Vec::new();
        for _ in 0..6 {
            b.next().and_then(|v| v.ok()).map(|w| {
                w.split_whitespace()
                    .map(|v| v.parse().ok().expect("Could not parse"))
                    .for_each(|v| rows.push(v))
            });
        }
        if rows.len() == 0 {
            break;
        } else {
            boards.push(Board {
                values: match rows.try_into() {
                    Ok(data) => data,
                    Err(_) => panic!("lol"),
                },
                rows: [0; 5],
                cols: [0; 5],
            });
        }
    }
    println!("{:?}", boards);

    let mut remaining = boards.len();
    for n in numbers {
        println!("{:?}", n);
        let mut to_remove: LinkedList<usize> = LinkedList::new();
        for (i, b) in boards.iter_mut().enumerate() {
            if b.stripe(n) {
                to_remove.push_back(i);
                remaining -= 1;
                if remaining == 0{
                    println!("{:?} END", b.sum() * (n as u32));
                    return Ok(())
                }
            }
        }
        while let Some(i) = to_remove.pop_back() {
            boards.swap_remove(i);
        }
        println!("{:?}", boards);
    }
    Ok(())
}
