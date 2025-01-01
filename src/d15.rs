type Grid = Vec<Vec<Pos>>;
type Instructions = Vec<Instruction>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Pos {
    Wall,
    Box,
    Empty,
    LeftBox,
    RightBox,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
struct Robot {
    x: i64,
    y: i64,
}

impl Robot {
    fn new(x: usize, y: usize) -> Self {
        Self { x: x as i64, y: y as i64 }
    }

    fn try_move(&mut self, grid: &mut Grid, instruction: Instruction) -> Result<(),String> {
        let (dx, dy) = instruction.to_direction();
        let mut x = self.x + dx;
        let mut y = self.y + dy;
        let first = try_get(x, y, grid)?;
        
        if first == Pos::Wall {
            return Err(String::from("Wall"));
        }

        if first == Pos::Empty {
            self.x = x;
            self.y = y;
            return Ok(());
        }

        let first = (x, y);
        
        x += dx;
        y += dy;

        let space = loop {
            match try_get(x, y, &grid)? {
                Pos::Wall => return Err(String::from("Wall after box")),
                Pos::Empty => break (x, y),
                _ => {
                    x += dx;
                    y += dy;
                },
            }
        };

        // set the first box to empty
        let (x, y) = first;
        grid[y as usize][x as usize] = Pos::Empty;
        
        // move the robot
        self.x = x;
        self.y = y;

        // set the empty space to a box
        let (x, y) = space;
        grid[y as usize][x as usize] = Pos::Box;
        
        Ok(())

    }


}

impl Pos {
    fn from_char(c: char) -> Option<Pos> {
        match c {
            '#' => Some(Pos::Wall),
            'O' => Some(Pos::Box),
            '.' => Some(Pos::Empty),
            _ => None,
        }
    }
}

impl TryFrom<char> for Pos {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Self::from_char(c).ok_or(())
    }
}

impl Instruction {
    fn from_char(c: char) -> Option<Instruction> {
        match c {
            '^' => Some(Instruction::Up),
            'v' => Some(Instruction::Down),
            '<' => Some(Instruction::Left),
            '>' => Some(Instruction::Right),
            _ => None,
        }
    }

    fn to_direction(&self) -> (i64, i64) {
        match self {
            Instruction::Up => (0, -1),
            Instruction::Down => (0, 1),
            Instruction::Left => (-1, 0),
            Instruction::Right => (1, 0),
        }
    }
}

impl TryFrom<char> for Instruction {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Self::from_char(c).ok_or(())
    }
}

fn try_get(x: i64, y: i64, grid: &Grid) -> Result<Pos,String> {
    if x < 0 || y < 0 {
        return Err(String::from("Out of bounds"));
    }
    if y as usize >= grid.len() || x as usize >= grid[y as usize].len() {
        return Err(String::from("Out of bounds"));
    }
    Ok(grid[y as usize][x as usize])
}


fn parse(input: &String) -> (Grid, Instructions, Robot) {
    // split once by double newline
    let (grid_str, instructions_str) = input
        .split_once("\n\n")
        .or(input.split_once("\r\n\r\n"))
        .unwrap();

    let mut robot: Option<Robot> = None;
    let mut grid: Grid = Vec::new();
    let mut instructions: Instructions = Vec::new();

    for (y, line) in grid_str.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            if c == '@' && robot.is_none() {
                robot = Some(Robot::new(x, y));
                row.push(Pos::Empty);
            } else {
                row.push(c.try_into().unwrap());
            }
        }
        grid.push(row);
    }

    for c in instructions_str.chars() {
        if let Ok(i) = Instruction::try_from(c) {
            instructions.push(i);
        }
    }

    (grid, instructions, robot.unwrap())
}

#[cfg(test)]
fn print_map(grid: &Grid, robot: &Robot) {
    for (y, row) in grid.iter().enumerate() {
        for (x, p) in row.iter().enumerate() {
            if robot.x as usize == x && robot.y as usize == y {
                print!("@");
            } else {
                match p {
                    Pos::Wall => print!("#"),
                    Pos::Box => print!("O"),
                    Pos::Empty => print!("."),
                    Pos::LeftBox => print!("["),
                    Pos::RightBox => print!("]"),
                }
            }
        }
        println!();
    }
}

pub fn part1(input: &String) -> Box<dyn ToString> {
    let (mut grid, instructions, mut robot) = parse(input);

    #[cfg(test)]
    {
        for i in instructions {
            println!("Instruction: {:?}", i);

            if let Err(str) = robot.try_move(&mut grid, i) {
                println!("Error: {}", str);
            }
        
            print_map(&grid, &robot);
            println!();
        }
    }

    #[cfg(not(test))]
    {
        for i in instructions {
            let _ = robot.try_move(&mut grid, i);
        }
    }

    let mut gps = 0;
    for (y, row) in grid.into_iter().enumerate() {
        for (x, p) in row.into_iter().enumerate() {
            if p == Pos::Box {
                gps += 100 * y + x;
            }
        }
    }

    Box::new(
        i64::try_from(gps).unwrap()
    )
}

fn scale_up(grid: Vec<Vec<Pos>>) -> Vec<Vec<Pos>> {
    let mut new_grid = Vec::new();
    for row in grid {
        let mut new_row = Vec::new();
        for p in row {
            if p == Pos::Box {
                new_row.push(Pos::LeftBox);
                new_row.push(Pos::RightBox);
            } else {
                new_row.push(p);
                new_row.push(p);
            }
        }
        new_grid.push(new_row);
    }
    new_grid
}

pub fn part2(input: &String) -> Box<dyn ToString> {
    let (grid, instructions, mut robot) = parse(input);
    let mut grid = scale_up(grid);
    robot.x *= 2;


    fn try_move(grid: &Vec<Vec<Pos>>, (x,y): (i64,i64), instruction: &Instruction) -> Result<Vec<Vec<Pos>>,String> {
        let from = try_get(x, y, &grid)?;
        let (dx, dy) = instruction.to_direction();
        let tox = x + dx;
        let toy = y + dy;

        let to = try_get(tox, toy, &grid)?;

        match to {
            Pos::Wall => return Err("Cannot move into wall".to_string()),
            Pos::Empty => {
                let mut new_grid = grid.clone();
                new_grid[y as usize][x as usize] = Pos::Empty;
                new_grid[toy as usize][tox as usize] = from;
                Ok(new_grid)
            },
            Pos::LeftBox => {
                let mut new_grid = try_move(grid, (tox, toy), instruction)?;
                new_grid[y as usize][x as usize] = Pos::Empty;
                new_grid[toy as usize][tox as usize] = from;

                if *instruction == Instruction::Up || *instruction == Instruction::Down {                    
                    new_grid = try_move(&new_grid, (x + 1,toy), instruction)?;
                }
                Ok(new_grid)
            },
            Pos::RightBox => {                
                let mut new_grid = try_move(grid, (tox, toy), instruction)?;
                new_grid[y as usize][x as usize] = Pos::Empty;
                new_grid[toy as usize][tox as usize] = from;

                if *instruction == Instruction::Up || *instruction == Instruction::Down {
                    new_grid = try_move(&new_grid, (x - 1,toy), instruction)?;
                }

                Ok(new_grid)
            },
            _ => panic!("Unsupported position type for part 2")
        }
    }

    for instr in instructions {
        if let Ok(new_grid) = try_move(&grid, (robot.x, robot.y), &instr) {
            robot.x += instr.to_direction().0;
            robot.y += instr.to_direction().1;
            grid = new_grid;
        }
    }

    
    let mut gps = 0;
    for (y, row) in grid.into_iter().enumerate() {
        for (x, p) in row.into_iter().enumerate() {
            if p == Pos::LeftBox {
                gps += 100 * y + x;
            }
        }
    }

    Box::new(
        i64::try_from(gps).unwrap()
    )
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    const TEST_INPUT_SMALL: &str = indoc! {"
        ########
        #..O.O.#
        ##@.O..#
        #...O..#
        #.#.O..#
        #...O..#
        #......#
        ########

        <^^>>>vv<v>>v<<"
    };

    const TEST_RESULT_SMALL: i64 = 2028;
    const TEST_INPUT: &str = indoc! {"
        ##########
        #..O..O.O#
        #......O.#
        #.OO..O.O#
        #..O@..O.#
        #O#..O...#
        #O..O..O.#
        #.OO.O.OO#
        #....O...#
        ##########

        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
    };
    const TEST_RESULT: i64 = 10092;
    const TEST_RESULT2: i64 = 9021;

    // Test for part1
    #[test]
    fn test_part1_small() {
        assert_eq!(part1(&String::from(TEST_INPUT_SMALL)).to_string(), TEST_RESULT_SMALL.to_string());
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&String::from(TEST_INPUT)).to_string(), TEST_RESULT.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&String::from(TEST_INPUT)).to_string(), TEST_RESULT2.to_string());
    }
}