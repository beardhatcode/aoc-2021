use std::cmp::max;
use std::io::BufRead;
use std::str::FromStr;

#[path = "../input_reader.rs"]
mod input_reader;

const GRIDSIZE: usize = 1000;
const GRIDSIZE_U: u32 = GRIDSIZE as u32;

#[derive(Debug, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq)]
struct Line {
    from: Point,
    to: Point,
}

impl FromStr for Point {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim().split(',').collect();

        let x_fromstr = coords[0].parse().expect("Could not parse x coord");
        let y_fromstr = coords[1].parse().expect("Could not parse y coord");

        if x_fromstr < GRIDSIZE_U && y_fromstr < GRIDSIZE_U {
            Ok(Point {
                x: x_fromstr,
                y: y_fromstr,
            })
        } else {
            panic!("out of bounds {} ", s);
        }
    }
}

impl FromStr for Line {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to): (&str, &str) = s
            .trim()
            .split_once("->")
            .ok_or("Could not find \"->\" in line")?;

        Ok(Line {
            from: from.trim().parse()?,
            to: to.trim().parse()?,
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = input_reader::get_lines()?;
    let mut b = reader.lines();

    let mut grid: Vec<Vec<u32>> = vec![vec![0; GRIDSIZE]; GRIDSIZE];
    let mut overlaps = 0;
    while let Some(Ok(l)) = b.next() {
        let line: Line = l.parse()?;
        if line.from.x == line.to.x || line.from.y == line.to.y {
            let mut init_x = line.from.x as i32;
            let dx: i32 = line.to.x as i32 - line.from.x as i32;
            let sx: i32 = dx.signum();

            let mut init_y = line.from.y as i32;
            let dy = line.to.y as i32 - line.from.y as i32;
            let sy: i32 = dy.signum();

            for _ in 0..=max(dx.abs(), dy.abs()) {
                grid[init_x as usize][init_y as usize] += 1;
                if grid[init_x as usize][init_y as usize] == 2 {
                    overlaps += 1;
                }
                init_x += sx;
                init_y += sy;
            }
        }
    }

    println!("{:?} overlaps", overlaps);
    Ok(())
}
