use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;


pub fn count_vowels(sentence:&str)->u32{
    sentence.chars().filter(|c|{
        match c {
            'a'|'e'|'i'|'o'|'u' => true,
            _ => false
        }
    }).count() as u32
   }
pub fn longest_word(sentence:&str)->&str{
    sentence.split_whitespace().max_by_key(|w|{
        w.len()
    }).unwrap()
   }

pub fn readfile(filename: &str) -> Result<Vec<String>, std::io::Error> {
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
