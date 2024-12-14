use std::{ops::{Add, Mul, Rem, Sub}, sync::LazyLock};
use image::RgbImage;
use regex::Regex;

static PARSE_REGEX_STR: &str = r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)";
static PARSE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(PARSE_REGEX_STR).unwrap());


#[cfg(test)] 
const WIDTH: usize = 11;
#[cfg(test)] 
const HEIGHT: usize = 7;

#[cfg(not(test))] 
const WIDTH: usize = 101;
#[cfg(not(test))] 
const HEIGHT: usize = 103;

static SIZE: Point = Point { x: WIDTH as i64, y: HEIGHT as i64 };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    p: Point,
    v: Point,
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

impl Rem<Point> for Point {
    type Output = Point;

    fn rem(self, rhs: Point) -> Point {
        Point {
            x: (self.x % rhs.x + rhs.x) % rhs.x,
            y: (self.y % rhs.y + rhs.y) % rhs.y,
        }
    }
}

impl Point {
    fn quadrant(&self) -> Option<usize> {
        let w = WIDTH as i64;
        let h = HEIGHT as i64;

        if self.y < h / 2 {
            if self.x < w / 2 {
                return Some(0);
            } else if self.x > w / 2 {
                return Some(1);
            }
        } else if self.y > h / 2 {
            if self.x < w / 2 {
                return Some(2);
            } else if self.x > w / 2 {
                return Some(3);
            }
        }

        None
    }
}

fn parse(input: &String) -> Vec<Robot> {
    PARSE_REGEX.captures_iter(input).map(|cap| {
        Robot {
            p: Point {
                x: cap[1].parse().unwrap(),
                y: cap[2].parse().unwrap(),
            },
            v: Point {
                x: cap[3].parse().unwrap(),
                y: cap[4].parse().unwrap(),
            },
        }
    }).collect()
}

// Part 1: Count the number of peaks reachable from starting points
pub fn part1(input: &String) -> i64 {
    let robots = parse(input);
    
    robots
        .iter()
        .filter_map(|robot| ((robot.p + (robot.v * 100)) % SIZE).quadrant())
        .fold(vec![0; 4], |mut acc, x| {
            // sum the number of bots in each quadrant
            acc[x] += 1 as i64;
            acc
        })
        .into_iter()
        .fold(1, |a, b| a * b)
}

// Part 2: Count the total number of paths from starting points to peaks
pub fn part2(input: &String) -> i64 {
    let mut robots = parse(input);
    
    //make sure output directory exists
    std::fs::create_dir_all("output").unwrap();

    for i in 1..20_000 {
        let mut grid = vec![vec![0; WIDTH]; HEIGHT];
        
        for bot in robots.iter_mut() {
            bot.p = (bot.p + bot.v) % SIZE;
            //img.put_pixel(bot.p.x as u32, bot.p.y as u32, image::Rgb([255, 255, 255]));
            grid[bot.p.y as usize][bot.p.x as usize] = 1;
            
        }
        
        // check if there's a cluster of 5x5 filled cells
        let mut found = false;
        for y in 0..HEIGHT - 5 {
            for x in 0..WIDTH - 5 {
                let mut sum = 0;
                for j in 0..5 {
                    for i in 0..5 {
                        sum += grid[y + j][x + i];
                    }
                }
                if sum == 25 {
                    found = true;
                }
            }
        }

        if found {
            let mut img = RgbImage::new(WIDTH as u32, HEIGHT as u32);
            for bot in robots.iter() {
                img.put_pixel(bot.p.x as u32, bot.p.y as u32, image::Rgb([255, 255, 255]));
            }
            img.save(format!("output/d14_{:0>5}.png", i)).unwrap();
            return i;
        }
    }

    panic!("No solution found");
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    const TEST_INPUT: &str = indoc! {"
        p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3"
    };
    const TEST_RESULT: i64 = 12;

    // Test for part1
    #[test]
    fn test_part1() {
        assert_eq!(part1(&String::from(TEST_INPUT)), TEST_RESULT);
    }
}