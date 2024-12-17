use std::collections::{HashMap, HashSet};

struct Pos {
    kind: Space,
    coord: Coord,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn new(x: i64, y: i64) -> Coord {
        Coord { x, y }
    }
}

impl std::ops::Sub for Coord {
    type Output = Coord;

    fn sub(self, other: Coord) -> Coord {
        Coord::new(self.x - other.x, self.y - other.y )
    }
}

impl std::ops::SubAssign for Coord {
    fn sub_assign(&mut self, other: Coord) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl std::ops::Add for Coord {
    type Output = Coord;

    fn add(self, other: Coord) -> Coord {
        Coord::new(self.x + other.x, self.y + other.y )
    }
}

impl std::ops::AddAssign for Coord {
    fn add_assign(&mut self, other: Coord) {
        self.x += other.x;
        self.y += other.y;
    }
}

enum Space {
    Empty,
    Antenna(char),
}

fn create_map(input: &String) -> (Vec<Pos>, i64, i64) {
    let mut map = Vec::new();

    let mut w = 0;
    let mut h = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let kind = match c {
                '.' => Space::Empty,
                _ => Space::Antenna(c),
            };

            map.push(Pos {
                kind,
                coord: Coord::new(x as i64, y as i64),
            });
        }

        w = line.len() as i64;
        h = y as i64;
    }

    (map, w, h+1)
}

fn index_map(map: Vec<Pos>) -> HashMap<char, Vec<Coord>> {
    let mut index = HashMap::new();

    for pos in map {
        if let Space::Antenna(c) = pos.kind {
            index.entry(c).or_insert(Vec::new()).push(pos.coord);
        }
    }

    index
}


pub fn part1(input: &String) -> Box<dyn ToString> {
    let (map, w, h) = create_map(input);
    let index = index_map(map);
    let mut nodes = HashSet::new();

    for pos in index.into_values() {
        for i in 0..pos.len() {
            for j in i+1..pos.len() {
                let d = pos[j] - pos[i];
                let a = pos[i] - d;
                let b = pos[j] + d;

                if a.x >= 0 && a.x < w && a.y >= 0 && a.y < h {
                    nodes.insert(a);
                }
                if b.x >= 0 && b.x < w && b.y >= 0 && b.y < h {
                    nodes.insert(b);
                }
            }
        }
    }

    Box::new(nodes.len())
}

pub fn part2(input: &String) -> Box<dyn ToString> {
    let (map, w, h) = create_map(input);
    let index = index_map(map);
    let mut nodes = HashSet::new();

    for pos in index.into_values() {
        for i in 0..pos.len() {
            for j in i+1..pos.len() {
                let mut a = pos[i];
                let mut b = pos[j];

                // we need to go backwards here, 
                // so that we also "cover" the antennas themselves with antinodes
                let d = a - b;

                loop {
                    a -= d;
    
                    if a.x >= 0 && a.x < w && a.y >= 0 && a.y < h {
                        nodes.insert(a);
                    } else {
                        break;
                    }
                }
                
                loop {
                    b += d;

                    if b.x >= 0 && b.x < w && b.y >= 0 && b.y < h {
                        nodes.insert(b);
                    } else {
                        break;
                    }
                }

            }
        }
    }

    Box::new(nodes.len())

}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    const TEST_INPUT: &str = indoc! {"
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............"
    };
    const TEST_RESULT1: i64 = 14;
    const TEST_RESULT2: i64 = 34;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&String::from(TEST_INPUT)).to_string(), TEST_RESULT1.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&String::from(TEST_INPUT)).to_string(), TEST_RESULT2.to_string());
    }
}