use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn readfile(filename: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(filename).and_then(|f|{
        let reader = BufReader::new(f);
        let mut lines: Vec<String> = Vec::new();
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    println!("line {}", line);
                    lines.push(line);
                }
                Err(error) => {
                  return  Err(error)
                }
            }
           
        }
        return Ok(lines);
    });
    return file;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a file name as an argument.");
    }
    let fileName = &args[1];

    println!("My path is {}.", fileName);

    match readfile(fileName){
        Ok(lines) => {
            for line in lines {
                println!("text= {}", line);
            }
        },
        Err(error) => {
            println!("error {}", error);
        }
    }
}