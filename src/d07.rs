
pub fn part1(input: &String) -> i64 {
    let mut sum = 0;

    for line in input.lines() {
        let (left, right) = line.split_once(":").unwrap();
        let result: usize = left.parse().unwrap();
        let nums: Vec<usize> = right.split_whitespace().map(|n| n.parse().unwrap()).collect();
        let num_permutations: usize = 1 << (nums.len() - 1);

        'nextOp: for op in 0..num_permutations {
            let mut tmp = nums[0];

            for i in 1..nums.len() {
                if op & (1 << (i-1)) != 0 {
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
    let mut sum = 0;
    
    #[derive(Clone)]
    enum Op {
        Add = 0,
        Mul = 1,
        Concat = 2,
    }

    fn next_perm(ops: &mut Vec<Op>) -> bool {
        let mut carry = 1;
        for op in ops.iter_mut().rev() {
            if carry == 0 {
                break;
            }
            *op = match op {
                Op::Add => {
                    carry = 0;
                    Op::Mul
                },
                Op::Mul => {
                    carry = 0;
                    Op::Concat
                },
                Op::Concat => {
                    carry = 1;
                    Op::Add
                },
            };
        }
        if carry == 1 {
            false
        } else {
            true
        }
    }
    
    'nextLine: for line in input.lines() {
        let (left, right) = line.split_once(":").unwrap();
        let result: usize = left.parse().unwrap();
        let nums: Vec<usize> = right.split_whitespace().map(|n| n.parse().unwrap()).collect();
        let num_ops: usize = nums.len() - 1;

        let mut ops = vec![Op::Add; num_ops];
        loop {
            let mut tmp = nums[0];
            let mut i = 0;
            for op in ops.iter() {
                i += 1;
                match op {
                    Op::Add => {
                        tmp += nums[i];
                    },
                    Op::Mul => {
                        tmp *= nums[i];
                    },
                    Op::Concat => {
                        tmp = (format!("{}{}", tmp, nums[i])).parse().unwrap();
                    },
                }
            }

            if tmp == result {
                sum += result;
                break;
            }

            if !next_perm(&mut ops) {
                continue 'nextLine;
            }
        }
    }

    sum as i64
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