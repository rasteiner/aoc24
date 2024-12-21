use std::{collections::HashMap, sync::LazyLock, vec};

enum Button {
    Empty, 
    Button(char)
}

impl ToString for Button {
    fn to_string(&self) -> String {
        match self {
            Button::Empty => "".to_string(),
            Button::Button(c) => c.to_string()
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn to_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
    fn to_dir(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

enum Action {
    Move(Direction),
    Push,
}

impl Action {
    fn to_char(&self) -> char {
        match self {
            Action::Move(d) => d.to_char(),
            Action::Push => 'A',
        }
    }
}

static NUMPAD: LazyLock<Vec<Vec<Button>>> = LazyLock::new(||vec![
    vec![Button::Button('7'), Button::Button('8'), Button::Button('9')],
    vec![Button::Button('4'), Button::Button('5'), Button::Button('6')],
    vec![Button::Button('1'), Button::Button('2'), Button::Button('3')],
    vec![Button::Empty,       Button::Button('0'), Button::Button('A')]
]);


static DIRPAD: LazyLock<Vec<Vec<Button>>> = LazyLock::new(||vec![
    vec![Button::Empty,       Button::Button('^'), Button::Button('A')],
    vec![Button::Button('<'), Button::Button('v'), Button::Button('>')],
]);

static NUMPAD_INDEX: LazyLock<HashMap<char, (usize, usize)>> = LazyLock::new(|| index_pad(&*NUMPAD));
static DIRPAD_INDEX: LazyLock<HashMap<char, (usize, usize)>> = LazyLock::new(|| index_pad(&*DIRPAD));

fn index_pad(buttons: &Vec<Vec<Button>>) -> HashMap<char, (usize, usize)> {
    let mut map = HashMap::new();
    for (y, row) in buttons.iter().enumerate() {
        for (x, button) in row.iter().enumerate() {
            if let Button::Button(c) = button {
                map.insert(*c, (x, y));
            }
        }
    }
    map
}

struct Robot {
    directions: usize,
    pad: &'static Vec<Vec<Button>>,
    index: &'static HashMap<char, (usize, usize)>, 
    x: usize,
    y: usize
}

static DIRECTIONS: LazyLock<Vec<Vec<Direction>>> = LazyLock::new(|| 
    // all 24 orders of directions
    vec![
        vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right],
        vec![Direction::Up, Direction::Down, Direction::Right, Direction::Left],
        vec![Direction::Up, Direction::Left, Direction::Down, Direction::Right],
        vec![Direction::Up, Direction::Left, Direction::Right, Direction::Down],
        vec![Direction::Up, Direction::Right, Direction::Down, Direction::Left],
        vec![Direction::Up, Direction::Right, Direction::Left, Direction::Down],
        vec![Direction::Down, Direction::Up, Direction::Left, Direction::Right],
        vec![Direction::Down, Direction::Up, Direction::Right, Direction::Left],
        vec![Direction::Down, Direction::Left, Direction::Up, Direction::Right],
        vec![Direction::Down, Direction::Left, Direction::Right, Direction::Up],
        vec![Direction::Down, Direction::Right, Direction::Up, Direction::Left],
        vec![Direction::Down, Direction::Right, Direction::Left, Direction::Up],
        vec![Direction::Left, Direction::Up, Direction::Down, Direction::Right],
        vec![Direction::Left, Direction::Up, Direction::Right, Direction::Down],
        vec![Direction::Left, Direction::Down, Direction::Up, Direction::Right],
        vec![Direction::Left, Direction::Down, Direction::Right, Direction::Up],
        vec![Direction::Left, Direction::Right, Direction::Up, Direction::Down],
        vec![Direction::Left, Direction::Right, Direction::Down, Direction::Up],
        vec![Direction::Right, Direction::Up, Direction::Down, Direction::Left],
        vec![Direction::Right, Direction::Up, Direction::Left, Direction::Down],
        vec![Direction::Right, Direction::Down, Direction::Up, Direction::Left],
        vec![Direction::Right, Direction::Down, Direction::Left, Direction::Up],
        vec![Direction::Right, Direction::Left, Direction::Up, Direction::Down],
        vec![Direction::Right, Direction::Left, Direction::Down, Direction::Up],
    ]
);

fn get_best_transition(from: char, to: char) -> usize {
    if from == to {
        return 1;
    }

    let mut best_length = usize::MAX;
    
    for directions_1 in 0..DIRECTIONS.len() {
        for directions_2 in 0..DIRECTIONS.len() {
            let mut robots = vec![Robot::new_numpad(directions_1), Robot::new_dirpad(directions_2), Robot::new_dirpad(0)];
            let &(x,y) = NUMPAD_INDEX.get(&from).unwrap();
            robots[0].x = x;
            robots[0].y = y;

            let sequence: String= robots.iter_mut().fold(to.to_string(), |acc, robot| {
                let seq: String = robot.find_sequence(&acc).iter().map(|a| a.to_char()).collect();
                seq
            });

            if sequence.len() < best_length {
                best_length = sequence.len();
            }
        }
    }

    best_length
}

impl Robot {
    fn new(pad: &'static Vec<Vec<Button>>, index: &'static HashMap<char, (usize, usize)>, directions: usize) -> Robot {
        let &(x,y) = index.get(&'A').unwrap();

        Robot {
            pad,
            directions,
            index,
            x,
            y
        }
    }

    fn new_numpad(directions: usize) -> Robot {
        Robot::new(&*NUMPAD, &*NUMPAD_INDEX, directions)
    }

    fn new_dirpad(directions: usize) -> Robot {
        Robot::new(&*NUMPAD, &*DIRPAD_INDEX, directions)
    }

    fn find_sequence(&mut self, input: &String) -> Vec<Action> {
        let mut sequence = vec![];

        for c in input.chars() {
            let &(tx, ty) = self.index.get(&c).unwrap();
            
            loop {
                // find first direction
                let dx = (tx as isize - self.x as isize).signum();
                let dy = (ty as isize - self.y as isize).signum();

                if dx == 0 && dy == 0 {
                    break;
                }

                let &dir = DIRECTIONS[self.directions].iter().find(|&d| {
                    let (dx_, dy_) = d.to_dir();
                    // where would I end up if I moved in this direction
                    if let Some(Button::Empty) = self.pad.get((self.y as isize + dy_) as usize).and_then(|row| row.get((self.x as isize + dx_) as usize)) {
                        return false;
                    }
                    (dx != 0 && dx_ == dx || dy != 0 && dy_ == dy)
                }).expect(format!("No direction found for movement {},{}", dx,dy).as_str());
                
                let (dx, dy) = dir.to_dir();
                self.x = (self.x as isize + dx) as usize;
                self.y = (self.y as isize + dy) as usize;

                sequence.push(Action::Move(dir));
            
            }

            sequence.push(Action::Push);
        }

        sequence
        
    }
}


pub fn part1(input: &String) -> Box<dyn ToString> {

    let mut sum = 0;
    let mut cache = HashMap::new(); 

    for line in input.lines() {
        let mut len = 0;
        let mut previous = 'A';
        for c in line.chars() {
            // parse as hex number
            let seqlen = cache.entry((previous, c))
                .or_insert_with(|| get_best_transition(previous, c))
                .clone();
            len += seqlen;
            previous = c;
        }

        let num = line[0..3].chars().collect::<String>().parse::<usize>().unwrap();
        //println!("{} * {}", sequence.len(), num);
        sum += len * num;
    }

    Box::new(sum)
}

pub fn part2(input: &String) -> Box<dyn ToString> {
    Box::new(0)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    const TEST_INPUT: &str = indoc! {"
        029A
        980A
        179A
        456A
        379A"
    };

    // Test for part1
    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT.to_string()).to_string(), "126384".to_string());
    }

    
    
}