pub mod solution {
    use tracing::warn;

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let blocks: Vec<_> = input
            .chars()
            .flat_map(|c| c.to_digit(10))
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
                warn!(i, block, "\nfile");
                // unmoved file blocks
                let id = i / 2;
                let end = (pos + block).min(block_count);
                let sum = (pos..end)
                    .map(|pos| {
                        warn!(id, pos, val = id * pos);
                        id * pos
                    })
                    .sum();
                pos = end;
                sum
            } else {
                warn!(i, block, "\nfree");
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
                        sum += (pos..end)
                            .map(|pos| {
                                warn!(id, pos, val = id * pos);
                                id * pos
                            })
                            .sum::<usize>();
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
        todo!("b")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "1928";
    const EXPECTED_B: &str = "todo_expected_b";

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
