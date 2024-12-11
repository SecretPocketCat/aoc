pub mod solution {
    use count_digits::CountDigits;
    use itertools::{repeat_n, Itertools};
    use math::POWERS_OF_10;
    use rayon::prelude::*;
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
        let lines: Vec<_> = input.lines().collect();
        let sum: u64 = lines
            .into_par_iter()
            .filter_map(|l| {
                let (total, nums) = l.split_once(':').expect("Valid example line");
                let total: u64 = total.parse().expect("Valid total number");
                let nums: Vec<u64> = nums.split_whitespace().flat_map(str::parse).collect();
                repeat_n(operations.iter().copied(), nums.len() - 1)
                    .multi_cartesian_product()
                    .any(|ops| eval_ops(nums[0], 0, &nums, &ops) == total)
                    .then_some(total)
            })
            .sum();
        Ok(sum.to_string())
    }

    fn eval_ops(val: u64, i: usize, nums: &[u64], ops: &[Operation]) -> u64 {
        let Some(op) = ops.get(i) else {
            return val;
        };
        let next = nums[i + 1];
        match op {
            Operation::Addition => eval_ops(val + next, i + 1, nums, ops),
            Operation::Multiplication => eval_ops(val * next, i + 1, nums, ops),
            Operation::Concat => eval_ops(
                val * POWERS_OF_10[next.count_digits()] + next,
                i + 1,
                nums,
                ops,
            ),
        }
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
