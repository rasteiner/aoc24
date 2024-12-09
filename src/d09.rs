
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Space,
    File(i64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ChunkKind {
    File(i64),
    Space,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Chunk {
    kind: ChunkKind,
    size: i64,
}

fn parse(input: &String) -> Vec<Block> {
    let mut id = 0;
    let mut disk: Vec<Block> = Vec::new();
    
    let mut is_file = true;
    for c in input.chars().map(|c| c.to_string().parse::<i64>().unwrap()) {

        if c > 0 {
            if is_file {
                for _ in 0..c {
                    disk.push(Block::File(id));
                }
                id += 1;
            } else {
                for _ in 0..c {
                    disk.push(Block::Space);
                }
            }
        }

        is_file = !is_file;
    }
    
    disk
}

fn checksum(disk: &Vec<Block>) -> i64 {
    let mut checksum = 0;
    for (i, b) in disk.into_iter().enumerate() {
        if let Block::File(id) = b {
            checksum += id * (i as i64);
        }
    }
    checksum
}

pub fn part1(input: &String) -> i64 {
    let disk = parse(input);
    let mut fragmented = Vec::new();

    let filesize = disk.iter().filter(|&x| match x {
        Block::File(_) => true,
        _ => false,
    }).count();

    let disk_clone = disk.clone();
    let mut reverse_files = disk_clone.iter().rev().filter(|&x| match x {
        Block::File(_) => true,
        _ => false,
    }).into_iter();
    
    for b in disk.into_iter() {
        if b == Block::Space {
            if let Some(f) = reverse_files.next() {
                fragmented.push(*f);
            } else {
                break;
            }
        } else {
            if fragmented.len() < filesize {
                fragmented.push(b);
            } else {
                break;
            }
        }
    }

    checksum(&fragmented)
}

fn parse2(input: &String) -> Vec<Chunk> {
    let mut id = 0;
    let mut disk: Vec<Chunk> = Vec::new();
    
    let mut is_file = true;
    for c in input.chars().map(|c| c.to_string().parse::<i64>().unwrap()) {

        if c > 0 {
            if is_file {
                disk.push(Chunk { kind: ChunkKind::File(id), size: c });
                id += 1;
            } else {
                disk.push(Chunk { kind: ChunkKind::Space, size: c });
            }
        }

        is_file = !is_file;
    }
    
    disk

}

pub fn part2(input: &String) -> i64 {
    let mut disk = parse2(input);

    for chunk in disk.clone().into_iter().rev() {
    
        match chunk.kind {
            ChunkKind::File(id) => {
                if let Some(space_i) = disk.iter().position(|c| match c.kind {
                    ChunkKind::Space => c.size >= chunk.size,
                    _ => false,
                }) {

                    // I need to search for the correct index of the file we are moving
                    // because when inserting extra space chunks, the index of the file might change
                    // this makes it a bit slower, but it's the only way I could think of.
                    // At least we actually only search for useful indices (the ones that come after the space chunk)
                    let file_i = disk[space_i+1..].iter().position(|c| match c.kind {
                        ChunkKind::File(i) => i == id,
                        _ => false,
                    });

                    if file_i.is_none() {
                        continue;
                    }

                    let file_i = space_i + 1 + file_i.unwrap();                                        

                    let space = disk[space_i].size;
                    disk[file_i] = Chunk {kind: ChunkKind::Space, size: chunk.size};
                    disk[space_i] = Chunk {kind: ChunkKind::File(id), size: chunk.size};

                    if space > chunk.size {
                        disk.insert(space_i + 1, Chunk {kind: ChunkKind::Space, size: space - chunk.size});
                    }
                }
            },
            _ => (),
        };
    }


    let mut compact = Vec::new();
    for chunk in disk.into_iter() {
        match chunk.kind {
            ChunkKind::File(id) => for _ in 0..chunk.size { compact.push(Block::File(id)) },
            ChunkKind::Space => for _ in 0..chunk.size { compact.push(Block::Space) },
        }
    }

    checksum(&compact)

}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2333133121414131402";
    const TEST_RESULT1: i64 = 1928;
    const TEST_RESULT2: i64 = 2858;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&String::from(TEST_INPUT)), TEST_RESULT1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&String::from(TEST_INPUT)), TEST_RESULT2);
    }
}