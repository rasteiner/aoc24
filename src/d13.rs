use std::{ops::{Mul, Add, Sub}, sync::LazyLock};
use regex::Regex;

static PARSE_REGEX_STR: &str = r"Button [A-Z]: X([+-]\d+), Y([+-]\d+)\r?\nButton [A-Z]: X([+-]\d+), Y([+-]\d+)\r?\nPrize: X=(\d+), Y=(\d+)";
static PARSE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(PARSE_REGEX_STR).unwrap());

const PART_2_DIFF: i64 = 10_000_000_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
struct Puzzle {
    a: Point,
    b: Point,
    prize: Point,
}

impl Mul<i64> for Point {
    type Output = Point;

    fn mul(self, rhs: i64) -> Point {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

fn parse(input: &String) -> Vec<Puzzle> {
    PARSE_REGEX.captures_iter(input).map(|cap| {
        Puzzle {
            a: Point {
                x: cap[1].parse().unwrap(),
                y: cap[2].parse().unwrap(),
            },
            b: Point {
                x: cap[3].parse().unwrap(),
                y: cap[4].parse().unwrap(),
            },
            prize: Point {
                x: cap[5].parse().unwrap(),
                y: cap[6].parse().unwrap(),
            },
        }
    }).collect()
}

fn solve_with_math(puzzle: Puzzle, larger: bool) -> Option<i64> {
    let p = match larger {
        true => Point {
            x: puzzle.prize.x + PART_2_DIFF,
            y: puzzle.prize.y + PART_2_DIFF,
        }, 
        false => puzzle.prize,
    };
    
    // 94 * a + 22 * b = 8400
    // 34 * a + 67 * b = 5400

    // multiply each equation by the other's coefficient
    // 67 * (94 * a + 22 * b) = 67 * 8400
    // 22 * (34 * a + 67 * b) = 22 * 5400

    // continue with variable names:
    let ax = puzzle.a.x;
    let ay = puzzle.a.y;
    let bx = puzzle.b.x;
    let by = puzzle.b.y;

    // by * (ax * a + bx * b) = by * p.x;
    // bx * (ay * a + by * b) = bx * p.y;

    // expands to:
    // by * ax * a + by * bx * b = by * p.x;
    // bx * ay * a + bx * by * b = bx * p.y;

    // subtract the second equation from the first
    // (by * ax - bx * ay) * a = by * p.x - bx * p.y;

    // solve for a
    let a = (by * p.x - bx * p.y) / (by * ax - bx * ay);

    // replace a in the first equation to solve for b
    let b = (p.x - ax * a) / bx;

    // we only accept positive solutions
    if a < 0 || b < 0 {
        return None;
    }

    // we only accept integer solutions
    if puzzle.a * a + puzzle.b * b != p {
        return None;
    }

    Some(a * 3 + b)
}

// Part 1: Count the number of peaks reachable from starting points
pub fn part1(input: &String) -> i64 {
    let puzzles = parse(input);
    
    puzzles
        .into_iter()
        .map(|p| solve_with_math(p, false))
        .filter_map(|x| x)
        .sum()
}

// Part 2: Count the total number of paths from starting points to peaks
pub fn part2(input: &String) -> i64 {
    let puzzles = parse(input);
    
    puzzles
        .into_iter()
        .map(|p| solve_with_math(p, true))
        .filter_map(|x| x)
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    const TEST_INPUT: &str = indoc! {"
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279"
    };
    const TEST_RESULT: i64 = 480;

    // Test for part1
    #[test]
    fn test_part1() {
        assert_eq!(part1(&String::from(TEST_INPUT)), TEST_RESULT);
    }
}