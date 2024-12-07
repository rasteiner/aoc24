use std::{collections::HashSet, str::FromStr, sync::{Arc, Mutex}, thread::spawn};

#[derive(Clone)]
struct World {
    map: Vec<Space>,
    width: usize,
    height: usize,
    guard: Guard,
}

impl FromStr for World {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<&str>>();
        let height = lines.len();
        let width = lines[0].len();
        let mut guard: Option<Guard> = None;

        let map = lines
            .join("")
            .chars()
            .enumerate()
            .map(|(i, c)| {
                match c {
                    '.' => Ok(Space::Empty),
                    '#' => Ok(Space::Obstacle),
                    '^' | 'v' | '<' | '>' => {
                        guard = Some(Guard::new(i % width, i / width, c.try_into()?));
                        Ok(Space::Empty)
                    },
                    _ => Err(format!("Invalid space: {}", c)),
                }
            })
            .collect::<Result<Vec<Space>, String>>()?;

        if guard.is_none() {
            return Err("No guard found".to_string());
        }

        Ok(Self { map, width, height, guard: guard.unwrap() })
    }
}

impl World {
    fn get(&self, (x, y): (i64, i64)) -> Result<&Space, WorldError> {
        if x < 0 || y < 0 {
            return Err(WorldError::OutOfBounds);
        }
        
        let x = x as usize;
        let y = y as usize;

        if x >= self.width || y >= self.height {
            return Err(WorldError::OutOfBounds);
        }
        
        let i: usize = y * self.width + x;

        Ok(&self.map[i])
    }

    fn set(&mut self, (x, y): (i64, i64), s: Space) -> Result<(), WorldError> {
        if x < 0 || y < 0 {
            return Err(WorldError::OutOfBounds);
        }
        
        let x = x as usize;
        let y = y as usize;

        if x >= self.width || y >= self.height {
            return Err(WorldError::OutOfBounds);
        }

        let i: usize = y * self.width + x;

        self.map[i] = s;

        Ok(())
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Space {
    Empty,
    Obstacle
}


#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(s: char) -> Result<Self, Self::Error> {
        match s {
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => Err(format!("Invalid direction: {}", s)),
        }
    }
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Clone, Debug)]
struct Guard {
    x: i64,
    y: i64,
    dir: Direction,
    path: HashSet<(i64, i64, Direction)>,
}

#[derive(Debug)]
enum WorldError {
    OutOfBounds,
    LoopDetected,
}

impl Guard {
    fn new(x: usize, y: usize, dir: Direction) -> Self {
        let x = x.try_into().unwrap();
        let y = y.try_into().unwrap();

        Self { x, y, dir: dir.clone(), path: HashSet::from([(x,y,dir)]) }
    }

    fn next_pos(&self) -> (i64, i64) {
        match self.dir {
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
        }
    }

    fn move_forward(&mut self) -> Result<(), WorldError> {
        let (x, y) = self.next_pos();
        let step = (x, y, self.dir.clone());
        
        if self.path.contains(&step) {
            return Err(WorldError::LoopDetected);
        }

        self.x = x;
        self.y = y;

        self.path.insert(step);

        Ok(())
    }

    fn turn_right(&mut self) {
        self.dir = self.dir.turn_right();
    }
}

pub fn part1(input: &String) -> i64 {
    let mut w = World::from_str(input).or_else(|e| {
        eprintln!("{}", e);
        Err(0)
    }).unwrap();

    while let Ok(s) = w.get(w.guard.next_pos()) {
        match s {
            Space::Empty => {
                if w.guard.move_forward().is_err() {
                    panic!("Loop detected");
                }
            },
            Space::Obstacle => {
                w.guard.turn_right();
            },
        }
    }

    let unique_coords: HashSet<(i64, i64)> = w.guard.path.into_iter().map(|step| (step.0, step.1)).collect();
    unique_coords.len() as i64
}

pub fn part2(input: &String) -> i64 {
    let mut w = World::from_str(input).or_else(|e| {
        eprintln!("{}", e);
        Err(0)
    }).unwrap();
    
    let original_guard = w.guard.clone();

    while let Ok(s) = w.get(w.guard.next_pos()) {
        match s {
            Space::Empty => {
                if w.guard.move_forward().is_err() {
                    panic!("Loop detected");
                }
            },
            Space::Obstacle => {
                w.guard.turn_right();
            },
        }
    }

    let unique_coords: HashSet<(i64, i64)> = w.guard.path.iter().map(|step| (step.0, step.1)).collect();
    
    let loop_count = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for pos in unique_coords {
        if original_guard.x == pos.0 && original_guard.y == pos.1 {
            continue;
        }

        let mut world = w.clone();
        world.guard = original_guard.clone();
        let loop_count = Arc::clone(&loop_count);

        let handle = spawn(move || {
            world.set(pos, Space::Obstacle).unwrap();

            loop {
                if let Ok(s) = world.get(world.guard.next_pos()) {
                    match s {
                        Space::Empty => {
                            if world.guard.move_forward().is_err() {
                                let mut count = loop_count.lock().unwrap();
                                *count += 1;
                                break;
                            }
                        },
                        Space::Obstacle => {
                            world.guard.turn_right();
                        }
                    }
                } else {
                    break;
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let loop_count = Arc::try_unwrap(loop_count).unwrap().into_inner().unwrap();

    loop_count
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;
    
    const TEST_INPUT: &str = indoc! {"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#..."
    };
    const TEST_RESULT1: i64 = 41;
    const TEST_RESULT2: i64 = 6;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&String::from(TEST_INPUT)), TEST_RESULT1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&String::from(TEST_INPUT)), TEST_RESULT2);
    }
}