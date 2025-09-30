use std::io;
use std::thread::sleep;
use std::time::Duration;
use chrono::Datelike;
use chrono::Utc;

fn main() {
    println!("Enter your year of birth:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let birth_year: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    println!("Calculating...");
    sleep(Duration::from_secs(2));

    let current_year = Utc::now().year();
    let age = current_year - birth_year;

    let generation = match birth_year {
        1928..=1945 => "Silent Generation",
        1946..=1964 => "Baby Boomer",
        1965..=1980 => "Generation X",
        1981..=1996 => "Millennial",
        1997..=2012 => "Generation Z",
        2013..=2025 => "Generation Alpha",
        _ => "Unknown Generation",
    };

    println!("You are {} years old, a {}.", age, generation);
}
