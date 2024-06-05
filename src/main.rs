use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;


fn count_vowels(sentence:&str)->u32{
    sentence.chars().filter(|c|{
        match c {
            'a'|'e'|'i'|'o'|'u' => true,
            _ => false
        }
    }).count() as u32
   }
   fn longest_word(sentence:&str)->&str{
    sentence.split_whitespace().max_by_key(|w|{
        w.len()
    }).unwrap()
   }

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
    println!("Vowels {}",count_vowels("Hello World"));
    println!("Longest {}",longest_word("Hello World Community jazz"));
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a file name as an argument.");
    }
    let fileName = &args[1];

    println!("My path is {}.", fileName);

    match readfile(fileName){
        Ok(lines) => {
            for line in lines {
               // let text = line.as_str();
                              count_vowels(line.as_str());
            }
        },
        Err(error) => {
            println!("error {}", error);
        }
    }
}