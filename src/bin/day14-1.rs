use std::collections::HashMap;
use std::hash::Hash;
use std::io::BufRead;
use std::ops::Deref;
use std::str::FromStr;

#[path = "../input_reader.rs"]
mod input_reader;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Line {
    from: [char; 2],
    to: char,
}

impl FromStr for Line {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to): (&str, &str) = s
            .trim()
            .split_once("->")
            .ok_or("Could not find \"->\" in line")?;

        Ok(Line {
            from: match from.trim().chars().collect::<Vec<char>>().try_into() {
                Ok(data) => data,
                Err(_) => panic!("lol"),
            },
            to: to.trim().chars().next().unwrap(),
        })
    }
}

/**
 * Increment the count of key with v (insert new if not exists)
 **/
fn inc_with<K: Hash + Eq>(counts: &mut HashMap<K, u64>, key: K, v: u64) {
    match counts.get_mut(&key) {
        Some(x) => *x = *x + v,
        None => {
            counts.insert(key, v);
        }
    }
}

/**
 * Count the number of occurences of each char in the iteratable
 **/
// TODO: use IntoIter instread of Deref of [T] to get iter
fn count_pairs(pattern: &dyn Deref<Target = [char]>) -> HashMap<[char; 2], u64> {
    let mut pairs: HashMap<[char; 2], u64> = HashMap::new();

    let mut it = pattern.iter();

    let mut prevchar = it.next().expect("need at least one character");
    while let Some(nextchar) = it.next() {
        let v: [char; 2] = [*prevchar, *nextchar];
        inc_with(&mut pairs, v, 1);
        prevchar = nextchar;
    }
    return pairs;
}

/**
 * Transforms a map of pairs to counts by applying the right action.
 *
 * For (A,B) -> C, (that is AB becomes ACB)
 * the counts of A and B remain, but the count of C increases with the number of occurences of AB.
 * The number of (A,C) and (C,B) is the amount of (A,B) in the input. Because other transformations
 * may also insert (C,B) (like (X,B)->B) we need to increment and not just set the count. To keep
 * track of the old and new counts, 2 separate maps are needed.
 *
 * To quickly find the transformation to be applied, the list must be sorted by from
 **/
fn transform(
    pairs: &mut HashMap<[char; 2], u64>, // num occurences per pair of characters
    counts: &mut HashMap<char, u64>,     // counts of individual letters
    actions: &Vec<Line>,                 // sorted transformations
) {
    let mut pairs_new: HashMap<[char; 2], u64> = HashMap::new();
    for (key, val) in &*pairs {
        let found = actions.binary_search_by_key(&key, |v| &v.from);
        match found {
            Ok(pos) => {
                let new_char: char = actions[pos].to.to_owned();
                inc_with(&mut pairs_new, [key[0], new_char], *val);
                inc_with(&mut pairs_new, [new_char, key[1]], *val);
                inc_with(counts, new_char, *val);
            }
            Err(_) => {
                // does not change
                inc_with(&mut pairs_new, *key, *val);
            }
        }
    }
    *pairs = pairs_new;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = input_reader::get_lines()?;
    let mut b = reader.lines();

    let pattern: Vec<char> = b.next().unwrap().unwrap().chars().collect();
    b.next(); // skip empty line

    let mut counts: HashMap<char, u64> = HashMap::new();
    for c in &pattern {
        inc_with(&mut counts, *c, 1);
    }

    let mut actions = Vec::new();
    while let Some(Ok(l)) = b.next() {
        let line: Line = l.parse()?;
        actions.push(line);
    }

    // sort actions for quick retrival of the right transform
    actions.sort_by_key(|v| v.from);

    let mut pairs = count_pairs(&pattern);
    for _ in 0..10 {
        transform(&mut pairs, &mut counts, &actions);
    }

    // get result
    let mut counts_only: Vec<&u64> = counts.values().collect();
    counts_only.sort();
    println!("{:?}", counts_only[counts_only.len() - 1] - counts_only[0]);
    Ok(())
}
