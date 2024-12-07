pub mod solution {
    // use std::collections::HashSet;

    use itertools::{repeat_n, Itertools};
    use tracing::warn;

    #[derive(Debug, Clone, Copy)]
    enum Operation {
        Addition,
        Multiplication,
    }

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let sum: u64 = input
            .lines()
            .filter_map(|l| {
                let (total, nums) = l.split_once(':').expect("Valid example line");
                let total: u64 = total.parse().expect("Valid total number");
                let nums: Vec<u64> = nums.split_whitespace().flat_map(str::parse).collect();
                let permutations: Vec<_> = repeat_n(
                    [Operation::Addition, Operation::Multiplication].into_iter(),
                    nums.len() - 1,
                )
                .multi_cartesian_product()
                .collect();
                // todo:
                // let mut cache = HashSet::new();
                for ops in permutations {
                    warn!(?ops);
                    let mut res = nums[0];
                    for (op_i, op) in ops.iter().enumerate() {
                        res = match op {
                            Operation::Addition => res + nums[op_i + 1],
                            Operation::Multiplication => res * nums[op_i + 1],
                        };
                    }
                    warn!(total, res, ?nums, ?ops);
                    if res == total {
                        return Some(res);
                    }
                }

                None
            })
            .sum();
        Ok(sum.to_string())
    }

    #[tracing::instrument(fields(input = format!("{:?}[...]", input.lines().next())))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        todo!("b")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "3749";
    const EXPECTED_B: &str = "todo_expected_b";

    #[test]
    #[traced_test]
    fn day_7_a() {
        let res = solution::part_a(TEST_INPUT);
        assert_eq!(EXPECTED_A, res.unwrap());
    }

    #[test]
    #[traced_test]
    fn day_7_b() {
        let res = solution::part_b(TEST_INPUT);
        assert_eq!(EXPECTED_B, res.unwrap());
    }
}
