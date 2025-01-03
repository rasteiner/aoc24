fn parse(input: &String) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| line
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect()
        ).collect()
}

fn check(report: &Vec<i64>) -> bool {
    let sign = (report[1] - report[0]).signum();
    report
        .windows(2)
        .map(|window| window[1] - window[0])
        .all(|diff| diff.signum() == sign && diff.abs() <= 3)
}

pub fn part1(input: &String) -> Box<dyn ToString> {
    let reports = parse(input);

    Box::new(reports.into_iter().filter(check).count())
}

pub fn part2(input: &String) -> Box<dyn ToString> {
    let reports = parse(input);

    Box::new(reports.into_iter().filter(|report| {
        if check(report) {
            return true;
        }

        (0..report.len()).any(|i| {
            let mut report = report.clone();
            report.remove(i);
            check(&report)
        })
    }).count() as i64)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;
    
    const TEST_INPUT: &str = indoc! {"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9"
    };
    const TEST_RESULT1: i64 = 2;
    const TEST_RESULT2: i64 = 4;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&String::from(TEST_INPUT)).to_string(), TEST_RESULT1.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&String::from(TEST_INPUT)).to_string(), TEST_RESULT2.to_string());
    }
}