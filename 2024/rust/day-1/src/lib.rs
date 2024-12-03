pub mod solution {
    use std::collections::HashMap;

    #[tracing::instrument(fields(input = format!("{:?}[...]", input.lines().next())))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let (mut a, mut b): (Vec<_>, Vec<_>) = input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| {
                let mut nums = l.split_whitespace().map(|w| w.parse::<isize>().unwrap());
                (nums.next().unwrap(), nums.next().unwrap())
            })
            .unzip();
        a.sort_unstable();
        b.sort_unstable();
        let res: isize = a.iter().zip(&b).map(|(a, b)| (b - a).abs()).sum();
        Ok(res.to_string())
    }

    #[tracing::instrument(fields(input = format!("{:?}[...]", input.lines().next())))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        let (nums, counts) = input.lines().filter(|l| !l.is_empty()).fold(
            (Vec::new(), HashMap::new()),
            |(mut nums, mut counts), l| {
                let mut line_nums = l.split_whitespace().map(|w| w.parse::<isize>().unwrap());
                let a = line_nums.next().unwrap();
                nums.push(a);
                let b = line_nums.next().unwrap();
                counts.entry(b).and_modify(|count| *count += 1).or_insert(1);
                (nums, counts)
            },
        );
        let res: isize = nums.iter().map(|n| n * counts.get(n).unwrap_or(&0)).sum();
        Ok(res.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "11";
    const EXPECTED_B: &str = "31";

    #[test]
    fn day_1_a() {
        let res = solution::part_a(TEST_INPUT);
        assert_eq!(EXPECTED_A, res.unwrap());
    }

    #[test]
    fn day_1_b() {
        let res = solution::part_b(TEST_INPUT);
        assert_eq!(EXPECTED_B, res.unwrap());
    }
}
