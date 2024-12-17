fn parse_columns(input: &String) -> (Vec<i64>, Vec<i64>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let mut nums = line.split_whitespace().map(|n| n.parse::<i64>().unwrap());
        left.push(nums.next().unwrap());
        right.push(nums.next().unwrap());
    }

    (left, right)
}

pub fn part1(input: &String) -> Box<dyn ToString> {
    let (mut left, mut right) = parse_columns(input);

    // sort the vectors
    left.sort();
    right.sort();

    // zip the vectors and sum the differences
    let sum: i64 = left.into_iter().zip(right.into_iter()).map(|(l, r)| (r - l).abs()).sum();
    Box::new(sum)
}

pub fn part2(input: &String) -> Box<dyn ToString> {
    let (left, right) = parse_columns(input);

    // for each number of left, count how many times it appears in right, multiply and sum
    let mut sum: i64 = 0;
    for l in left {
        sum += l * right.iter().filter(|&r| *r == l).count() as i64;
    }

    Box::new(sum)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    const TEST_INPUT: &str = indoc! {"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3"
    };
    const TEST_RESULT1: i64 = 11;
    const TEST_RESULT2: i64 = 31;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT.to_string()).to_string(), TEST_RESULT1.to_string());        
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TEST_INPUT.to_string()).to_string(), TEST_RESULT2.to_string());
    }
}