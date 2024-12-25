pub mod solution {
    // note: tried rayon to paralellize the parsing and the combo count but it was  slower or just very slightly faster respectively
    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let lines: Vec<_> = input.lines().collect();
        let parsed_chunks: Vec<_> = lines
            .chunks(8)
            .map(|chunk_lines| {
                let is_lock = chunk_lines[0].starts_with('#');
                let pins = chunk_lines
                    .iter()
                    .skip(1)
                    .take(5)
                    .fold([0u8; 5], |mut acc, l| {
                        for (i, c) in l.chars().enumerate() {
                            if c == '#' {
                                acc[i] += 1;
                            }
                        }
                        acc
                    });
                (pins, is_lock)
            })
            .collect();
        let capacity = parsed_chunks.len();
        let (locks, keys) = parsed_chunks.into_iter().fold(
            (Vec::with_capacity(capacity), Vec::with_capacity(capacity)),
            |(mut locks, mut keys), (pins, is_lock)| {
                (if is_lock { &mut locks } else { &mut keys }).push(pins);
                (locks, keys)
            },
        );
        let valid_combo_count: usize = locks
            .into_iter()
            .map(|lock_pins| {
                keys.iter()
                    .filter(|key_pins| (0..5).all(|i| lock_pins[i] + key_pins[i] <= 5))
                    .count()
            })
            .sum();
        Ok(valid_combo_count.to_string())
    }

    #[tracing::instrument(skip(_input))]
    pub fn part_b(_input: &str) -> anyhow::Result<String> {
        Ok("Do all the stars".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "3";

    #[test]
    #[traced_test]
    fn day_25_a() {
        let res = solution::part_a(TEST_INPUT);
        assert_eq!(EXPECTED_A, res.unwrap());
    }
}
