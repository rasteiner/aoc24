mod d01;

fn main() {
    let input = include_str!("../inputs/d01.txt");
    println!("Part 1: {}", d01::part1(input));
    println!("Part 2: {}", d01::part2(input));
}
