use std::collections::HashSet;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

type Grid = Vec<Vec<char>>;
type Area = HashSet<(usize, usize)>;

pub fn make_grid(input: &String) -> Grid {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

// copied this from wikipedia (https://en.wikipedia.org/wiki/Flood_fill stack based not recursive)
fn flood_fill(grid: &Grid, x: usize, y: usize, visited: &mut Vec<Vec<bool>>) -> Area {
    let mut area = Area::new();
    let mut stack = vec![(x, y)];

    while let Some((x, y)) = stack.pop() {
        if visited[y][x] {
            continue;
        }
        visited[y][x] = true;

        area.insert((x, y));

        if x > 0 && grid[y][x - 1] == grid[y][x] {
            stack.push((x - 1, y));
        }
        if x < grid[y].len() - 1 && grid[y][x + 1] == grid[y][x] {
            stack.push((x + 1, y));
        }
        if y > 0 && grid[y - 1][x] == grid[y][x] {
            stack.push((x, y - 1));
        }
        if y < grid.len() - 1 && grid[y + 1][x] == grid[y][x] {
            stack.push((x, y + 1));
        }
    }

    area
}

fn perimeter(area: &Area) -> i64 {
    let mut perimeter = 0;
    for (x, y) in area {
        if !area.contains(&(x + 1, *y)) {
            perimeter += 1;
        }
        if *x == 0 || !area.contains(&(x - 1, *y)) {
            perimeter += 1;
        }
        if !area.contains(&(*x, y + 1)) {
            perimeter += 1;
        }
        if *y == 0 || !area.contains(&(*x, y - 1)) {
            perimeter += 1;
        }
    }
    perimeter
}

pub fn part1(input: &String) -> Box<dyn ToString> {
    let grid = make_grid(input);
    let mut areas: Vec<Area> = vec![];

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if visited[y][x] {
                continue;
            }
            let new_area = flood_fill(&grid, x, y, &mut visited);
            areas.push(new_area);
        }
    }

    Box::new(areas.into_par_iter().map(|a| {
        perimeter(&a) * a.len() as i64
    }).sum::<i64>())
}


fn sides(area: &Area) -> i64 {
    // we actually need to count corners

    let mut corners = 0;
    for (x, y) in area {
        // we use those multiple times, so we hash them only once here
        let tl = *x > 0 && *y > 0 && area.contains(&(x - 1, y - 1));
        let t = *y > 0 && area.contains(&(*x, y - 1));
        let tr = *y > 0 && area.contains(&(x + 1, y - 1));
        let r = area.contains(&(x + 1, *y));
        let br = area.contains(&(x + 1, y + 1));
        let b = area.contains(&(*x, y + 1));
        let bl = *x > 0 && area.contains(&(x - 1, y + 1));
        let l = *x > 0 && area.contains(&(x - 1, *y));

        // Convex corners
        // bottom right
        if !r && !b {
            corners += 1;
        }
        // top right
        if !r && !t {
            corners += 1;
        }
        // bottom left
        if !l && !b {
            corners += 1;
        }
        // top left
        if !l && !t {
            corners += 1;
        }

        // Concave corners
        // bottom right (there a cell to the right and below, but not to the bottom right)
        if r && b && !br {
            corners += 1;
        }
        // top right (there a cell to the right and above, but not to the top right)
        if r && t && !tr {
            corners += 1;
        }
        // bottom left (there a cell to the left and below, but not to the bottom left)
        if l && b && !bl {
            corners += 1;
        }
        // top left (there a cell to the left and above, but not to the top left)
        if l && t && !tl {
            corners += 1;
        }
    }
    corners
}

pub fn part2(input: &String) -> Box<dyn ToString> {
    let grid = make_grid(input);
    let mut areas: Vec<Area> = vec![];

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if visited[y][x] {
                continue;
            }
            let new_area = flood_fill(&grid, x, y, &mut visited);
            areas.push(new_area);
        }
    }

    Box::new(areas.into_par_iter().map(|a| {
        sides(&a) * a.len() as i64
    }).sum::<i64>())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    const TEST_SMALL: &str = indoc! {"
        AAAA
        BBCD
        BBCC
        EEEC"
    };
    const TEST_RESULT_SMALL: i64 = 140;
    const TEST_RESULT_SMALL_P2: i64 = 80;
    

    const TEST_SMALL2: &str = indoc! {"
        OOOOO
        OXOXO
        OOOOO
        OXOXO
        OOOOO"
    };
    const TEST_RESULT_SMALL2: i64 = 772;
    const TEST_RESULT_SMALL2_P2: i64 = 436;

    
    const TEST_INPUT: &str = indoc! {"
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE"
    };
    const TEST_RESULT1: i64 = 1930;
    const TEST_RESULT2: i64 = 1206;

    // Test for part1
    #[test]
    fn test_part1_small() {
        assert_eq!(part1(&String::from(TEST_SMALL)).to_string(), TEST_RESULT_SMALL.to_string());
        assert_eq!(part1(&String::from(TEST_SMALL2)).to_string(), TEST_RESULT_SMALL2.to_string());
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&String::from(TEST_INPUT)).to_string(), TEST_RESULT1.to_string());
    }

    // Test for part2
    #[test]
    fn test_part2_small() {
        assert_eq!(part2(&String::from(TEST_SMALL)).to_string(), TEST_RESULT_SMALL_P2.to_string());
        assert_eq!(part2(&String::from(TEST_SMALL2)).to_string(), TEST_RESULT_SMALL2_P2.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&String::from(TEST_INPUT)).to_string(), TEST_RESULT2.to_string());
    }
}