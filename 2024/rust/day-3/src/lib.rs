pub mod solution {
    #[tracing::instrument(fields(input = format!("{:?}[...]", input.lines().next())))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let rgx = regex::Regex::new(r"^(\d+),(\d+)\)").expect("Valid regex");
        let count: u64 = input
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
    const EXPECTED_A: &str = "161";
    const EXPECTED_B: &str = "todo_expected_b";

    #[test]
    fn day_3_a() {
        let res = solution::part_a(TEST_INPUT);
        assert_eq!(EXPECTED_A, res.unwrap());
    }

    #[test]
    fn day_3_b() {
        let res = solution::part_b(TEST_INPUT);
        assert_eq!(EXPECTED_B, res.unwrap());
    }
}
