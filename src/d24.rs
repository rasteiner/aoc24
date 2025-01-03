use std::{collections::{HashMap, VecDeque}, str::FromStr};

use clipboard::ClipboardProvider;
use itertools::Itertools;
use rayon::str;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Operation {
    AND,
    OR,
    XOR,
}

#[derive(Debug, PartialEq, Eq, Clone)]
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

impl ToString for Operation {
    fn to_string(&self) -> String {
        match self {
            Operation::AND => "AND",
            Operation::OR => "OR",
            Operation::XOR => "XOR"
        }.to_string()
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


pub fn part2(input: &String) -> Box<dyn ToString> {
    let (_, connections) = parse(input);
    
    let mut nodes = Vec::new();
    let mut gates = Vec::new();
    let mut edges = Vec::new();
    
    #[derive(Debug, Hash, PartialEq, Eq)]
    struct Node {
        id: String, 
        label: String, 
        shape: &'static str,
    }

    #[derive(Debug, Hash, PartialEq, Eq)]
    struct Edge {
        label: String, 
        from: String,
        to: String,
        to_type: String,
    }
    let switches = [
        ("z05", "gdd"),
        ("z09", "cwt"),
        ("css", "jmv"),
        ("z37", "pqt"),
    ];

    // apply switches
    let connections = connections.iter().map(|conn| {
        let mut conn = conn.clone();
        for &(a, b) in switches.iter() {
            if conn.out == a {
                conn.out = b;
            } else if conn.out == b {
                conn.out = a;
            }
        }
        conn
    }).collect::<Vec<_>>();
    
    // add inputs
    for i in 0..=44 {
        for c in ['x', 'y'] {
            nodes.push(Node {
                id: format!{"{}{:02}", c, i},
                label: format!{"{}{:02}", c, i},
                shape: "circle"
            });
        }
    }

    for conn in connections.iter() {
        
        let id = format!("_{}", conn.out);

        gates.push(Node {
            id: id.clone(),
            label: conn.op.to_string(),
            shape: "rect"
        });
        
        edges.push(Edge {
            label: conn.i1.to_string(),
            from: match conn.i1.chars().nth(0).unwrap() {'x' | 'y' => conn.i1.to_string(), _=> format!("_{}", conn.i1)},
            to: id.clone(),
            to_type: conn.op.to_string(),
        });
        
        edges.push(Edge {
            label: conn.i2.to_string(),
            from: match conn.i1.chars().nth(0).unwrap() {'x' | 'y' => conn.i2.to_string(), _=> format!("_{}", conn.i2)},
            to: id.clone(),
            to_type: conn.op.to_string(),
        });

        if conn.out.chars().nth(0) == Some('z') {
            edges.push(Edge {
                label: conn.out.to_string(),
                from: id.clone(),
                to: conn.out.to_string(),
                to_type: "output".to_string(),
            });
        }
    }

    // sort gates by operation
    gates.sort_by(|a,b| a.label.cmp(&b.label));

    // sort edges by operation
    edges.sort_by(|a,b| a.to_type.cmp(&b.to_type));

    // add to nodes
    nodes.extend(gates.into_iter());

    // add outputs
    for i in 0..=44 {
        nodes.push(Node {
            id: format!{"z{:02}", i},
            label: format!{"z{:02}", i},
            shape: "circle"
        });
    }

    let mut txt = "flowchart LR\n".to_string();

    for node in nodes.iter() {
        txt.push_str(&format!("\t{}@{{ shape: {}, label: {} }}\n", node.id, node.shape, node.label));
    }
    for edge in edges.iter() {
        txt.push_str(&format!("\t{} --> |{}| {}\n", edge.from, edge.label, edge.to));
    }

    let mut clip = clipboard::ClipboardContext::new().unwrap();
    clip.set_contents(txt).unwrap();

    println!("Mermaid flowchart copied to clipboard");

    let mut result = switches.into_iter().fold(vec![], |mut acc, (a,b)| {
        acc.push(a);
        acc.push(b);
        acc
    });

    result.sort();

    Box::new(result.join(","))
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