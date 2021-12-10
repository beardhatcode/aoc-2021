use std::io::BufRead;
use std::collections::HashMap;

#[path = "../input_reader.rs"]
mod input_reader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = input_reader::get_lines()?.lines();

    let friends : HashMap<char, char> = HashMap::from([
        ('(', ')'),
        ('<', '>'),
        ('[', ']'),
        ('{', '}'),
    ]);

    let points: HashMap<char, u32> = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);

    let mut score = 0;
    while let Some(Ok(l)) = reader.next() {
        //println!("{:?}", l);
        let mut bracket_stack: Vec<char> = Vec::new();
        for c in l.chars() {
            if let Some(other) = friends.get(&c) {
                // Opening brack
                bracket_stack.push(*other);
            } else {
                // closing brac
                let expected = bracket_stack.pop().unwrap_or('n');
                if c != expected {
                    //println!("WRONG got {} expected {}      scoring {}", c ,expected, points[&c]);
                    score += points[&c];
                }

            }
        }
    }

    println!("{}", score);

    Ok(())
}
