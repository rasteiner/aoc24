use std::sync::LazyLock;

static MULREG: LazyLock<regex::Regex> = LazyLock::new(|| regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap());

pub fn part1(input: &String) -> i32 {
    let mut sum = 0;

    for (_, [a, b]) in MULREG.captures_iter(&input).map(|c| c.extract()) {
        let a = a.parse::<i32>().unwrap();
        let b = b.parse::<i32>().unwrap();
        sum += a * b;
    }

    sum
}

pub fn part2(input: &String) -> i32 {
    let mut sum = 0;
    
    for chunk in input.split("do()") {
        let before_dont = chunk.split("don't()").next().unwrap();
        sum += part1(&String::from(before_dont));
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const TEST_INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_INPUT2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    const TEST_RESULT1: i32 = 161;
    const TEST_RESULT2: i32 = 48;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&String::from(TEST_INPUT)), TEST_RESULT1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&String::from(TEST_INPUT2)), TEST_RESULT2);
    }
}