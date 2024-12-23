pub mod solution {
    use std::collections::{HashMap, HashSet};

    use itertools::Itertools;
    use rayon::prelude::*;

    const STEP_COUNT: u32 = 2000;

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let nums = parse_nums(input);
        let res: usize = nums
            .into_par_iter()
            .map(|n| (0..STEP_COUNT).fold(n, |acc, _| secret_number(acc)))
            .sum();
        Ok(res.to_string())
    }

    #[tracing::instrument(skip(input))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        let res = solve_b(input, STEP_COUNT);
        Ok(res.to_string())
    }

    pub(crate) fn solve_b(input: &str, step_count: u32) -> usize {
        let nums = parse_nums(input);
        let monkey_sequences: Vec<_> = nums
            .into_par_iter()
            .map(|n| {
                let mut acc = Vec::with_capacity(step_count as _);
                acc.push((n % 10) as u8);
                (0..step_count).fold((n, acc), |(num, mut units), _| {
                    let num = secret_number(num);
                    let num_units = num % 10;
                    units.push(num_units as u8);
                    (num, units)
                })
            })
            .map(|(_, units)| {
                let capacity = units.len();
                units
                    .windows(5)
                    .fold(HashMap::with_capacity(capacity), |mut acc, sequence| {
                        let seq: (_, _, _, _) = sequence
                            .windows(2)
                            .map(|win| win[0] as i8 - win[1] as i8)
                            .collect_tuple()
                            .expect("Valid sequence tuple");
                        acc.entry(seq).or_insert(*sequence.last().unwrap());
                        acc
                    })
            })
            .collect();
        let all_sequences: HashSet<_> = monkey_sequences
            .iter()
            .flat_map(|monkey| monkey.keys())
            .collect();
        all_sequences
            .into_par_iter()
            .map(|seq| {
                monkey_sequences
                    .iter()
                    .map(|monkey| *monkey.get(seq).unwrap_or(&0) as usize)
                    .sum::<usize>()
            })
            .max()
            .expect("At least one sequence")
    }

    fn parse_nums(input: &str) -> Vec<usize> {
        input.lines().flat_map(str::parse).collect()
    }

    pub(crate) fn secret_number(num: usize) -> usize {
        let num_1 = mix_and_prune(num << 6, num);
        let num_2 = mix_and_prune(num_1 >> 5, num_1);
        let num_3 = mix_and_prune(num_2 << 11, num_2);
        num_3
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

    #[test_case(123 => 15887950)]
    #[traced_test]
    fn day_22_secret_number(num: usize) -> usize {
        solution::secret_number(num)
    }

    #[test]
    #[traced_test]
    fn day_22_a() {
        let res = solution::part_a(include_str!("../inputs/example.txt"));
        assert_eq!("37327623", res.unwrap());
    }

    #[test]
    #[traced_test]
    fn day_22_b() {
        let res = solution::part_b(include_str!("../inputs/example_b.txt"));
        assert_eq!("23", res.unwrap());
    }

    #[test]
    #[traced_test]
    fn day_22_b_simple() {
        let res = solution::solve_b("123", 10);
        assert_eq!(6, res);
    }
}
