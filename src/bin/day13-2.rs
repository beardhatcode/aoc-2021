use std::cmp::max;
use std::io::BufRead;
use std::str::FromStr;

#[path = "../input_reader.rs"]
mod input_reader;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: u32,
    y: u32,
}

impl FromStr for Point {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim().split(',').collect();

        let x_fromstr = coords[0].parse().expect("Could not parse x coord");
        let y_fromstr = coords[1].parse().expect("Could not parse y coord");

        Ok(Point {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = input_reader::get_lines()?;
    let mut b = reader.lines();

    let mut points: Vec<Point> = Vec::new();
    while let Some(Ok(l)) = b.next() {
        if l.trim().len() == 0 {
            break;
        }
        points.push(l.parse()?);
    }

    while let Some(Ok(instr)) = b.next() {
        let instr = &instr[("fold along ".len())..];
        let direction = instr.chars().next().unwrap();
        let pos: u32 = instr[2..].parse()?;
        println!("{} {}", direction, pos);

        if direction == 'y' {
            for mut p in &mut points {
                if p.y > pos {
                    p.y = pos - (p.y - pos);
                }
            }
        } else {
            for mut p in &mut points {
                if p.x > pos {
                    p.x = pos - (p.x - pos);
                }
            }
        }
    }

    let mut max_x = 0;
    let mut max_y = 0;
    for p in &points {
        max_x = max(max_x, p.x);
        max_y = max(max_y, p.y);
    }

    // Render dost
    let mut data = vec![vec![' '; 1 + max_x as usize]; 1 + max_y as usize];
    for p in points {
        data[p.y as usize][p.x as usize] = '#';
    }

    for l in data {
        for c in l {
            print!("{}", c);
        }
        print!("\n");
    }

    Ok(())
}
