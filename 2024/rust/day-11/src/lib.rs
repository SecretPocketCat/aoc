pub mod solution {
    use count_digits::CountDigits;
    use std::cell::LazyCell;
    use tracing::warn;

    const POWERS_OF_10: [u64; 20] = [
        1,
        10,
        100,
        1_000,
        10_000,
        100_000,
        1_000_000,
        10_000_000,
        100_000_000,
        1_000_000_000,
        10_000_000_000,
        100_000_000_000,
        1_000_000_000_000,
        10_000_000_000_000,
        100_000_000_000_000,
        1_000_000_000_000_000,
        10_000_000_000_000_000,
        100_000_000_000_000_000,
        1_000_000_000_000_000_000,
        10_000_000_000_000_000_000,
    ];

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        Ok(eval(input, 25).to_string())
    }

    #[tracing::instrument(skip(input))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        Ok(eval(input, 75).to_string())
    }

    pub fn eval_num(num: u64, iterations: u8) -> u64 {
        if iterations == 0 {
            return 1;
        }

        let rem_iter = iterations - 1;
        let digits = LazyCell::new(|| num.count_digits());
        match num {
            0 => eval_num(1, rem_iter),
            n if *digits % 2 == 0 => {
                let digits = *digits;
                let divisor = POWERS_OF_10[digits / 2];
                let a = n / divisor;
                let b = n % divisor;
                eval_num(a, rem_iter) + eval_num(b, rem_iter)
            }
            n => eval_num(n * 2024, rem_iter),
        }
    }

    pub fn eval(input: &str, iterations: u8) -> u64 {
        input
            .split_whitespace()
            .flat_map(str::parse)
            .map(|num| eval_num(num, iterations))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "55312";

    #[test]
    #[traced_test]
    fn day_11_a() {
        let res = solution::part_a(TEST_INPUT);
        assert_eq!(EXPECTED_A, res.unwrap());
    }

    #[test_case(0, 1 => 1)]
    #[test_case(0, 2 => 1)]
    #[test_case(0, 3 => 2)]
    #[test_case(1, 1 => 1)]
    #[test_case(2024, 1 => 2)]
    #[test_case(99, 1 => 2)]
    #[test_case(99, 2 => 2)]
    #[traced_test]
    fn day_11_eval_num(num: u64, iterations: u8) -> u64 {
        solution::eval_num(num, iterations)
    }
}
