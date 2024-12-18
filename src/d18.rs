use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tile {
    Empty,
    Blocked
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Grid {
    grid: Vec<Vec<Tile>>,
    width: usize,
    height: usize
}

fn make_coords_iter(input: &String) -> impl Iterator<Item=(usize, usize)> + '_ {
    input.lines().map(|line| {
        let (x, y) = line.split_once(",").unwrap();
        (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
    })
}

fn make_grid(input: &String, width: usize, height: usize, bytes: usize) -> Grid {
    let mut tiles = vec![vec![Tile::Empty; width]; height];
    let mut coords = make_coords_iter(input);

    for _ in 0..bytes {
        if let Some((x, y)) = coords.next() {
            tiles[y][x] = Tile::Blocked;
        }
    }

    Grid {
        grid: tiles,
        width,
        height
    }
}

fn find_path_cost(grid: &Grid) -> Option<i64> {
    let width = grid.width;
    let height = grid.height;

    let start = (0, 0);
    let end = (width - 1, height - 1);

    let mut stack = VecDeque::new();
    stack.push_back(start);

    let mut costs = vec![vec![std::i64::MAX; width]; height];
    costs[start.1][start.0] = 0;

    while let Some((x, y)) = stack.pop_front() {
        let cost = costs[y][x];
        if (x, y) == end {
            return Some(cost);
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let nx = x as i64 + dx;
            let ny = y as i64 + dy;
            if nx >= 0 && nx < width as i64 && ny >= 0 && ny < height as i64 {
                let nx = nx as usize;
                let ny = ny as usize;
                if grid.grid[ny][nx] == Tile::Empty && cost + 1 < costs[ny][nx] {
                    costs[ny][nx] = cost + 1;
                    stack.push_back((nx, ny));
                }
            }
        }
    }

    None
}


// these are in separate functions because test and real input have different arguments that aren't specified in the input
fn solve_part1(input: &String, width: usize, height: usize, bytes: usize) -> i64 {
    let grid = make_grid(input, width, height, bytes);
    find_path_cost(&grid).unwrap()
}

fn solve_part2(input: &String, width: usize, height: usize) -> String {
    let tiles = vec![vec![Tile::Empty; width]; height];
    let mut coords = make_coords_iter(input);

    let mut map = Grid {
        grid: tiles,
        width,
        height
    };

    while let Some((x,y)) = coords.next()  {
        map.grid[y][x] = Tile::Blocked;

        if find_path_cost(&map).is_none() {
            return format!("{},{}", x, y);
        }
    }
    
    panic!("No solution found");
}

pub fn part1(input: &String) -> Box<dyn ToString> {
    Box::new(solve_part1(input, 71, 71, 1024))
}

pub fn part2(input: &String) -> Box<dyn ToString> {
    Box::new(solve_part2(input, 71, 71))
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    const TEST_INPUT: &str = indoc! {"
        5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0"
    };
    const TEST_RESULT_1: i64 = 22;
    const TEST_RESULT_2: &str = "6,1";

    // Test for part1
    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&String::from(TEST_INPUT), 7, 7, 12), TEST_RESULT_1);
    }

    // Test for part2
    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&String::from(TEST_INPUT), 7, 7), TEST_RESULT_2.to_string());
    }
    
}