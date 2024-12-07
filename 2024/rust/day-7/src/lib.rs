pub mod solution {
    // use std::collections::HashSet;

    use itertools::{repeat_n, Itertools};
    use tracing::warn;

    #[derive(Debug, Clone, Copy)]
    enum Operation {
        Addition,
        Multiplication,
        Concat,
    }

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        eval(input, &[Operation::Addition, Operation::Multiplication])
    }

    #[tracing::instrument(fields(input = format!("{:?}[...]", input.lines().next())))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        eval(
            input,
            &[
                Operation::Addition,
                Operation::Multiplication,
                Operation::Concat,
            ],
        )
    }

    #[tracing::instrument(skip_all)]
    pub fn eval(input: &str, operations: &[Operation]) -> anyhow::Result<String> {
        let sum: u64 = input
            .lines()
            .filter_map(|l| {
                let (total, nums) = l.split_once(':').expect("Valid example line");
                let total: u64 = total.parse().expect("Valid total number");
                let nums: Vec<u64> = nums.split_whitespace().flat_map(str::parse).collect();
                let permutations: Vec<_> = repeat_n(operations.iter(), nums.len() - 1)
                    .multi_cartesian_product()
                    .collect();
                // todo:
                // let mut cache = HashSet::new();
                for ops in permutations {
                    let mut res = nums[0];
                    for (op_i, op) in ops.iter().enumerate() {
                        res = match op {
                            Operation::Addition => res + nums[op_i + 1],
                            Operation::Multiplication => res * nums[op_i + 1],
                            Operation::Concat => (res.to_string() + &nums[op_i + 1].to_string())
                                .parse()
                                .expect("Concatenated number"),
                        };
                    }
                    if res == total {
                        return Some(res);
                    }
                }

                None
            })
            .sum();
        Ok(sum.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "3749";
    const EXPECTED_B: &str = "11387";

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
