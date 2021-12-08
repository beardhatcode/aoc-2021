use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr;

#[path = "../input_reader.rs"]
mod input_reader;

#[derive(Debug, PartialEq)]
struct SegInput {
    wire: [u8; 10], // bin numbers
    result: [u8; 4],
}

fn seg_to_bin(segw: &str) -> u8 {
    let bits: HashMap<char, u8> = HashMap::from([
        ('a', 1),
        ('b', 2),
        ('c', 4),
        ('d', 8),
        ('e', 16),
        ('f', 32),
        ('g', 64),
    ]);

    let mut n = 0;
    for c in segw.trim().chars() {
        n = n | bits.get(&c).unwrap_or(&0);
    }
    n
}

impl FromStr for SegInput {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to): (&str, &str) = s
            .trim()
            .split_once(" | ")
            .ok_or("Could not find \" | \" in line")?;

        let parts: Vec<u8> = from.split(" ").map(seg_to_bin).collect();
        let parts_result: Vec<u8> = to.split(" ").map(seg_to_bin).collect();

        Ok(SegInput {
            wire: match parts.try_into() {
                Ok(data) => data,
                Err(_) => panic!("Incorrect number of segments in first part"),
            },
            result: match parts_result.try_into() {
                Ok(data) => data,
                Err(_) => panic!("Incorrect number of segments in second part"),
            },
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = input_reader::get_lines()?;
    let mut b = reader.lines();

    let mut cnt = 0;
    while let Some(Ok(l)) = b.next() {
        let line: SegInput = l.parse()?;
        let res = line.result;
        for segment in res {
            cnt += match segment.count_ones() {
                2 => 1, // 1
                4 => 1, // 1
                3 => 1, // 1
                7 => 1, // 1
                _ => 0,
            }
        }
    }
    println!("{:?}", cnt);

    Ok(())
}
