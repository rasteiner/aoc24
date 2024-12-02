const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";
const TEST_RESULT1: i32 = 11;
const TEST_RESULT2: i32 = 31;

fn parse_columns(input: &String) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let mut nums = line.split_whitespace().map(|n| n.parse::<i32>().unwrap());
        left.push(nums.next().unwrap());
        right.push(nums.next().unwrap());
    }

    (left, right)
}

pub fn part1(input: &String) -> i32 {
    let (mut left, mut right) = parse_columns(input);

    // sort the vectors
    left.sort();
    right.sort();

    // zip the vectors and sum the differences
    left.into_iter().zip(right.into_iter()).map(|(l, r)| (r - l).abs()).sum()
}

pub fn part2(input: &String) -> i32 {
    let (left, right) = parse_columns(input);

    // for each number of left, count how many times it appears in right, multiply and sum
    let mut sum = 0;
    for l in left {
        sum += l * right.iter().filter(|&r| *r == l).count() as i32;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&String::from(TEST_INPUT)), TEST_RESULT1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&String::from(TEST_INPUT)), TEST_RESULT2);
    }
}