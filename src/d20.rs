use std::{collections::{HashMap, VecDeque}, ops::AddAssign};
use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tile {
    Empty,
    Blocked
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Grid {
    grid: Vec<Vec<Tile>>,
    start: (usize, usize),
    end: (usize, usize),
    width: usize,
    height: usize
}


fn make_grid(input: &String) -> Grid {

    let mut tiles = vec![];

    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => row.push(Tile::Blocked),
                '.' => row.push(Tile::Empty),
                'S' => {
                    row.push(Tile::Empty);
                    start = (x, y);
                }
                'E' => {
                    row.push(Tile::Empty);
                    end = (x, y);
                }
                _ => panic!("Invalid character in input")
            }
        }
        tiles.push(row);
    }

    Grid {
        width: tiles[0].len(),
        height: tiles.len(),
        grid: tiles,
        start,
        end
    }
}

fn find_path(grid: &Grid) -> Option<Vec<(usize, usize)>> {
    let width = grid.width;
    let height = grid.height;

    let start = grid.start;
    let end = grid.end;

    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut costs = vec![vec![std::i64::MAX; width]; height];
    costs[start.1][start.0] = 0;
    
    while let Some((x, y)) = queue.pop_front() {
        let cost = costs[y][x];
        if (x, y) == end {
            break;
        }

        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)].iter() {
            let nx = x as i64 + dx;
            let ny = y as i64 + dy;

            if nx >= 0 && nx < width as i64 && ny >= 0 && ny < height as i64 {
                let nx = nx as usize;
                let ny = ny as usize;

                if grid.grid[ny][nx] == Tile::Empty && cost + 1 < costs[ny][nx] {
                    costs[ny][nx] = cost + 1;
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    if costs[end.1][end.0] == std::i64::MAX {
        return None;
    }

    let mut path = vec![];
    let mut current = end;

    while current != start {
        path.push(current);
        let cost = costs[current.1][current.0];
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)].iter() {
            let nx = current.0 as i64 + dx;
            let ny = current.1 as i64 + dy;

            if nx >= 0 && nx < width as i64 && ny >= 0 && ny < height as i64 {
                let nx = nx as usize;
                let ny = ny as usize;

                if grid.grid[ny][nx] == Tile::Empty && costs[ny][nx] == cost - 1 {
                    current = (nx, ny);
                    break;
                }
            }
        }
    }

    path.push(start);

    path.reverse();
    Some(path)
}

fn count_around_kernel(input: &String, min_saving: usize, cheat_time: i64) -> usize {
    
    let map = make_grid(input);
    let path = find_path(&map).unwrap();

    let h = map.width as i64;
    let w = map.height as i64;

    let mut count = 0;

    // index the path
    let mut path_index = HashMap::new();
    for (i, (x, y)) in path.iter().enumerate() {
        path_index.insert((*x, *y), i);
    }


    for (i, (x, y)) in path.into_iter().enumerate() {
        let x = x as i64;
        let y = y as i64;
        
        for dx  in -cheat_time..=cheat_time {
            let r = cheat_time - dx.abs();

            for dy in -r..=r {
                let nx = x + dx;
                let ny = y + dy;
                
                if nx < 0 || nx >= w || ny < 0 || ny >= h {
                    continue;
                }
                
                if let Some(&j) = path_index.get(&(nx as usize, ny as usize)) {
                    if i >= j {
                        continue;
                    }

                    let j = j as i64;
                    let i = i as i64;

                    if j - i - dx.abs() - dy.abs() < min_saving as i64 {
                        continue;
                    }

                    count += 1;
                }
            }
        }
    }

    count
}

fn count_on_path(input: &String, min_saving: usize, cheat_time: usize) -> usize {
    let map = make_grid(input);
    let path = find_path(&map).unwrap();
    
    #[cfg(test)]
    let mut savings = HashMap::new();

    let mut count = 0;

    for (i, &(x, y)) in path.iter().enumerate() {
        for (j, &(nx, ny)) in path.iter().enumerate().skip(i+min_saving) {
            // check if in range
            let md = x.abs_diff(nx) + y.abs_diff(ny);
            if md > cheat_time {
                continue;
            }

            if j-i-md < min_saving {
                continue;
            }

            count += 1;
            
            #[cfg(test)]
            savings.entry(j-i-md).or_insert(0).add_assign(1);
        }
    }

    #[cfg(test)]
    {
        // sort savings by key and print all
        for (steps, count_for_steps) in savings.iter().sorted() {
            println!("There are {} cheats that save {} picoseconds", count_for_steps, steps);
        }
    }
    
    count
}

pub fn part1(input: &String) -> Box<dyn ToString> {
    Box::new(count_around_kernel(input, 100, 2))
}

pub fn part2(input: &String) -> Box<dyn ToString> {
    Box::new(count_on_path(input, 100, 20))
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    const TEST_INPUT: &str = indoc! {"
        ###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############"
    };

    // Test for part1
    #[test]
    fn test_part1() {
        assert_eq!(count_on_path(&TEST_INPUT.to_string(), 2, 2), 44);
        assert_eq!(count_around_kernel(&TEST_INPUT.to_string(), 2, 2), 44);
    }

    // Test for part2
    #[test]
    fn test_part2() {
        assert_eq!(count_on_path(&TEST_INPUT.to_string(), 50, 20), 285);
        assert_eq!(count_around_kernel(&TEST_INPUT.to_string(), 50, 20), 285);
    }
    
    
}