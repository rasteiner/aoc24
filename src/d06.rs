use std::{collections::HashSet, str::FromStr, sync::BarrierWaitResult, thread::sleep, time::Duration};

struct World {
    map: Vec<Space>,
    width: usize,
    height: usize,
    guard: Guard,

    extra_obstacle: Option<(i32,i32)>
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

        Ok(Self { map, width, height, guard: guard.unwrap(), extra_obstacle: None })
    }
}

impl World {
    fn get(&self, (x, y): (i32, i32)) -> Result<&Space, WorldError> {
        if x < 0 || y < 0 {
            return Err(WorldError::OutOfBounds);
        }
        
        let x = x as usize;
        let y = y as usize;

        if x >= self.width || y >= self.height {
            return Err(WorldError::OutOfBounds);
        }
        
        if let Some((ex, ey)) = self.extra_obstacle {
            let ex = ex as usize;
            let ey = ey as usize;

            if x == ex && y == ey {
                return Ok(&Space::Obstacle);
            }
        }

        let i: usize = y * self.width + x;

        Ok(&self.map[i])
    }

    fn move_extra_obstacle(&mut self) -> Result<(), WorldError> {
        if let Some(o) = self.extra_obstacle {
            let current = self.width * o.1 as usize + o.0 as usize;
            let guard_i = self.width * self.guard.y as usize + self.guard.x as usize;

            if let Some(next) = self.map[current+1..].iter().enumerate().find_map(|(i, s)| {
                if *s == Space::Empty && current + 1 + i != guard_i {
                    Some(i)
                } else {
                    None
                }
            }) {
                let next = next + current + 1;
                let x = next % self.width;
                let y = next / self.width;

                self.extra_obstacle = Some((x as i32, y as i32));
                return Ok(());
            } else {
                return Err(WorldError::OutOfBounds);
            }            
        } else {
            self.extra_obstacle = Some((0,0));
            Ok(())
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
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

#[derive(Clone)]
struct Guard {
    x: i32,
    y: i32,
    dir: Direction,
    path: HashSet<(i32, i32, Direction)>,
}

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

    fn next_pos(&self) -> (i32, i32) {
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

pub fn part1(input: &String) -> i32 {
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
            _ => {},
        }
    }

    let unique_coords: HashSet<(i32, i32)> = w.guard.path.into_iter().map(|step| {
        let x = (step.0, step.1); x
    }).collect();
    unique_coords.len() as i32
}

pub fn part2(input: &String) -> i32 {
    let mut w = World::from_str(input).or_else(|e| {
        eprintln!("{}", e);
        Err(0)
    }).unwrap();

    
    let original_guard = w.guard.clone();
    
    let mut loop_count = 0;

    loop {
        match w.move_extra_obstacle() {
            Ok(_) => {
            },
            Err(WorldError::OutOfBounds) => {
                break;
            },
            Err(_) => {
                panic!("Unexpected error");
            },
        }

        loop {
            if let Ok(s) = w.get(w.guard.next_pos()) {
                match s {
                    Space::Empty => {
                        if w.guard.move_forward().is_err() {
                            loop_count += 1;
                            break;
                        }
                    },
                    Space::Obstacle => {
                        w.guard.turn_right();
                    }
                }
            } else {
                break;
            }
        }

        // reset guard
        w.guard = original_guard.clone();
    }

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
    const TEST_RESULT1: i32 = 41;
    const TEST_RESULT2: i32 = 6;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&String::from(TEST_INPUT)), TEST_RESULT1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&String::from(TEST_INPUT)), TEST_RESULT2);
    }
}