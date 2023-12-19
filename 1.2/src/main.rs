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

fn get_numbers() -> Vec<Vec<char>> {
    let start  = vec![
        vec!['o', 'n', 'e'],
        vec!['t', 'w', 'o'],
        vec!['t', 'h', 'r', 'e', 'e'],
        vec!['f', 'o', 'u', 'r'],
        vec!['f', 'i', 'v', 'e'],
        vec!['s', 'i', 'x'],
        vec!['s', 'e', 'v', 'e', 'n'],
        vec!['e', 'i', 'g', 'h', 't'],
        vec!['n', 'i', 'n', 'e']
    ];

    return start;
}

pub fn get_num(input: Vec<char>) -> u32 {
    let mut result = 0u32;
    let mut numbers = get_numbers();

    let mut first_found = false;
    let mut first = 0u32;
    let mut last = 0u32;

    // go through each index
    let mut index = 0usize;

    'outer: loop {

        if index >= input.len() {
            break;
        }

        let c = input[index];

        // if this is a digit, store it and continue
        if c.is_digit(10) {
            if first_found {
                last = char::to_digit(c, 10).unwrap();
            } else {
                last = char::to_digit(c, 10).unwrap();
                first = last;
                first_found = true;
            }
            index = index + 1;
            continue;
        }

        let mut value : char = '0';
        'inner: for i in 0..numbers.len() {
            let n = &numbers[i];
            // if there are enough characters, then do the comparison
            if index + n.len() <= input.len() {
                // do the comparison
                let a = &n[0.. n.len()];
                let b = &input[index.. index+n.len()];

                if a == b {
                    if first_found {
                        last = (i + 1usize) as u32;
                    } else {
                        last = (i + 1usize) as u32;
                        first = last;
                        first_found = true;
                    }
                    break 'inner;
                }
            }
        }
        index = index + 1;
    }

    result = first * 10 + last;
    return result;
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
                let n = get_num(ip.chars().collect());
                sum = sum + n;
                digits.push(n);
            }
        }
    }

    println!("digits: {:?}", digits);
    println!("sum: {:?}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! make_test {
        ($name:ident, $str:expr, $exp:expr) => {
            #[test]
            fn $name() {
                let first : Vec<_> = $str.chars().collect();
                let expected : u32 = $exp.parse::<u32>().unwrap();
                let conv = get_num(first);
                assert_eq!(conv, expected);
            }
        };
    }

    make_test!(first_test,
        "ttmtqrh3four4oneightrkv",
        "38");

    make_test!(second_test,
        "seven6sevenphjfhdtnrhfsgcfived9seven",
        "77");

    make_test!(third_test,
        "7fjkfdlmhqxtwoxcpssngss",
        "72");

    make_test!(fourth_test,
        "gsntbddbnone4cjqjmspzcsxmvvthreefive",
        "15");

}
