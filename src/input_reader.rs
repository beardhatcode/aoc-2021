use std::env;
use std::io::BufReader;
use std::fs::File;

pub fn get_lines() -> Result<BufReader<Box<dyn std::io::Read>>, std::io::Error>{
    let args: Vec<String> = env::args().collect();

    let file : Box<dyn std::io::Read> = match &args[..] {
        [_, filename] => Box::new(File::open(filename)?),
        _ => Box::new(std::io::stdin()),
    };

    let reader : BufReader<Box<dyn std::io::Read>>= BufReader::new(file);
    Ok(reader)

}
