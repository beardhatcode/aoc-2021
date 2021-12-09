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
        n = n | bits.get(&c).expect("Could not parse segment");
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

// The variables are named as follows.
// Variable names with multiple are the or
//  aaaa
// b    c
// b    c
//  dddd
// e    f
// e    f
//  gggg

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = input_reader::get_lines()?;
    let mut b = reader.lines();

    let mut total = 0;
    while let Some(Ok(l)) = b.next() {
        let line: SegInput = l.parse()?;
        let mut cf: Option<u8> = None;
        let mut bcdf: Option<u8> = None;
        let mut acf: Option<u8> = None;
        let mut cde = 0;
        let abcdefg: u8 = 127;
        let mut adg: u8 = 127;
        for segment in line.wire {
            match segment.count_ones() {
                2 => cf = Some(segment),   // 1
                4 => bcdf = Some(segment), // 4
                3 => acf = Some(segment),  // 7
                6 => {
                    // 0, 6 or 9
                    // Collect the holes in cde
                    cde = 127 & (cde | (!segment))
                }
                5 => {
                    // 5 or 2 or 3
                    adg = adg & segment;
                }
                7 => (), // 8 -> leans us nothing
                _ => (),
            }
        }
        let cf = cf.expect("Did not find 1");
        let bcdf = bcdf.expect("Did not find 4");
        let acf = acf.expect("Did not find 7");
        let cde = cde;
        let adg = adg;
        //let bd = bcdf & (!cf);
        let a = acf & (!cf);
        let c = cf & cde;
        //let f = acf ^ (c | a);
        let eg = abcdefg ^ (bcdf | acf);
        let e = eg & cde;
        let g = eg & (!e);
        let d = adg & (!g) & (!a);
        //let b = bd & (!d);
        //println!("  ┌   {:5<}   ┐  ", a);
        //println!("{:5<}       {:5<}", b, c);
        //println!("  ├   {:5<}   ┤  ", d);
        //println!("{:5<}       {:5<}", e, f);
        //println!("  └   {:5<}   ┘  ", g);

        let mut value: u32 = 0;
        for segment in line.result {
            let num: u8 = match segment.count_ones() {
                2 => 1, // 1
                4 => 4, // 4
                3 => 7, // 7
                6 =>
                // 0, 6 or 9
                {
                    if (segment & d) == 0 {
                        0
                    } else {
                        if segment & c == 0 {
                            6
                        } else {
                            9
                        }
                    }
                }
                5 => {
                    if segment & c == 0 {
                        5
                    } else {
                        if segment & e == 0 {
                            3
                        } else {
                            2
                        }
                    }
                }
                7 => 8,
                _ => panic!("kk"),
            };
            value = value * 10 + (num as u32);
        }
        println!("{:?}", value);
        total += value;
    }

    println!("{:?} total", total);
    Ok(())
}
