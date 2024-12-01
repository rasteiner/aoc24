use std::time::Instant;

mod d01;

fn main() {
    let input = include_str!("../inputs/d01.txt");

    let start_part1 = Instant::now();
    let result_part1 = d01::part1(input);
    let duration_part1 = start_part1.elapsed();
    println!("Part 1: {} (took {:?})", result_part1, duration_part1);

    let start_part2 = Instant::now();
    let result_part2 = d01::part2(input);
    let duration_part2 = start_part2.elapsed();
    println!("Part 2: {} (took {:?})", result_part2, duration_part2);
}
