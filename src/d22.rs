use std::sync::atomic::{AtomicU64, Ordering};

use itertools::Itertools;
use rayon::prelude::*;

fn mix(num: u64, mix: u64) -> u64 {
    num ^ mix
}
fn prune(num: u64) -> u64 {
    num % 16777216
}
fn next(num: u64) -> u64 {
    let mut r = num * 64;
    let mut num = prune(mix(num, r));

    r = num / 32;
    num = prune(mix(num, r));

    r = num * 2048;
    num = prune(mix(num, r));

    num
}

#[derive(Clone, Copy)]
struct Generator {
    num: u64,
    digit: u8,
    counter: u64,
}

impl Iterator for Generator {
    type Item = (u8, i8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter == 2000 {
            return None;
        }
        self.counter += 1;
        let prev = self.digit;
        self.num = next(self.num);
        self.digit = (self.num % 10) as u8;

        Some((self.digit, self.digit as i8 - prev as i8))
    }
}

// just put the plain 8 bit representations in a u32
fn encode((n1, n2, n3, n4): (i8, i8, i8, i8)) -> u32 {
    ((n1 as u8 as u32) << 24) | ((n2 as u8 as u32) << 16) | ((n3 as u8 as u32) << 8) | (n4 as u8 as u32)
}

fn decode(num: u32) -> (i8, i8, i8, i8) {
    (
        (num >> 24) as i8,
        (num >> 16) as i8,
        (num >> 8) as i8,
        num as i8
    )
}

fn encode_sequences(num: u64) -> (Vec<u32>, Vec<u8>) {
    let gen = Generator { num, digit: (num % 10) as u8, counter: 0 };
    let mut sequences: Vec<u32> = vec![];
    let mut digits: Vec<u8> = vec![];

    for (n1, n2, n3, n4) in gen.tuple_windows() {
        sequences.push(encode((n1.1, n2.1, n3.1, n4.1)));
        digits.push(n4.0);
    }

    (sequences, digits)
} 

fn run2000(num: u64) -> u64 {
    let mut last = num;
    for _ in 0..2000 {
        last = next(last);
    }
    last
}

pub fn part1(input: &String) -> Box<dyn ToString> {
    Box::new(
        input.lines()
            .par_bridge()
            .map(|line| line.parse().unwrap())
            .map(run2000)
            .sum::<u64>()
    )
}

fn has_sequence_encoded(sequences: &Vec<u32>, digits: &Vec<u8>, combination: u32) -> Option<u8> {
    for (i, &seq) in sequences.iter().enumerate() {
        if seq == combination {
            return Some(digits[i]);
        }
    }

    None
}

fn run_partial_encoded(nums: &Vec<(Vec<u32>, Vec<u8>)>, n1: i8) -> u64 {
    let mut best_score: u64 = 0;

    for n2 in -9..=9 {
        for n3 in -9..=9 {
            for n4 in -9..=9 {
                let combination = encode((n1, n2, n3, n4));
                let s = nums.iter().filter_map(|num| has_sequence_encoded(&num.0, &num.1, combination)).map(|u8| u8 as u64).sum();
                if s > best_score {
                    println!("{:?}: {}", decode(combination), s);
                    best_score = s;
                }
            }
        }
    }

    best_score
}

pub fn part2(input: &String) -> Box<dyn ToString> {
    let nums: Vec<(Vec<u32>, Vec<u8>)> = input.lines().map(|line| encode_sequences(line.parse().unwrap())).collect();

    // mutex for best_score
    let best_score = AtomicU64::new(0);

    (-9..=9).par_bridge().for_each(|n1| {
        let best = run_partial_encoded(&nums, n1);
        best_score.fetch_max(best, Ordering::Relaxed);
    });

    Box::new(best_score.load(Ordering::Relaxed))
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    const TEST_INPUT: &str = indoc! {"
        1
        10
        100
        2024"
    };
    const TEST_INPUT_2: &str = indoc! {"
        1
        2
        3
        2024"
    };


    #[test]
    fn test_mix() {
        assert_eq!(mix(42,15), 37);
    }
    
    #[test]
    fn test_prune() {
        assert_eq!(prune(100000000), 16113920);
    }

    #[test]
    fn test_next() {
        let mut n = vec![];
        let mut last = 123;

        for _ in 0..10 {
            last = next(last);
            n.push(last);
        }

        assert_eq!(n, vec![
            15887950,
            16495136,
            527345,
            704524,
            1553684,
            12683156,
            11100544,
            12249484,
            7753432,
            5908254,
        ]);
    }

    #[test]
    fn test_2000() {
        assert_eq!(run2000(1), 8685429);
        assert_eq!(run2000(10), 4700978);
        assert_eq!(run2000(100), 15273692);
        assert_eq!(run2000(2024), 8667524);
    }
    
    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT.to_string()).to_string(), "37327623".to_string());
    }

    #[test]
    fn test_gen() {
        let gen = Generator { num: 123, digit: 3, counter: 0 };
        let n: Vec<(u8, i8)> = gen.take(9).collect();
        assert_eq!(n, vec![
            (0,-3),
            (6,6),
            (5,-1),
            (4,-1),
            (4,0),
            (6,2),
            (4,-2),
            (4,0),
            (2,-2),
        ]);
    }

    #[test]
    fn test_encode() {
        assert_eq!(encode((0,0,0,0)), 0);
        assert_eq!(encode((0,0,0,1)), 1);
        assert_eq!(encode((0,0,0,-1)), 255);
        assert_eq!(encode((0,0,1,0)), 256);
    }

    #[test]
    fn test_has_sequence_encoded() {
        let (seq, dig) = encode_sequences(1);
        assert_eq!(has_sequence_encoded(&seq, &dig, encode((-2,1,-1,3))), Some(7));
    }

    #[test]
    fn test_part2_encoded() {
        assert_eq!(part2(&TEST_INPUT_2.to_string()).to_string(), "23".to_string());
    }
}