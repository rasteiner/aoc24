use std::collections::HashMap;

use rayon::iter::{ParallelBridge, ParallelIterator};

fn parse(input: &String) -> (Vec<&str>, impl Iterator<Item=&str>) {
    let mut lines = input.lines();

    let towels = lines
        .next().unwrap()
        .split(", ")
        .collect::<Vec<&str>>();

    let lines = lines.skip(1);

    (towels, lines)
}

pub fn part1(input: &String) -> Box<dyn ToString> {
    let (towels, mut lines) = parse(input);
    let mut count = 0;

    // create a regex pattern
    let any_towel_pattern: String = format!("^({})*$", towels.join("|"));
    let reg = regex::Regex::new(&any_towel_pattern).unwrap();
        
    while let Some(line) = lines.next() {
        if reg.is_match(line) {
            count += 1;
        }                
    }
    Box::new(count)
}

pub fn part2(input: &String) -> Box<dyn ToString> {
    let (towels, lines) = parse(input);

    fn do_it(str: &str, options: &Vec<&str>, cache: &mut HashMap<String, i64>) -> i64 {
        if let Some(&c) = cache.get(str) {
            return c;
        }

        let mut count: i64 = 0;
        for option in options {
            if str.get(0..option.len()) == Some(option) {
                if let Some(remaining) = str.get(option.len()..) {
                    if remaining.is_empty() {
                        count += 1;
                    } else {
                        count += do_it(remaining, options, cache);
                    }
                }
            } else {
                continue;
            }
        }

        cache.insert(str.to_string(), count);
        count
    }

    let count: i64 = lines.par_bridge().map(|line| {
        let mut cache = HashMap::new();
        do_it(line, &towels, &mut cache)
    }).sum();

    Box::new(count)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    const TEST_INPUT: &str = indoc! {"
        r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb"
    };
    
    const TEST_RESULT_1: i64 = 6;
    const TEST_RESULT_2: i64 = 16;

    // Test for part1
    #[test]
    fn test_part1() {
        assert_eq!(part1(&String::from(TEST_INPUT)).to_string(), TEST_RESULT_1.to_string());
    }

    // Test for part2
    #[test]
    fn test_part2() {
        assert_eq!(part2(&String::from(TEST_INPUT)).to_string(), TEST_RESULT_2.to_string());
    }
    
}

