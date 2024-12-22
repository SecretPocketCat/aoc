pub mod solution {
    use rayon::prelude::*;

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let nums: Vec<usize> = input.lines().flat_map(str::parse).collect();
        let res: usize = nums.into_par_iter().map(|n| secret_number(n, 2000)).sum();
        Ok(res.to_string())
    }

    #[tracing::instrument(skip(input))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        todo!("b")
    }

    pub(crate) fn secret_number(num: usize, step_count: u32) -> usize {
        (0..step_count).fold(num, |acc, _| {
            let num_1 = mix_and_prune(acc << 6, acc);
            let num_2 = mix_and_prune(num_1 >> 5, num_1);
            let num_3 = mix_and_prune(num_2 << 11, num_2);
            num_3
        })
    }

    fn mix_and_prune(num: usize, secret: usize) -> usize {
        (num ^ secret) % 16777216
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "37327623";
    const EXPECTED_B: &str = "todo_expected_b";

    #[test_case(123, 1 => 15887950)]
    #[test_case(123, 2 => 16495136)]
    #[test_case(123, 3 => 527345)]
    #[test_case(123, 10 => 5908254)]
    #[traced_test]
    fn day_22_secret_number(num: usize, step_count: u32) -> usize {
        solution::secret_number(num, step_count)
    }

    #[test]
    #[traced_test]
    fn day_22_a() {
        let res = solution::part_a(TEST_INPUT);
        assert_eq!(EXPECTED_A, res.unwrap());
    }

    // #[test]
    // #[traced_test]
    // fn day_22_b() {
    //     let res = solution::part_b(TEST_INPUT);
    //     assert_eq!(EXPECTED_B, res.unwrap());
    // }
}
