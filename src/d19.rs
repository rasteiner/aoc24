pub fn part1(input: &String) -> Box<dyn ToString> {
    let mut lines = input.lines();

    let towels = lines
        .next().unwrap()
        .split(", ")
        .collect::<Vec<&str>>();

    let mut lines = lines.skip(1);
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
    Box::new(0)
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
    const TEST_RESULT_2: &str = "0";

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

