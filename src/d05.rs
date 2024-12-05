use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Rule {
    before: HashSet<i32>,
}

#[derive(Debug)]
struct Manual {
    pages: Vec<i32>,
}

impl Manual {
    fn is_valid(&self, rules: &HashMap<i32,Rule>) -> bool {
        for i in 1..self.pages.len() {
            let current = self.pages[i];
            if let Some(rule) = rules.get(&current) {
                for before in &rule.before {
                    if self.pages.iter().take(i).any(|&p| p == *before) {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn sort(&self, rules: &HashMap<i32,Rule>) -> Manual {
        let mut pages = self.pages.clone();
        
        pages.sort_by(|a, b| {
            if let Some(rule) = rules.get(a) {
                if rule.before.contains(b) {
                    return std::cmp::Ordering::Less;
                }
            }
            if let Some(rule) = rules.get(b) {
                if rule.before.contains(a) {
                    return std::cmp::Ordering::Greater;
                }
            }
            std::cmp::Ordering::Equal
        });

        Manual { pages }
    }

    fn middle(&self) -> i32 {
        self.pages[self.pages.len() / 2]
    }
}

impl Rule {
    fn add_page(&mut self, page: i32) {
        self.before.insert(page);
    }
}

struct ParseError {
    message: String,
}

fn parse(input: &String) -> Result<(HashMap<i32,Rule>, Vec<Manual>), ParseError> {
    let mut rules = HashMap::new();
    let mut manuals = Vec::new();

    // split input by double newline
    if let Some((rulestr, manualstr)) = input.split_once("\n\n").or(input.split_once("\r\n\r\n")) {
        // parse rules
        for line in rulestr.lines() {
            if let Some((page, before)) = line.split_once("|").and_then(|(a,b)| {
                Some((a.parse::<i32>().ok()?, b.parse::<i32>().ok()?)) 
            }) {
                let rule = rules.entry(page).or_insert(Rule {
                    before: HashSet::new(),
                });
                rule.add_page(before);
            } else {
                return Err(ParseError { message: format!("Error parsing rule: {}", line) });
            }
        }

        // parse manuals
        for line in manualstr.lines() {
            let pages: Vec<i32> = line.split(",").filter_map(|n| n.parse::<i32>().ok()).collect();
            manuals.push(Manual { pages });
        }
        return Ok((rules, manuals));

    } else {
        return Err(ParseError { message: String::from("Error parsing input") });
    }
    
}

pub fn part1(input: &String) -> i32 {
    match parse(input) {
        Ok((rules, manuals)) =>
            manuals
                .iter()
                .filter(|m| m.is_valid(&rules))
                .map(Manual::middle)
                .sum(),

        Err(e) => {
            eprintln!("{}", e.message);
            0
        }
    }
}

pub fn part2(input: &String) -> i32 {
    match parse(input) {
        Ok((rules, manuals)) => 
            manuals
                .iter()
                .filter(|m| !m.is_valid(&rules))
                .map(|m| m.sort(&rules).middle())
                .sum(),

        Err(e) => {
            eprintln!("{}", e.message);
            0
        }
    }
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