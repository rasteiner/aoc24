use std::collections::{HashMap, HashSet};

type Link = (String, String);

fn parse_input(input: &String) -> Vec<Link> {
    input.lines().map(|l| {
        let (l,r) = l.split_once('-').unwrap();
        (l.to_string(), r.to_string())

    }).collect()
}

fn create_graph(links: &Vec<Link>) -> HashMap<String, HashSet<String>> {
    let mut graph = HashMap::new();

    for (l, r) in links {
        graph.entry(l.clone()).or_insert(HashSet::new()).insert(r.clone());
        graph.entry(r.clone()).or_insert(HashSet::new()).insert(l.clone());
    }

    graph
}

pub fn part1(input: &String) -> Box<dyn ToString> {
    let links = parse_input(input);
    let graph = create_graph(&links);

    let mut nets = HashSet::new();

    for (k, v) in &graph {
        for n in v {
            
            if let Some(other) = graph.get(n) {
                // get intersection of the two sets
                let intersection = v.intersection(other).collect::<Vec<_>>();
                
                for i in intersection {
                    let mut items = vec![k, n, i];
                    items.sort();
                    nets.insert(items);
                }
            }
        }
    }
    
    Box::new(
        nets
            .into_iter()
            .filter(|n| n.iter().any(|&x| x.starts_with("t")))
            .count()
    )
}

/**
 * From https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm "With Pivoting"
 * 
 * R: The current clique being built
 * P: the set of vertices that are candidates for the clique
 * X: the set of vertices that have already been excluded from the clique
 * 
 * A clique is a subset of vertices of an undirected graph such that every two distinct vertices in the clique are adjacent.
 */
fn bron_kerbosch(
    r: HashSet<String>,
    p: &mut HashSet<String>,
    x: &mut HashSet<String>,
    graph: &HashMap<String, HashSet<String>>,
    cliques: &mut Vec<HashSet<String>>
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r);
        return;
    }

    // chose a pivot vertex
    let pivot = p.union(x).next().unwrap().clone();

    let subp: HashSet<String> = p.difference(graph.get(&pivot).unwrap()).cloned().collect();

    for v in subp {
        let mut nr = r.clone();
        nr.insert(v.clone());
        let mut np: HashSet<String> = p.intersection(graph.get(&v).unwrap()).cloned().collect();
        let mut nx: HashSet<String> = x.intersection(graph.get(&v).unwrap()).cloned().collect();
        bron_kerbosch(nr, &mut np, &mut nx, &graph, cliques);

        p.remove(&v.clone());
        x.insert(v.clone());
    }
}

pub fn part2(input: &String) -> Box<dyn ToString> {
    let links = parse_input(input);
    let graph = create_graph(&links);

    let mut p: HashSet<String> = graph.keys().cloned().collect();
    let mut cliques = Vec::new();

    bron_kerbosch(HashSet::new(), &mut p, &mut HashSet::new(), &graph, &mut cliques);

    // get largest clique
    let mut largest: Vec<String> = cliques
        .into_iter()
        .max_by_key(|c| c.len())
        .unwrap()
        .into_iter()
        .collect();

    largest.sort();

    Box::new(largest.join(","))
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    const TEST_INPUT: &str = indoc! {"
        kh-tc
        qp-kh
        de-cg
        ka-co
        yn-aq
        qp-ub
        cg-tb
        vc-aq
        tb-ka
        wh-tc
        yn-cg
        kh-ub
        ta-co
        de-co
        tc-td
        tb-wq
        wh-td
        ta-ka
        td-qp
        aq-cg
        wq-ub
        ub-vc
        de-ta
        wq-aq
        wq-vc
        wh-yn
        ka-de
        kh-ta
        co-tc
        wh-qp
        tb-vc
        td-yn
    "};

    #[test] 
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT.to_string()).to_string(), "7".to_string());
    }

    #[test] 
    fn test_part2() {
        assert_eq!(part2(&TEST_INPUT.to_string()).to_string(), "co,de,ka,ta".to_string());
    }
}