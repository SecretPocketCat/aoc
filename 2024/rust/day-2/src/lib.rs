pub mod solution {
    #[tracing::instrument(fields(input = format!("{:?}[...]", input.lines().next())))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let count = input
            .lines()
            .filter(|l| {
                let chars: Vec<_> = l
                    .split_whitespace()
                    .map(|num| num.parse::<i64>())
                    .flatten()
                    .collect();
                let mut inc = None;
                chars.windows(2).all(|nums| {
                    let delta = nums[1] - nums[0];
                    let sign = delta > 0;
                    match inc {
                        Some(inc) if inc != sign => {
                            return false;
                        }
                        None => {
                            inc = Some(delta > 0);
                        }
                        _ => {}
                    }

                    (1..=3).contains(&delta.abs())
                })
            })
            .count();
        Ok(count.to_string())
    }

    #[tracing::instrument(fields(input = format!("{:?}[...]", input.lines().next())))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        todo!("b")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "2";
    const EXPECTED_B: &str = "todo_expected_b";

    #[test]
    fn day_2_a() {
        let res = solution::part_a(TEST_INPUT);
        assert_eq!(EXPECTED_A, res.unwrap());
    }

    #[test]
    fn day_2_b() {
        let res = solution::part_b(TEST_INPUT);
        assert_eq!(EXPECTED_B, res.unwrap());
    }
}
