use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

type Map = Vec<Vec<u8>>;
type Coord = (usize, usize);

// Create a map from the input string
fn create_map(input: &String) -> Map {
    let mut map = Vec::new();

    for line in input.lines() {
        // Convert each character to a u8 and subtract '0' to get the numeric value
        map.push(line.chars().map(|c| (c as u8) - ('0' as u8)).collect());
    }

    map
}

// Find all coordinates where the map value is 9 (peaks)
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

// Find all coordinates where the map value is 0 (starting points)
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

// Get the neighboring coordinates with the next consecutive value
fn next(map: &Map, coord: &Coord) -> Result<Vec<Coord>, ()> {
    let this_num = map[coord.1][coord.0];
    if this_num == 9 {
        // Cannot proceed further if current number is 9
        return Err(());
    }
    let next_num = this_num + 1;

    let mut next = Vec::new();
    // Check the left neighbor
    if coord.0 > 0 {
        if map[coord.1][coord.0 - 1] == next_num {
            next.push((coord.0 - 1, coord.1));
        }
    }
    // Check the right neighbor
    if coord.0 < map[coord.1].len() - 1 {
        if map[coord.1][coord.0 + 1] == next_num {
            next.push((coord.0 + 1, coord.1));
        }
    }
    // Check the top neighbor
    if coord.1 > 0 {
        if map[coord.1 - 1][coord.0] == next_num {
            next.push((coord.0, coord.1 - 1));
        }
    }
    // Check the bottom neighbor
    if coord.1 < map.len() - 1 {
        if map[coord.1 + 1][coord.0] == next_num {
            next.push((coord.0, coord.1 + 1));
        }
    }

    Ok(next)
}

// Part 1: Count the number of peaks reachable from starting points
pub fn part1(input: &String) -> Box<dyn ToString> {
    let map = create_map(input);
    let peaks = peaks(&map);
    let starts = starts(&map);

    // Recursive function to determine if a path from a coordinate can reach a peak
    fn find(map: &Map, coord: &Coord, cache: &mut HashMap<Coord, bool>) -> bool {
        if let Some(&found) = cache.get(coord) {
            // Return cached result if available
            return found;
        }

        let mut found = false;
        if let Ok(next_coords) = next(map, coord) {
            for next_coord in next_coords {
                if find(map, &next_coord, cache) {
                    found = true;
                }
            }
        } else {
            // Reached a peak
            found = true;
        }

        cache.insert(*coord, found);
        found
    }

    let count = AtomicUsize::new(0);

    // Parallel iteration over all starting points
    starts.par_iter().for_each(|start| {
        let mut cache: HashMap<Coord, bool> = HashMap::new();
        find(&map, start, &mut cache);

        // Count the number of peaks reachable from this starting point
        count.fetch_add(
            peaks.iter().filter(|&&peak| cache.get(&peak).copied().unwrap_or(false)).count(),
            Ordering::SeqCst,
        );
    });

    Box::new(count.load(Ordering::SeqCst))
}

// Part 2: Count the total number of paths from starting points to peaks
pub fn part2(input: &String) -> Box<dyn ToString> {
    let map = create_map(input);
    let starts = starts(&map);

    // Recursive function to count the number of paths from a coordinate to peaks
    fn find(map: &Map, coord: &Coord, cache: &mut HashMap<Coord, i64>) -> i64 {
        if let Some(&count) = cache.get(coord) {
            // Return cached count if available
            return count;
        }

        let mut total_paths = 0;
        if let Ok(next_coords) = next(map, coord) {
            for next_coord in next_coords {
                total_paths += find(map, &next_coord, cache);
            }
        } else {
            // Reached a peak
            total_paths = 1;
        }

        cache.insert(*coord, total_paths);
        total_paths
    }

    let mut count = 0;

    // Iterate over all starting points
    starts.iter().for_each(|start| {
        let mut cache: HashMap<Coord, i64> = HashMap::new();
        count += find(&map, start, &mut cache);
    });

    Box::new(count)
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

    // Test for part1
    #[test]
    fn test_part1() {
        assert_eq!(part1(&String::from(TEST_INPUT)).to_string(), TEST_RESULT1.to_string());
    }

    // Test for part2
    #[test]
    fn test_part2() {
        assert_eq!(part2(&String::from(TEST_INPUT)).to_string(), TEST_RESULT2.to_string());
    }
}