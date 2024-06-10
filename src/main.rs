use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;
mod ios;
use enums::to_bytes;
use ios::count_vowels;
use ios::longest_word;
mod enums;
use std::convert::TryInto;


fn birthday(s: &[i32], d: i32, m: i32) -> i32 {
    //m size partition
    //brute check sum
    //early exists
    let len:i32 = s.len().try_into().unwrap();
    let mu:usize = m.try_into().unwrap();
     match m  {
        1 => {
            let v:i32 =  s.into_iter().filter(|i| **i==d).count().try_into().unwrap();
            return v
        } ,
        n if n <= len  => {
            let mut possiblities = 0;
            print!("counting");
            for i in 0..=(s.len()-mu){
                let end:usize = (i+mu-1).try_into().unwrap();
                let sum = s[i..=end].iter().sum::<i32>();
                print!("{}=  from {} to {} end",sum,i,end);
                if(sum == d){
                 possiblities +=1;
                 print!("found");
                }
               
            }
            return possiblities;
        },
        _=> return 1
}
}

fn main() {
    println!("Hello, world!");
    println!("Vowels {}",ios::count_vowels("Hello World"));
    println!("Longest {}",longest_word("Hello World Community jazz"));
    println!("file size format 100_000_005_000_000 {}",enums::format_size(100_002_000_000_000_000));
    println!("file size format 2500 bytes {}",enums::format_size(2500));
    enums::dispay_sizes(enums::to_bytes(1360000, "kb"));
    println!("birthdat {} = 1",birthday(&[1,2,3,4,5,6,7,8,9,10],10,1));
    println!("birthdat {} = 3",birthday(&[1,2,3,4,5,6,7,8,9,10],10,2));

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2months() {
        assert_eq!(birthday(&[1,2,3],3, 2), 1);
    }

    #[test]
    fn test_1month() {
        assert_eq!(birthday(&[1,2,3],3, 1), 1);
    }

}