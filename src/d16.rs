use std::{collections::{HashMap, HashSet, VecDeque}, ops::Add};

use colored::Colorize;

type Path = Vec<(Vector, Direction)>;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall, 
    Empty,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Move {
    Forward,
    Left,
    Right,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn vector(&self) -> Vector {
        match self {
            Direction::North => Vector(0, -1),
            Direction::East => Vector(1, 0),
            Direction::South => Vector(0, 1),
            Direction::West => Vector(-1, 0),
        }
    }
}

impl TryFrom<Direction> for usize {
    type Error = ();

    fn try_from(value: Direction) -> Result<Self, Self::Error> {
        match value {
            Direction::North => Ok(0),
            Direction::East => Ok(1),
            Direction::South => Ok(2),
            Direction::West => Ok(3),
        }
    }
}

impl TryFrom<usize> for Direction {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Direction::North),
            1 => Ok(Direction::East),
            2 => Ok(Direction::South),
            3 => Ok(Direction::West),
            _ => Err(()),
        }
    }
}
#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
struct Vector (i64, i64);

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector(self.0 + rhs.0, self.1 + rhs.1)
    }
}

/**
 * Parse the input into a 2D vector of Tiles, return a tuple of (Start, End, and the map)
 */

struct Map {
    map: Vec<Vec<Tile>>,
    costs: Vec<Vec<Vec<i64>>>,
    width: usize,
    height: usize,
    start: Vector,
    end: Vector,
}

impl Map {

    fn new(input: &String) -> Map {
        let mut start = Vector(0, 0);
        let mut end = Vector(0, 0);
    
        let map: Vec<Vec<Tile>> = input
            .lines()
            .enumerate()
            .map(|(y, line)| line.chars()
                .enumerate()
                .map(|(x, c)| 
                    match c {
                        '#' => Tile::Wall,
                        '.' => Tile::Empty,
                        'S' => { start = Vector(x as i64, y as i64); Tile::Empty },
                        'E' => { end = Vector(x as i64, y as i64); Tile::Empty },
                        _ => panic!("Invalid character in input"),
                    }
                ).collect()
            ).collect();
    
        let width = map[0].len();
        let height = map.len();
    
        Map {
            map,
            costs: vec![vec![vec![std::i64::MAX; 4]; width]; height],
            width,
            height,
            start,
            end,
        }
    }

    
    fn try_get(&self, pos: Vector, dir: Direction) -> Option<(Tile,i64)> {
        let x = pos.0;
        let y = pos.1;

        if x < 0 || y < 0 {
            return None;
        }
        
        let (x, y) = (x as usize, y as usize);

        if x < self.width && y < self.height {
            let t = self.map[y][x];
            let cost = self.costs[y][x][usize::try_from(dir).unwrap()];
            Some((t, cost))
        } else {
            None
        }
    }

    fn try_get_lowest(&self, pos: Vector) -> Option<(i64, Direction)> {
        let x = pos.0;
        let y = pos.1;

        if x < 0 || y < 0 {
            return None;
        }
        
        let (x, y) = (x as usize, y as usize);

        if x < self.width && y < self.height {
            if self.map[y][x] == Tile::Wall {
                return None;
            }

            let mut min_cost = std::i64::MAX;
            let mut min_dir = Direction::North;

            for dir in 0..4 {
                let cost = self.costs[y][x][dir];
                if cost < min_cost {
                    min_cost = cost;
                    min_dir = Direction::try_from(dir).unwrap();
                }
            }

            Some((min_cost, min_dir))
        } else {
            None
        }
    }

    fn set(&mut self, pos: Vector, dir: Direction, cost: i64) {
        let x = pos.0;
        let y = pos.1;

        if x >= 0 && y >= 0 {
            let (x, y) = (x as usize, y as usize);
            if x < self.map[0].len() && y < self.map.len() {
                self.costs[y][x][usize::try_from(dir).unwrap()] = cost;
            }
        }
    }

    fn compute(&mut self) -> HashSet<Vector> {

        let start_dir = Direction::East;

        self.set(self.start, start_dir, 0);
    
        let mut queue = VecDeque::new();
        queue.push_back((self.start, start_dir));
    
        let mut visited: HashSet<Vector> = HashSet::new();

        while let Some((pos, dir)) = queue.pop_front() {
    
            visited.insert(pos);

            let cost = self.try_get(pos, dir).unwrap().1;
    
            // try to move forward
            if let Some((Tile::Empty, c)) = self.try_get(pos + dir.vector(), dir) {
                if cost + 1 < c {
                    let np = pos + dir.vector();
                    self.set(np, dir, cost + 1);
                    queue.push_back((np, dir));
                }
            }
    
            // try to turn left
            let left_dir = dir.turn_left();
            if let Some((Tile::Empty, c)) = self.try_get(pos, left_dir) {
                if cost + 1000 < c {
                    self.set(pos, left_dir, cost + 1000);
                    queue.push_back((pos, left_dir));
                }
            }
    
            // try to turn right
            let right_dir = dir.turn_right();
            if let Some((Tile::Empty, c)) = self.try_get(pos, right_dir) {
                if cost + 1000 < c {
                    self.set(pos, right_dir, cost + 1000);
                    queue.push_back((pos, right_dir));
                }
            }
        }

        visited
    
    }

    fn init_costs(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                for dir in 0..4 {
                    self.costs[y][x][dir] = std::i64::MAX;
                }
            }
        }
    }

    /** Recursively find all paths from start to end */
    fn find_paths(&mut self) -> Vec<Path> {

        self.init_costs();


        fn find_paths_recursive(map: &mut Map, pos: Vector, dir: Direction, last_move: Option<Move>) -> Vec<Path> {
            let cost = map.try_get(pos, dir).unwrap().1;
    
            let mut new_paths = Vec::new();

            //qtln!("{} {} {} {:?}", pos.0, pos.1, cost, dir);

            // try to move forward
            if let Some((Tile::Empty, c)) = map.try_get(pos + dir.vector(), dir) {
                if cost + 1 <= c {
                    let np = pos + dir.vector();
                    map.set(np, dir, cost + 1);
                    
                    if np == map.end {
                        return vec![vec![(np, dir)]];
                    } else {
                        let mut paths = find_paths_recursive(map, np, dir, Some(Move::Forward));
                        for path in paths.iter_mut() {
                            path.push((np, dir));
                        }

                        new_paths.extend(paths);
                    }
                }
            }
    
            if last_move.is_none() || last_move.unwrap() == Move::Forward {
                    
                // try to turn left
                let left_dir = dir.turn_left();
                if let Some((Tile::Empty, c)) = map.try_get(pos, left_dir) {
                    if cost + 1000 <= c {
                        map.set(pos, left_dir, cost + 1000);
                        let mut paths = find_paths_recursive(map, pos, left_dir, Some(Move::Left));
                        for path in paths.iter_mut() {
                            path.push((pos, left_dir));
                        }

                        new_paths.extend(paths);
                    }
                }
        
                // try to turn right
                let right_dir = dir.turn_right();
                if let Some((Tile::Empty, c)) = map.try_get(pos, right_dir) {
                    if cost + 1000 <= c {
                        map.set(pos, right_dir, cost + 1000);
                        let mut paths = find_paths_recursive(map, pos, right_dir, Some(Move::Right));
                        for path in paths.iter_mut() {
                            path.push((pos, right_dir));
                        }

                        new_paths.extend(paths);
                    }
                }
            }

            new_paths
        }

        self.set(self.start, Direction::East, 0);
        let all_paths = find_paths_recursive(self, self.start, Direction::East, None);
        
        // calculate cost of each path
        let mut costs: HashMap<i64, Vec<Path>> = HashMap::new();
        let mut min_cost = std::i64::MAX;

        for path in all_paths.iter() {
            let mut cost = 0;
            let mut last_pos = self.start;
            let mut last_dir = Direction::East;

            for (pos, dir) in path {
                if last_dir != *dir {
                    cost += 1000;
                } else if last_pos != *pos {
                    cost += 1;
                }

                last_pos = *pos;
                last_dir = *dir;
            }

            min_cost = min_cost.min(cost);
            costs.entry(cost).or_insert(Vec::new()).push(path.clone());
        }

        costs.remove(&min_cost).unwrap()
    }


    fn print(&self, visited: &HashSet<Vector>) {
        // find lowest and highest costs
        let lowest = visited.iter().map(|pos| self.try_get_lowest(*pos).unwrap().0).min().unwrap() as f64;
        let highest = visited.iter().map(|pos| self.try_get_lowest(*pos).unwrap().0).max().unwrap() as f64;

        let interpolate_color = |cost: i64| {
            let cost = cost as f64;
            let ratio = (cost - lowest) / (highest - lowest);
            let r = (ratio * 255.0) as u8;
            let g = (255.0 - ratio * 255.0) as u8;
            let b = 0u8;
            (r, g, b)
        };

        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Vector(x as i64, y as i64);
                if visited.contains(&pos) {
                    let cost = self.try_get_lowest(pos).unwrap().0;
                    let (r, g, b) = interpolate_color(cost);
                    print!("{}", "O".green().on_truecolor(r,g,b));
                } else {
                    match self.map[y][x] {
                        Tile::Wall => print!("#"),
                        Tile::Empty => print!("{}", ".".truecolor(30, 30, 30)),
                    }
                }
            }
            println!();
        }
    }
}

pub fn part1(input: &String) -> Box<dyn ToString> {
    let mut map = Map::new(input);
    map.compute();

    Box::new(map.try_get_lowest(map.end).unwrap().0)
}

pub fn part2(input: &String) -> Box<dyn ToString> {
    let mut map = Map::new(input);

    let set = std::thread::Builder::new().stack_size(32 * 1024 * 1024).spawn(move|| {    
        let paths = map.find_paths();
        let mut set: HashSet<Vector> = HashSet::new();
        for path in paths {
            for (pos, _) in path {
                set.insert(pos);
            }
        }
    
        map.print(&set);
        set
    }).unwrap().join().unwrap();

    
    Box::new(set.len())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    const TEST_INPUT_1: &str = indoc! {"
        ###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############"
    };
    const TEST_RESULT_1: i64 = 7036;
    const TEST2_RESULT_1: i64 = 45;

    
    const TEST_INPUT_2: &str = indoc! {"
        #################
        #...#...#...#..E#
        #.#.#.#.#.#.#.#.#
        #.#.#.#...#...#.#
        #.#.#.#.###.#.#.#
        #...#.#.#.....#.#
        #.#.#.#.#.#####.#
        #.#...#.#.#.....#
        #.#.#####.#.###.#
        #.#.#.......#...#
        #.#.###.#####.###
        #.#.#...#.....#.#
        #.#.#.#####.###.#
        #.#.#.........#.#
        #.#.#.#########.#
        #S#.............#
        #################"
    };
    const TEST_RESULT_2: i64 = 11048;
    const TEST2_RESULT_2: i64 = 64;


    // Test for part1
    #[test]
    fn test_part1() {
        assert_eq!(part1(&String::from(TEST_INPUT_1)).to_string(), TEST_RESULT_1.to_string());
        assert_eq!(part1(&String::from(TEST_INPUT_2)).to_string(), TEST_RESULT_2.to_string());
    }
    
    // Test for part2
    #[test]
    fn test_part2() {
        assert_eq!(part2(&String::from(TEST_INPUT_1)).to_string(), TEST2_RESULT_1.to_string());
        assert_eq!(part2(&String::from(TEST_INPUT_2)).to_string(), TEST2_RESULT_2.to_string());
    }
}