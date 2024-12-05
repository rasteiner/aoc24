use std::str::FromStr;

struct Input {
    rules: Rules,
    manuals: Manuals,
}

type Manuals = Vec<Manual>;

#[derive(Debug)]
struct Manual {
    pages: Vec<usize>,
}

struct Rules {
    map: Vec<Vec<usize>>,
}

struct RuleDef(usize, usize);

impl Rules {
    fn get(&self, key: usize) -> Option<&Vec<usize>> {
        self.map.get(key)
    }
}

impl Manual {
    fn is_valid(&self, rules: &Rules) -> bool {
        for i in 1..self.pages.len() {
            let current = self.pages[i];
            if let Some(rule) = rules.get(current) {
                for before in rule {
                    if self.pages.iter().take(i).any(|&p| p == *before) {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn sort(mut self, rules: &Rules) -> Manual {
        self.pages.sort_by(|a, b| {
            if let Some(rule) = rules.get(*a) {
                if rule.contains(b) {
                    return std::cmp::Ordering::Less;
                }
            }
            if let Some(rule) = rules.get(*b) {
                if rule.contains(a) {
                    return std::cmp::Ordering::Greater;
                }
            }
            std::cmp::Ordering::Equal
        });

        self
    }

    fn middle(&self) -> usize {
        self.pages[self.pages.len() / 2]
    }
}

impl FromStr for RuleDef {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        input
            .split_once("|").ok_or(parse_error(input))
            .and_then(|(a, b)| {
                let a: usize = a.parse().map_err(|_| number_parse_error(input, a))?;
                let b: usize = b.parse().map_err(|_| number_parse_error(input, b))?;
                Ok(RuleDef(a, b))
            })
    }
}

impl FromStr for Rules {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let rules = input
            .lines()
            .map(|line| line.parse())
            .try_fold(Vec::new(), |mut map: Vec<Vec<usize>>, rule| {
                let rule: RuleDef = rule?;
                match map.get_mut(rule.0) {
                    Some(vec) => vec.push(rule.1),
                    None => {
                        if map.len() <= rule.0 {
                            map.resize(rule.0 + 1, Vec::new());
                        }
                        map[rule.0] = vec![rule.1];
                    } 
                }
                Ok(map)
            })?;
            
        Ok(Rules { map: rules })
    }
}

impl FromStr for Manual {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let pages  = input
            .split(',')
            .map(|n| n.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()
            .map_err(|_| parse_error(input))?;

        Ok(Manual { pages })
    }
}

impl FromStr for Input {
    type Err = ParseError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split = input
            .split_once("\n\n")
            .or_else(|| input.split_once("\r\n\r\n"));

        if let Some((rulestr, manualstr)) = split {
            let rules = rulestr.parse()?;
    
            let manuals = manualstr
                .lines()
                .map(|line| line.parse())
                .collect::<Result<Manuals,_>>()?;
    
            return Ok(Input {rules, manuals});
        } else {
            return Err(ParseError { message: String::from("Error parsing input") });
        }
    }
}

struct ParseError {
    message: String,
}

pub fn part1(input: &String) -> i32 {
    match input.parse::<Input>() {
        Ok(input) => 
            input.manuals
                .into_iter()
                .filter(|m| m.is_valid(&input.rules))
                .map(|m| m.middle())
                .sum::<usize>()
                .try_into()
                .expect("Sum is too large for i32"),

        Err(e) => {
            eprintln!("{}", e.message);
            0
        }
    }
}

pub fn part2(input: &String) -> i32 {
    match input.parse::<Input>() {
        Ok(input) => 
            input.manuals
                .into_iter()
                .filter(|m| !m.is_valid(&input.rules))
                .map(|m| m.sort(&input.rules).middle())
                .sum::<usize>()
                .try_into()
                .expect("Sum is too large for i32"),

        Err(e) => {
            eprintln!("{}", e.message);
            0
        }
    }
}

fn parse_error(input: &str) -> ParseError {
    ParseError { message: format!("Error parsing [{}]", input) }
}

fn number_parse_error(input: &str, nan: &str) -> ParseError {
    ParseError { message: format!("Error parsing [{}], {} is not a number", input, nan) }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;
    
    const TEST_INPUT: &str = indoc! {"
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47"
    };
    const TEST_RESULT1: i32 = 143;
    const TEST_RESULT2: i32 = 123;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&String::from(TEST_INPUT)), TEST_RESULT1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&String::from(TEST_INPUT)), TEST_RESULT2);
    }
}