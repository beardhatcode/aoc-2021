use std::collections::HashMap;
use std::io::BufRead;

#[path = "../input_reader.rs"]
mod input_reader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = input_reader::get_lines()?.lines();

    let friends: HashMap<char, char> =
        HashMap::from([('(', ')'), ('<', '>'), ('[', ']'), ('{', '}')]);

    let points: HashMap<char, u64> = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);

    let mut scores: Vec<u64> = Vec::new();
    while let Some(Ok(l)) = reader.next() {
        //println!("{:?}", l);
        let mut bracket_stack: Vec<char> = Vec::new();
        let mut valid = true;
        for c in l.chars() {
            if let Some(other) = friends.get(&c) {
                // Opening brack
                bracket_stack.push(*other);
            } else {
                // closing brac
                let expected = bracket_stack.pop().unwrap_or('n');
                if c != expected {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            let mut correction_score: u64 = 0;
            while let Some(close) = bracket_stack.pop() {
                //println!(" add a {}", close);
                correction_score *= 5;
                correction_score += points[&close];
            }
            //println!("     -> {}", correction_score);
            scores.push(correction_score);
        }
    }

    scores.sort();
    println!("{:?}", scores[scores.len() / 2]);

    Ok(())
}
