pub mod solution {
    use std::collections::HashSet;

    use tracing::warn;

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let blocks: Vec<_> = input
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|n| n as usize)
            .collect();
        let mut file_blocks: Vec<_> = blocks.iter().copied().step_by(2).collect();
        let block_count = file_blocks.iter().sum();
        let mut checksum = 0;
        let mut pos = 0;
        for (i, block) in blocks.iter().enumerate() {
            if pos >= block_count {
                break;
            }
            let file = i % 2 == 0;
            checksum += if file {
                // unmoved file blocks
                let id = i / 2;
                let end = (pos + block).min(block_count);
                let sum = (pos..end).map(|pos| id * pos).sum();
                pos = end;
                sum
            } else {
                let mut sum = 0;
                let mut free_block_size = *block;
                while free_block_size > 0 {
                    if let Some(file_block_size) = file_blocks.last().copied() {
                        let moved_i = file_blocks.len() - 1;
                        let moved_block_size = if free_block_size >= file_block_size {
                            file_blocks.pop();
                            free_block_size -= file_block_size;
                            file_block_size
                        } else {
                            file_blocks[moved_i] = file_block_size - free_block_size;
                            let res = free_block_size;
                            free_block_size = 0;
                            res
                        };
                        let id = moved_i;
                        let end = (pos + moved_block_size).min(block_count);
                        sum += (pos..end).map(|pos| id * pos).sum::<usize>();
                        pos = end;
                        if pos >= block_count {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                sum
            };
        }
        Ok(checksum.to_string())
    }

    #[tracing::instrument(skip(input))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        let blocks: Vec<_> = input
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|n| n as usize)
            .collect();
        let mut file_blocks: Vec<_> = blocks.iter().copied().step_by(2).enumerate().collect();
        let mut processable_files: HashSet<_> = file_blocks.iter().map(|(id, _)| *id).collect();
        let mut checksum = 0;
        let mut pos = 0;
        let mut dbg_str = String::new();
        for (i, block) in blocks.iter().enumerate() {
            let file = i % 2 == 0;
            checksum += if file {
                // unmoved file blocks
                let id = i / 2;
                if !processable_files.contains(&id) {
                    pos += block;
                    continue;
                }
                let end = pos + block;
                let sum = (pos..end).map(|pos| id * pos).sum();
                dbg_str.push_str(&(pos..end).map(|_| id.to_string()).collect::<String>());
                pos = end;
                sum
            } else {
                let mut block_rem = *block;
                let mut sum = 0;
                while block_rem > 0 {
                    if let Some(f_i) = file_blocks
                        .iter()
                        .rposition(|(file_id, b)| (*file_id * 2) > i && *b <= block_rem)
                    {
                        let (file_id, file_block_size) = file_blocks.remove(f_i);
                        processable_files.remove(&file_id);
                        let end = pos + file_block_size;
                        sum += (pos..end).map(|pos| file_id * pos).sum::<usize>();
                        dbg_str
                            .push_str(&(pos..end).map(|_| file_id.to_string()).collect::<String>());
                        block_rem -= file_block_size;
                        pos += file_block_size;
                    } else {
                        break;
                    }
                }
                pos += block_rem;
                dbg_str.push_str(&(0..block_rem).map(|_| '.').collect::<String>());
                sum
            };
        }
        Ok(checksum.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "1928";
    const EXPECTED_B: &str = "2858";

    #[test]
    #[traced_test]
    fn day_9_a() {
        let res = solution::part_a(TEST_INPUT);
        assert_eq!(EXPECTED_A, res.unwrap());
    }

    #[test]
    #[traced_test]
    fn day_9_a_simple() {
        let res = solution::part_a("12345");
        assert_eq!("60", res.unwrap());
    }

    #[test]
    #[traced_test]
    fn day_9_b() {
        let res = solution::part_b(TEST_INPUT);
        assert_eq!(EXPECTED_B, res.unwrap());
    }
}
