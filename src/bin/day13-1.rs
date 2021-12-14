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
    println!("{:?}", points);

    if let Some(Ok(instr)) = b.next() {
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
    points.sort();
    println!("{:?}", points);

    let mut it = points.iter();
    let mut prev = it.next().unwrap();
    let mut cnt = 1;
    for p in it {
        if p != prev {
            cnt += 1;
            prev = p;
        }
    }

    println!("{:?}", cnt);

    Ok(())
}
