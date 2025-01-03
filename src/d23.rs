use std::collections::{HashMap, HashSet};

fn parse_input(input: &String) -> Vec<(&str, &str)> {
    input.lines().map(|l| l.split_once('-').unwrap()).collect()
}

fn create_graph<'a>(links: &'a Vec<(&str, &str)>) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut graph = HashMap::new();

    for &(l, r) in links {
        graph.entry(l).or_insert(HashSet::new()).insert(r);
        graph.entry(r).or_insert(HashSet::new()).insert(l);
    }

    graph
}

fn create_adjacency_matrix<'a>(links: &'a Vec<(&str, &str)>) -> (Vec<&'a str>, Vec<Vec<bool>>) {
    let mut counter = 0;
    let mut nodes = HashMap::new();
    let mut index = Vec::new();
    for &(l, r) in links {
        if !nodes.contains_key(l) {
            nodes.insert(l, counter);
            index.push(l);
            counter += 1;
        }
        if !nodes.contains_key(r) {
            nodes.insert(r, counter);
            index.push(r);
            counter += 1;
        }
    }

    let mut matrix = vec![vec![false; counter]; counter];
    for &(l, r) in links {
        let l = nodes[l];
        let r = nodes[r];
        matrix[l][r] = true;
        matrix[r][l] = true;
    }
    
    (index, matrix)

}

pub fn part1(input: &String) -> Box<dyn ToString> {
    let links = parse_input(input);
    let (index, matrix) = create_adjacency_matrix(&links);

    // let mut nets = Vec::new();
    let mut count = 0;

    for i in 0..index.len() {
        for j in i+1..index.len() {
            if matrix[i][j] {
                for k in j+1..index.len() {
                    if matrix[i][k] && matrix[j][k] && 
                        (index[i].starts_with("t") || index[j].starts_with("t") || index[k].starts_with("t")) 
                    {
                        //nets.push(vec![index[i], index[j], index[k]]);
                        count += 1;
                    }
                }
            }
        }
    }
    
    Box::new(count)
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
fn bron_kerbosch<'a>(
    r: HashSet<&'a str>,
    p: &mut HashSet<&'a str>,
    x: &mut HashSet<&'a str>,
    graph: &HashMap<&'a str, HashSet<&'a str>>,
    cliques: &mut Vec<HashSet<&'a str>>
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r);
        return;
    }

    // chose a pivot vertex
    let pivot = p.union(x).next().unwrap();

    let subp: HashSet<&str> = p.difference(graph.get(pivot).unwrap()).cloned().collect();

    for v in subp {
        let mut nr = r.clone();
        nr.insert(v);
        let mut np: HashSet<&str> = p.intersection(graph.get(v).unwrap()).cloned().collect();
        let mut nx: HashSet<&str> = x.intersection(graph.get(v).unwrap()).cloned().collect();
        bron_kerbosch(nr, &mut np, &mut nx, &graph, cliques);

        // move v from p to x
        p.remove(v);
        x.insert(v);
    }
}

pub fn part2(input: &String) -> Box<dyn ToString> {
    let links = parse_input(input);
    let graph = create_graph(&links);

    let mut p: HashSet<&str> = graph.keys().cloned().collect();
    let mut cliques = Vec::new();

    bron_kerbosch(HashSet::new(), &mut p, &mut HashSet::new(), &graph, &mut cliques);

    // get largest clique
    let mut largest: Vec<&str> = cliques
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