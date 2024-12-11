pub mod solution {
    use count_digits::CountDigits;
    use std::cell::LazyCell;
    use tracing::warn;

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        Ok(eval(input, 25).to_string())
    }

    #[tracing::instrument(skip(input))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        Ok(eval(input, 75).to_string())
    }

    pub fn eval_nums(nums: &mut Vec<u64>) {
        for i in (0..nums.len()).rev() {
            let n = nums[i];
            let digits = LazyCell::new(|| n.count_digits());
            nums[i] = match n {
                0 => 1,
                n if *digits % 2 == 0 => {
                    let digits = *digits;
                    let a = n / 10u64.pow((digits / 2) as _);
                    let b = n % 10u64.pow((digits / 2) as _);
                    nums.insert(i + 1, b);
                    a
                }
                n => n * 2024,
            };
        }
    }

    pub fn eval(input: &str, iterations: u8) -> usize {
        let mut nums: Vec<u64> = input.split_whitespace().flat_map(str::parse).collect();
        for _ in 0..iterations {
            eval_nums(&mut nums);
        }
        nums.len()
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

    #[test_case(vec![0, 1, 10, 99, 999] => vec![1, 2024, 1, 0, 9, 9, 2_021_976])]
    #[test_case(vec![125, 17] => vec![253_000, 1, 7])]
    #[test_case(vec![253_000, 1, 7] => vec![253, 0, 2024, 14_168])]
    #[test_case(vec![253, 0, 2024, 14_168] => vec![512_072, 1, 20, 24, 28_676_032])]
    #[test_case(vec![512_072, 1, 20, 24, 28_676_032] => vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032])]
    #[test_case(vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032] => vec![1_036_288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32])]
    #[traced_test]
    fn day_11_eval_nums(mut nums: Vec<u64>) -> Vec<u64> {
        solution::eval_nums(&mut nums);
        nums
    }
}
