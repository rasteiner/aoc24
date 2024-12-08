use rayon::prelude::*;
use std::sync::atomic::{AtomicI64, Ordering};

#[derive(Clone, Copy)]
enum Op {
    Add,
    Mul,
    Concat,
}

pub fn part1(input: &String) -> i64 {
    let mut sum = 0;

    for line in input.lines() {
        let (left, right) = line.split_once(":").unwrap();
        let result: usize = left.parse().unwrap();
        let nums: Vec<usize> = right.split_whitespace().map(|n| n.parse().unwrap()).collect();
        let num_permutations = 1 << (nums.len() - 1);

        'nextOp: for op in 0..num_permutations {
            let mut tmp = nums[0];

            for i in 1..nums.len() {
                if op & (1 << (i - 1)) != 0 {
                    tmp += nums[i];
                } else {
                    tmp *= nums[i];
                }

                if tmp > result {
                    continue 'nextOp;
                }
            }

            if tmp == result {
                sum += result;
                break;
            }
        }
    }

    sum as i64
}

pub fn part2(input: &String) -> i64 {
    let sum = AtomicI64::new(0);

    input.par_lines().for_each(|line| {
        let (left, right) = line.split_once(":").unwrap();
        let result = left.parse::<usize>().unwrap();
        let nums: Vec<usize> = right
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let num_ops = nums.len() - 1;
        let total_permutations = 3usize.pow(num_ops as u32);

        for perm in 0..total_permutations {
            let mut tmp = nums[0];
            let mut current_perm = perm;
            let mut valid = true;

            for i in 1..nums.len() {
                let op = match current_perm % 3 {
                    0 => Op::Add,
                    1 => Op::Mul,
                    2 => Op::Concat,
                    _ => unreachable!(),
                };
                current_perm /= 3;

                match op {
                    Op::Add => tmp += nums[i],
                    Op::Mul => tmp *= nums[i],
                    Op::Concat => {
                        tmp = format!("{}{}", tmp, nums[i]).parse::<usize>().unwrap_or(0);
                    }
                }

                if tmp > result {
                    valid = false;
                    break;
                }
            }

            if valid && tmp == result {
                sum.fetch_add(result as i64, Ordering::SeqCst);
                break;
            }
        }
    });

    sum.load(Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    const TEST_INPUT: &str = indoc! {"
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20"
    };
    const TEST_RESULT1: i64 = 3749;
    const TEST_RESULT2: i64 = 11387;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&String::from(TEST_INPUT)), TEST_RESULT1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&String::from(TEST_INPUT)), TEST_RESULT2);
    }
}