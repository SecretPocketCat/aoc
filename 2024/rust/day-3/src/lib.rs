pub mod solution {
    #[tracing::instrument(fields(input = format!("{:?}[...]", input.lines().next())))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let rgx = regex::Regex::new(r"^(\d+),(\d+)\)").expect("Valid regex");
        let sum: u64 = input
            .split("mul(")
            .filter_map(|s| match rgx.captures(s) {
                Some(captures) => match (captures.get(1), captures.get(2)) {
                    (Some(a), Some(b)) => Some(
                        a.as_str()
                            .parse::<u64>()
                            .expect("Valid digit matched by regex")
                            * b.as_str()
                                .parse::<u64>()
                                .expect("Valid digit matched by regex"),
                    ),
                    _ => None,
                },
                None => None,
            })
            .sum();
        Ok(sum.to_string())
    }

    #[tracing::instrument(fields(input = format!("{:?}[...]", input.lines().next())))]

    pub fn part_b(input: &str) -> anyhow::Result<String> {
        let rgx = regex::Regex::new(r"^(\d+),(\d+)\)").expect("Valid regex");
        let sum_muls = |s: &str| {
            s.split("mul(")
                .filter_map(|s| match &rgx.captures(s) {
                    Some(captures) => match (captures.get(1), captures.get(2)) {
                        (Some(a), Some(b)) => Some(
                            a.as_str()
                                .parse::<u64>()
                                .expect("Valid digit matched by regex")
                                * b.as_str()
                                    .parse::<u64>()
                                    .expect("Valid digit matched by regex"),
                        ),
                        _ => None,
                    },
                    None => None,
                })
                .sum::<u64>()
        };
        let sum: u64 = input
            .split("don't()")
            .enumerate()
            .filter_map(|(i, s)| match i {
                0 => Some(sum_muls(s)),
                _ => s.split_once("do()").map(|(_, s)| sum_muls(s)),
            })
            .sum();
        Ok(sum.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED_A: &str = "161";
    const EXPECTED_B: &str = "48";

    #[test]
    fn day_3_a() {
        let res = solution::part_a(include_str!("../inputs/example_a.txt"));
        assert_eq!(EXPECTED_A, res.unwrap());
    }

    #[test]
    fn day_3_b() {
        let res = solution::part_b(include_str!("../inputs/example_b.txt"));
        assert_eq!(EXPECTED_B, res.unwrap());
    }
}
