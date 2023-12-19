use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_first_digit(chars: Vec<char>) -> u32 {
    for c in chars {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap();
        }
    }
    return 0u32;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} path/to/file", args[0]);
        return;
    }
    let file_path = &args[1];

    let mut digits = Vec::<u32>::new();
    let mut sum = 0u32;

    println!("reading file {}", file_path);
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                // go through each character in the line
                let mut num : u32 = get_first_digit(ip.chars().collect());

                // reverse it and go through th
                num = num * 10 + get_first_digit(ip.chars().rev().collect());

                sum = sum + num;
                digits.push(num);
            }
        }
    }

    println!("digits: {:?}", digits);
    println!("sum: {:?}", sum);
}
