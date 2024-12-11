use std::collections::HashMap;

fn set_or_increase(map: &mut HashMap<i64, i64>, key: i64, value: i64) {
    *map.entry(key).or_insert(0) += value;
}

fn part_keys(input: &str, blinks: u32) -> i64 {
    let initial: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut n: HashMap<i64, i64> = HashMap::new();
    for &v in &initial {
        set_or_increase(&mut n, v, 1);
    }

    for _ in 0..blinks {
        let mut n2: HashMap<i64, i64> = HashMap::new();
        for (&v, &c) in &n {
            if v == 0 {
                set_or_increase(&mut n2, 1, c);
            } else {
                let digits = v.ilog10() + 1;
                if digits % 2 == 0 {
                    let half = digits / 2;
                    let left = v / 10i64.pow(half as u32);
                    let right = v % 10i64.pow(half as u32);
                    
                    set_or_increase(&mut n2, left, c);
                    set_or_increase(&mut n2, right, c);
                } else {
                    set_or_increase(&mut n2, v * 2024, c);
                }
            }
        }
        n = n2;
    }
    n.values().sum()
}

pub fn part1(input: &String) -> i64 {
    part_keys(input, 25)
}

pub fn part2(input: &String) -> i64 {
    part_keys(input, 75)
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