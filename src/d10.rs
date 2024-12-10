use std::{collections::HashMap, sync::atomic::{AtomicUsize, Ordering}};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

type Map = Vec<Vec<u8>>;
type Coord = (usize, usize);

fn create_map(input: &String) -> Map {
    let mut map = Vec::new();

    for line in input.lines() {
        map.push(line.chars().map(|c| (c as u8) - ('0' as u8)).collect());
    }

    map
}

fn peaks(map: &Map) -> Vec<Coord> {
    let mut peaks = Vec::new();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 9 {
                peaks.push((x, y));
            }
        }
    }

    peaks
}

fn starts(map: &Map) -> Vec<Coord> {
    let mut starts = Vec::new();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 0 {
                starts.push((x, y));
            }
        }
    }

    starts
}

fn next(map: &Map, coord: &Coord) -> Result<Vec<Coord>, ()> {
    let this_num = map[coord.1][coord.0];
    if this_num == 9 {
        return Err(());
    }
    let next_num = this_num + 1;

    let mut next = Vec::new();
    if coord.0 > 0 {
        if map[coord.1][coord.0 - 1] == next_num {
            next.push((coord.0 - 1, coord.1));
        }
    }
    if coord.0 < map[coord.1].len() - 1 {
        if map[coord.1][coord.0 + 1] == next_num {
            next.push((coord.0 + 1, coord.1));
        }
    }
    if coord.1 > 0 {
        if map[coord.1 - 1][coord.0] == next_num {
            next.push((coord.0, coord.1 - 1));
        }
    }
    if coord.1 < map.len() - 1 {
        if map[coord.1 + 1][coord.0] == next_num {
            next.push((coord.0, coord.1 + 1));
        }
    }
    
    Ok(next)
}

pub fn part1(input: &String) -> i64 {
    let map = create_map(input);
    let peaks = peaks(&map);
    let starts = starts(&map);

    fn find(map: &Map, coord: &Coord, cache: &mut HashMap<Coord, bool>) -> bool {
        if let Some(&found) = cache.get(coord) {
            return found;
        }

        let mut found = false;
        if let Ok(next) = next(map, coord) {
            for n in next {
                if find(map, &n, cache) {
                    found = true;
                }
            }
        } else {
            found = true;
        }


        cache.insert(*coord, found);
        found
    }

    let count = AtomicUsize::new(0);

    starts.par_iter().for_each(|p| {
        let mut cache: HashMap<Coord, bool> = HashMap::new();
        find(&map, p, &mut cache);

        count.fetch_add(peaks.iter().filter_map(|p| cache.get(p)).count(), Ordering::SeqCst);
    });

    count.load(Ordering::SeqCst) as i64

}

pub fn part2(input: &String) -> i64 {
    let map = create_map(input);
    let starts = starts(&map);

    fn find(map: &Map, coord: &Coord, cache: &mut HashMap<Coord, i64>) -> i64 {
        if let Some(&found) = cache.get(coord) {
            return found;
        }

        let mut found = 0;
        if let Ok(next) = next(map, coord) {
            for n in next {
                let c = find(map, &n, cache);
                found += c;
            }
        } else {
            found = 1;
        }

        cache.insert(*coord, found);
        found
    }

    let mut count = 0;

    starts.iter().for_each(|p| {
        let mut cache: HashMap<Coord, i64> = HashMap::new();
        count += find(&map, p, &mut cache);
    });

    count as i64

}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    const TEST_INPUT: &str = indoc! {"
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732"
    };
    const TEST_RESULT1: i64 = 36;
    const TEST_RESULT2: i64 = 81;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&String::from(TEST_INPUT)), TEST_RESULT1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&String::from(TEST_INPUT)), TEST_RESULT2);
    }
}