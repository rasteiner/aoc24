use std::{fmt::{self, Display}, fs, sync::LazyLock, time::Instant};

mod d01;
mod d02;
mod d03;

type Part = fn(&String) -> i32;

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

static DAYS: LazyLock<Vec<Day>> = LazyLock::new(|| {
    let parts: Vec<(Part, Part)> = vec![
        (d01::part1, d01::part2),
        (d02::part1, d02::part2),
        (d03::part1, d03::part2),
    ];

    parts.into_iter().enumerate().map(|(i, (part1, part2))| Day { 
        num: i + 1,
        part1,
        part2,
    }).collect()
});

fn main() {

    loop {
        let day = inquire::Select::new("Choose the day to run", DAYS.clone()).prompt();
        if day.is_err() {
            println!("\nExiting...\n");
            break;
        }
        let day = day.unwrap();
        
        if let Ok(input) = fs::read_to_string(format!("inputs/d{:0>2}.txt", day.num)) {
            let start = Instant::now();
            let result = (day.part1)(&input);
            let duration = start.elapsed();
            println!("Part 1: {} (took {:?})", result, duration);
        
            let start = Instant::now();
            let result = (day.part2)(&input);
            let duration = start.elapsed();
            println!("Part 2: {} (took {:?})", result, duration);        
        } else {
            println!("Could not read input file");
        }
    }
}
