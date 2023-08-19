use std::io::{self, Read, Write};
use std::env;

const ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

fn encode_base58(mut n: u64) -> String {
    if n == 0 {
        return "1".to_string();
    }

    let mut result = String::new();
    while n > 0 {
        let remainder = (n % 58) as usize;
        result.push(ALPHABET.chars().nth(remainder).unwrap());
        n /= 58;
    }
    result.chars().rev().collect()
}

fn decode_base58(s: &str) -> Result<u64, &'static str> {
    let mut num = 0;
    for (_i, char) in s.chars().enumerate() {
        if let Some(position) = ALPHABET.find(char) {
            num = num * 58 + position as u64;
        } else {
            return Err("Invalid Base58 character");
        }
    }
    Ok(num)
}

fn print_help() {
    println!("int2b58 - Convert integers to Base58 and vice versa.");
    println!("Usage:");
    println!("  int2b58        Convert integer from stdin to Base58.");
    println!("  int2b58 -d     Convert Base58 string from stdin to integer.");
    println!("  int2b58 -h     Show this help message.");
    println!("  int2b58 --help Show this help message.");
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let is_decode = args.contains(&"-d".to_string()) || args.contains(&"--decode".to_string());
    let is_help = args.contains(&"-h".to_string()) || args.contains(&"--help".to_string());

    if is_help {
        print_help();
        return Ok(());
    }

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    if is_decode {
        match decode_base58(input.trim()) {
            Ok(num) => writeln!(io::stdout(), "{}", num)?,
            Err(e) => eprintln!("Error: {}", e),
        }
    } else {
        if let Ok(num) = input.trim().parse::<u64>() {
            let base58_string = encode_base58(num);
            writeln!(io::stdout(), "{}", base58_string)?;
        } else {
            eprintln!("Error: Input is not a valid integer");
        }
    }

    Ok(())
}
