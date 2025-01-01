fn parse(input: &String) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    let mut lines = input.lines();


    while let Some(line) = lines.next() {
        let block: Vec<Vec<char>> = lines.by_ref().take_while(|l| !l.is_empty()).map(|l| l.chars().collect()).collect();
        let mut thing = vec![0u8;5];

        if line.chars().nth(0) == Some('#') {
            for i in 0..5 {
                thing[i] = block.iter().position(|line| line[i] == '.').unwrap() as u8;
            }
            locks.push(thing);
        } else {
            for i in 0..5 {
                thing[i] = 5 - block.iter().position(|line| line[i] == '#').unwrap() as u8;
            }
            keys.push(thing);
        }
    }

    (locks, keys)
}

pub fn part1(input: &String) -> Box<dyn ToString> {
    let (locks, keys) = parse(input);

    let fits = |key: &Vec<u8>, lock: &Vec<u8>| {
        key.iter().zip(lock.iter()).all(|(a,b)| a + b <= 5)
    };

    let mut count = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            if fits(key, lock) {
                count += 1;
            }
        }
    }

    Box::new(count)
}


pub fn part2(input: &String) -> Box<dyn ToString> {
    
    Box::new(0)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    const TEST_INPUT: &str = indoc! {"
        #####
        .####
        .####
        .####
        .#.#.
        .#...
        .....

        #####
        ##.##
        .#.##
        ...##
        ...#.
        ...#.
        .....

        .....
        #....
        #....
        #...#
        #.#.#
        #.###
        #####

        .....
        .....
        #.#..
        ###..
        ###.#
        ###.#
        #####

        .....
        .....
        .....
        #....
        #.#..
        #.#.#
        #####
    "};

    #[test] 
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT.to_string()).to_string(), "3".to_string());
    }

    #[test] 
    fn test_part2() {

    }
}