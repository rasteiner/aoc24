pub fn part1(input: &String) -> i64 {
    let mut count = 0;
    let matrix = input.lines().map(|line| line.trim().bytes().collect::<Vec<u8>>()).collect::<Vec<Vec<u8>>>();

    let directions = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    const XMAS: [u8;4] = *b"XMAS";

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if matrix[y][x] != XMAS[0] {
                continue;
            }

            'nextDir: for (dx, dy) in directions.iter() {
                let mut mx = x as i64 + dx;
                let mut my = y as i64 + dy;

                for i in 1..XMAS.len() {
                    if mx < 0 || my < 0 || my >= matrix.len() as i64 || mx >= matrix[my as usize].len() as i64 {
                        continue 'nextDir;
                    }

                    if matrix[my as usize][mx as usize] != XMAS[i] {
                        continue 'nextDir;
                    }

                    mx += dx;
                    my += dy;
                }

                count += 1;
            }
        }
    }
    count
}

pub fn part2(input: &String) -> i64 {
    let mut count = 0;
    let matrix = input.lines().map(|line| line.trim().bytes().collect::<Vec<u8>>()).collect::<Vec<Vec<u8>>>();

    let corners = [
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    for y in 1..matrix.len()-1 {
        for x in 1..matrix[y].len()-1 {
            if matrix[y][x] != 'A' as u8 {
                continue;
            }

            let vals = corners.iter().map(|(dx, dy)| {
                let mx = x as i64 + dx;
                let my = y as i64 + dy;
                
                matrix[my as usize][mx as usize]
            }).collect::<Vec<u8>>();

            match vals.as_slice() {
                b"MMSS" | b"SSMM" | b"SMSM" | b"MSMS" => count += 1,
                _ => (),
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;
    
    const TEST_INPUT: &str = indoc! {"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX"
    };
    const TEST_RESULT1: i64 = 18;
    const TEST_RESULT2: i64 = 9;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&String::from(TEST_INPUT)), TEST_RESULT1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&String::from(TEST_INPUT)), TEST_RESULT2);
    }
}