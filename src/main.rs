use std::{fmt::{self, Display}, fs, sync::LazyLock, time::Instant};

use colored::Colorize;

type Part = fn(&String) -> Box<dyn ToString>;

#[macro_use]
mod days;
days!(d01, d02, d03, d04, d05, d06, d07, d08, d09, d10, d11, d12, d13, d14, d15, d16, d17, d18, d19, d20, d21, d22);

#[derive(Clone)]
struct Day {
    num: usize,
    part1: Part,
    part2: Part,
}

impl Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Day {:0>2}", self.num)
    }
}


fn main() {

    loop {
        let day = inquire::Select::new("Choose the day to run", DAYS.clone()).prompt();
        if day.is_err() {
            println!("\nExiting...\n");
            break;
        }
        let day = day.unwrap();
        
        if let Ok(input) = fs::read_to_string(format!("inputs/d{:0>2}.txt", day.num)) {
            println!("Running {}...\n", day);
            let start = Instant::now();
            let result = (day.part1)(&input);
            let duration = start.elapsed();
            println!("» Part 1: {} {}\n", result.to_string().green(), format!("(took {:?})", duration).dimmed());
        
            let start = Instant::now();
            let result = (day.part2)(&input);
            let duration = start.elapsed();
            println!("» Part 2: {} {}\n", result.to_string().green(), format!("(took {:?})", duration).dimmed());
            println!();
        } else {
            println!("{}", "Could not read input file".red());
        }
    }
}
