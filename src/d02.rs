const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
const TEST_RESULT1: i32 = 2;
const TEST_RESULT2: i32 = 4;

fn parse(input: &String) -> Vec<Vec<i32>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| 
            line.split_whitespace().map(|n| n.parse::<i32>())
                .filter_map(Result::ok)
                .collect()
        ).collect()
}

fn check (report: &Vec<i32>) -> bool {
    let sign = (report[1] - report[0]).signum();
    report.windows(2).all(|window| (window[1] - window[0]).signum() == sign && (window[1] - window[0]).abs() <= 3)
}

pub fn part1(input: &String) -> i32 {
    let reports = parse(input);
    reports.into_iter().filter(check).count() as i32
}

pub fn part2(input: &String) -> i32 {
    let reports = parse(input);
    reports.into_iter().filter(|report| {
        if check(report) {
            return true;
        }

        // remove one element at a time and check if the report is valid
        for i in 0..report.len() {
            let mut report = report.clone();
            report.remove(i);
            if check(&report) {
                return true;
            }
        }
        
        return false;
    }).count() as i32
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