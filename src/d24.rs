use std::{collections::{HashMap, HashSet, VecDeque}, str::FromStr};

use itertools::Itertools;
use rayon::vec;

#[derive(Debug, Clone, Copy)]
enum Operation {
    AND,
    OR,
    XOR,
}

#[derive(Debug)]
struct Gate<'a> {
    i1: &'a str,
    op: Operation,
    i2: &'a str,
    out: &'a str,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Operation::AND),
            "OR" => Ok(Operation::OR),
            "XOR" => Ok(Operation::XOR),
            _ => Err(()),
        }
    }
}

fn parse<'a>(input: &'a String) -> (HashMap<&'a str, u8>, Vec<Gate>) {
    let mut lines = input.lines();
    
    let start = lines.by_ref().take_while(|line| !line.is_empty()).filter_map(|line| {
        let (o, i) = line.split_once(": ").unwrap();

        let v = match i.parse::<u8>() {
            Ok(v) => v,
            Err(_) => {
                panic!("error parsing \"{}\" as u8", i);
            },
        };
        Some((o, v))
    }).collect();

    let _ = lines.by_ref().skip_while(|line| line.is_empty());

    let connections = lines.filter_map(|line| {
        let (left, right) = line.split_once(" -> ").unwrap();
        if let Some((i1, op, i2)) = left.split_whitespace().collect_tuple() {
            Some(Gate {
                i1: i1,
                op: op.parse().unwrap(),
                i2: i2,
                out: right,
            })
        } else {
            None
        }
    }).collect();

    (start, connections)
}

pub fn make_num(initial: &str, values: &HashMap<&str, u8>) -> u64 {
    let mut num: u64 = 0;

    for (k, &v) in values {
        if k.starts_with(initial) {
            let shift = k[1..].parse::<u32>().unwrap();
            num |= (v as u64) << shift;
        }
    }

    num
}

pub fn part1(input: &String) -> Box<dyn ToString> {
    let (mut values, connections) = parse(input);

    let mut queue = VecDeque::new();
    for gate in connections.iter() {
        queue.push_back(gate);
    }

    while let Some(gate) = queue.pop_front() {
        if values.contains_key(&gate.i1) && values.contains_key(&gate.i2) {
            let v1 = values[&gate.i1];
            let v2 = values[&gate.i2];
            let result = match gate.op {
                Operation::AND => v1 & v2,
                Operation::OR => v1 | v2,
                Operation::XOR => v1 ^ v2,
            };
            values.insert(gate.out, result);
        } else {
            queue.push_back(gate);
        }
    }
    
    Box::new(make_num("z", &values))
}


fn run<'a>(x_values: Vec<u8>, y_values: Vec<u8>, connections: &'a Vec<Gate>, switches: &HashMap<&'a str, &'a str>) -> Result<u64, ()> {
    let mut queue = VecDeque::new();
    let mut values = HashMap::new();

    for gate in connections.iter() {
        queue.push_back(gate);
    }

    let mut found_new_value_for = 0usize;
    
    while let Some(gate) = queue.pop_front() {
        

        let i1 = match gate.i1.chars().nth(0) {
            Some('x') => x_values.get(gate.i1[1..].parse::<usize>().unwrap()),
            Some('y') => y_values.get(gate.i1[1..].parse::<usize>().unwrap()),
            _ => values.get(gate.i1)
        };
        let i2 = match gate.i2.chars().nth(0) {
            Some('x') => x_values.get(gate.i2[1..].parse::<usize>().unwrap()),
            Some('y') => y_values.get(gate.i2[1..].parse::<usize>().unwrap()),
            _ => values.get(gate.i2)
        };

        if i1.is_some() && i2.is_some() {
            let v1 = *i1.unwrap();
            let v2 = *i2.unwrap();
            let result = match gate.op {
                Operation::AND => v1 & v2,
                Operation::OR => v1 | v2,
                Operation::XOR => v1 ^ v2,
            };

            let &out = switches.get(gate.out).unwrap_or(&gate.out);

            values.insert(out, result);
            found_new_value_for = 0;

        } else {
            if values.contains_key(&gate.out) {
                return Err(())
            }

            if found_new_value_for > connections.len() * 2 {
                return Err(());
            }

            found_new_value_for += 1;

            queue.push_back(gate);
        }
    }

    Ok(make_num("z", &values))
}



/**
 * returns true when an error is found
 */
fn check_wires<'a>(digit: u8, max: u8, connections: &'a Vec<Gate>, switches: &HashMap<&str, &str>) -> Result<(HashSet<String>, usize),()> {
    let mut problemset = HashSet::new();
    let mut error_count = 0;


    let setup = |x: usize, y: usize| -> (Vec<u8>, Vec<u8>) {
        let mut x_values = vec![0u8; max as usize];
        let mut y_values = vec![0u8; max as usize];
    
        for i in 0..max {
            let n = (1 as usize) << i;
            if x & (n) != 0 {
                x_values[i as usize] = 1;
            }

            if y & n != 0 {
                y_values[i as usize] = 1;
            }
        }

        (x_values, y_values)
    };

    // case 1: 0 + 0 = 0
    let (x_values, y_values) = setup(0, 0);
    let result = run(x_values.clone(), y_values.clone(), connections, switches)?;
    if result != 0 {
        // add the problmatic z gates (in this case all those that are 1) to the problemset
        for i in 0..max {
            if result & (1 << i) != 0 {
                problemset.insert(i);
                error_count += 1;
            }
        }
    }

    // case 2: 1 << digit + 0 = 1 << digit
    let (x_values, y_values) = setup(1 << digit, 0);
    let result = run(x_values.clone(), y_values.clone(), connections, switches)?;
    if result != 1 << digit {
        
        let should_be_1: u64 = 1 << digit;
        for i in 0..max {
            if result & (1 << i) != should_be_1 & (1 << i) {
                problemset.insert(i);
                error_count += 1;
            }
        }
    }

    // case 3: 0 + 1 << digit = 1 << digit
    let (x_values, y_values) = setup(0, 1 << digit);
    let result = run(x_values.clone(), y_values.clone(), connections, switches)?;

    if result != 1 << digit {
        let should_be_1: u64 = 1 << digit;
        for i in 0..max {
            if result & (1 << i) != should_be_1 & (1 << i) {
                problemset.insert(i);
                error_count += 1;
            }
        }
    }

    // case 4: 1 << digit + 1 << digit = 1 << (digit + 1)
    let (x_values, y_values) = setup(1 << digit, 1 << digit);
    let result = run(x_values.clone(), y_values.clone(), connections, switches)?;
    if result != 1 << (digit + 1) {
        let should_be_1: u64 = 1 << digit + 1;
        for i in 0..max {
            if result & (1 << i) != should_be_1 & (1 << i) {
                problemset.insert(i);
                error_count += 1;
            }
        }
    }

    // return all gates getween xi and zi, and also all the gates between yi and zi
    let mut result = HashSet::new();
    for z in problemset.iter() {
        result = result.union(&wires_from_to(format!("x{:02}", digit), format!("z{:02}", z), connections).into_iter().collect()).cloned().collect();
        result = result.union(&wires_from_to(format!("y{:02}", digit), format!("z{:02}", z), connections).into_iter().collect()).cloned().collect();
    }

    // remove all gates that begin with x or y from the problemset
    result = result.into_iter().filter(|gate| !gate.starts_with("x") && !gate.starts_with("y")).collect();
    Ok((result, error_count))


}

// finds the shortest path from one gate to another
fn wires_from_to(from: String, to: String, connections: &Vec<Gate>) -> Vec<String> {

    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
    for gate in connections {
        adjacency.entry(gate.i1.to_owned()).or_default().push(gate.out.to_owned());
        adjacency.entry(gate.i2.to_owned()).or_default().push(gate.out.to_owned());
    }

    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    queue.push_back(from.clone());
    visited.insert(from.clone(), None);

    while let Some(current) = queue.pop_front() {
        if current == to {
            let mut path = Vec::new();
            let mut node = Some(current);
            while let Some(n) = node {
                path.push(n.clone());
                node = visited[&n].clone();
            }
            path.reverse();
            return path;
        }
        if let Some(nexts) = adjacency.get(&current) {
            for n in nexts {
                if !visited.contains_key(n) {
                    visited.insert(n.clone(), Some(current.clone()));
                    queue.push_back(n.clone());
                }
            }
        }
    }

    Vec::new()
}

fn validate<'a>(switches: &Vec<(&str, &str)>, connections: &Vec<Gate>, digits: u8) -> Result<(HashSet<String>, usize), ()> {
    let mut problemset = HashSet::new();
    let mut error_count = 0;

    let mut switchmap = HashMap::new();
    for switch in switches {
        switchmap.insert(switch.0, switch.1);
        switchmap.insert(switch.1, switch.0);
    }

    for i in 0..digits {
        // check if there's a path from xi to zi and from yi to zi
        if wires_from_to(format!("x{:02}", i), format!("z{:02}", i), connections).len() == 0 {
            return Err(());
        }
        if wires_from_to(format!("y{:02}", i), format!("z{:02}", i), connections).len() == 0 {
            return Err(());
        }


        let (newset, errs) = check_wires(i, digits, &connections, &switchmap)?;
        if newset.len() > 0 {
            problemset = problemset.union(&newset).cloned().collect();
            error_count += errs;
        }
    }

    return Ok((problemset, error_count));
}

pub fn part2(input: &String) -> Box<dyn ToString> {
    let (values, connections) = parse(input);
    let mut digits = 0;

    // reset every value to 0
    values.iter().for_each(|(k, _)| {
        match k.chars().nth(0) {
            Some('x') => digits += 1,
            _ => (),
        }
    });

    let (problemset, err_count) = validate(&vec![], &connections, digits).expect("error validating without switches");
    
    println!("Bruteforcing Problemset: {:?} which has {} errors", problemset, err_count);

    let problemset: Vec<String> = problemset.into_iter().collect();
    let mut switched = vec![];

    for i in 0..problemset.len() {
        for j in i+1..problemset.len() {
            let switch = vec![(problemset[i].as_str(), problemset[j].as_str())];
            if let Ok((_, errs)) = validate(&switch, &connections, digits) {
                if errs < err_count {
                    println!("Switching {} and {} reduced errors from {} to {}", problemset[i], problemset[j], err_count, errs);
                    switched.push((problemset[i].clone(), problemset[j].clone()));
                }
            }
        }
        println!("Progress: {}/{}", i, problemset.len());
    }

    println!("Switched: {:?}", switched);

    let mut lowest_errs = err_count;
    let mut lowest_switches = vec![];

    // check if a combination of 4 switches can reduce the error count to 0
    for i in 0..switched.len() {
        for j in i+1..switched.len() {
            for k in j+1..switched.len() {
                for l in k+1..switched.len() {

                    let switches = vec![
                        (switched[i].0.as_str(), switched[i].1.as_str()),
                        (switched[j].0.as_str(), switched[j].1.as_str()),
                        (switched[k].0.as_str(), switched[k].1.as_str()),
                        (switched[l].0.as_str(), switched[l].1.as_str()),
                    ];
                    if let Ok((faulty, errs)) = validate(&switches, &connections, digits) {
                        if errs == 0 {
                            println!("Found a combination of switches that reduces errors to 0: {:?}", switches);
                            return Box::new(switches.iter().map(|(a, b)| format!("{},{}", a, b)).join(","));
                        } else if errs < lowest_errs {
                            println!("Found a combination of switches that reduces errors to {}: {:?}, faulty wires: {:?}", errs, switches, faulty);
                            lowest_errs = errs;
                            lowest_switches = switches;
                        }
                    }
                }
            }
        }
    }

    println!("Could not find a combination of switches that reduces errors to 0, but the lowest error count was {}", lowest_errs);

    Box::new(0)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    const TEST_INPUT: &str = indoc! {"
        x00: 1
        x01: 1
        x02: 1
        y00: 0
        y01: 1
        y02: 0

        x00 AND y00 -> z00
        x01 XOR y01 -> z01
        x02 OR y02 -> z02
    "};

    const TEST_INPUT2: &str = indoc! {"
        x00: 1
        x01: 0
        x02: 1
        x03: 1
        x04: 0
        y00: 1
        y01: 1
        y02: 1
        y03: 1
        y04: 1

        ntg XOR fgs -> mjb
        y02 OR x01 -> tnw
        kwq OR kpj -> z05
        x00 OR x03 -> fst
        tgd XOR rvg -> z01
        vdt OR tnw -> bfw
        bfw AND frj -> z10
        ffh OR nrd -> bqk
        y00 AND y03 -> djm
        y03 OR y00 -> psh
        bqk OR frj -> z08
        tnw OR fst -> frj
        gnj AND tgd -> z11
        bfw XOR mjb -> z00
        x03 OR x00 -> vdt
        gnj AND wpb -> z02
        x04 AND y00 -> kjc
        djm OR pbm -> qhw
        nrd AND vdt -> hwm
        kjc AND fst -> rvg
        y04 OR y02 -> fgs
        y01 AND x02 -> pbm
        ntg OR kjc -> kwq
        psh XOR fgs -> tgd
        qhw XOR tgd -> z09
        pbm OR djm -> kpj
        x03 XOR y03 -> ffh
        x00 XOR y04 -> ntg
        bfw OR bqk -> z06
        nrd XOR fgs -> wpb
        frj XOR qhw -> z04
        bqk OR frj -> z07
        y03 OR x01 -> nrd
        hwm AND bqk -> z03
        tgd XOR rvg -> z12
        tnw OR pbm -> gnj
    "};

    #[test] 
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT.to_string()).to_string(), "4".to_string());
        assert_eq!(part1(&TEST_INPUT2.to_string()).to_string(), "2024".to_string());
    }

    #[test] 
    fn test_part2() {
        assert_eq!(part2(&TEST_INPUT2.to_string()).to_string(), "co,de,ka,ta".to_string());
    }
}