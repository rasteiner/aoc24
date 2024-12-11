use std::collections::HashMap;
type LookupCache = HashMap<(i64,i64),i64>;

fn do_rules(stone: i64, depth: i64, cache: &mut LookupCache) -> i64 {
    if let Some(result) = cache.get(&(stone, depth)) {
        return *result;
    }
    
    let result = if depth == 0 {
        1
    } else if stone == 0 {
        do_rules(1, depth - 1, cache)
    } else {
        let digits = stone.ilog10() + 1;
        if digits % 2 == 0 {
            let half = digits / 2;
            let left = stone / 10_i64.pow(half as u32);
            let right = stone % 10_i64.pow(half as u32);

            do_rules(left, depth - 1, cache) + do_rules(right, depth - 1, cache)
        } else {
            do_rules(stone * 2024, depth - 1, cache)
        }
    };
    
    // store 
    cache.insert((stone, depth), result);

    result
}

fn parse_and_do(input: &String, depth: i64) -> i64 {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .map(|x| do_rules(x, depth, &mut HashMap::new()))
        .sum()
}

pub fn part1(input: &String) -> i64 {
    parse_and_do(input, 25)
}

pub fn part2(input: &String) -> i64 {
    parse_and_do(input, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "125 17";
    const TEST_RESULT: i64 = 55312;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&String::from(TEST_INPUT)), TEST_RESULT);
    }
}