pub mod solution {
    use itertools::Itertools;
    use std::collections::HashMap;

    #[derive(PartialEq, Eq, Hash, Debug)]
    enum Gate<'a> {
        And(&'a str, &'a str),
        Or(&'a str, &'a str),
        Xor(&'a str, &'a str),
    }

    #[derive(Debug)]
    struct Gates<'a> {
        resolved: HashMap<&'a str, bool>,
        gates: HashMap<&'a str, Gate<'a>>,
    }
    impl<'a> Gates<'a> {
        fn resolve_gate(&mut self, key: &'a str) -> bool {
            match self.resolved.get(key) {
                Some(bit) => *bit,
                None => {
                    let bit = match self.gates[key] {
                        Gate::And(a, b) => self.resolve_gate(a) & self.resolve_gate(b),
                        Gate::Or(a, b) => self.resolve_gate(a) | self.resolve_gate(b),
                        Gate::Xor(a, b) => self.resolve_gate(a) ^ self.resolve_gate(b),
                    };
                    self.resolved.insert(key, bit);
                    bit
                }
            }
        }

        fn solve(&mut self) -> usize {
            self.gates
                .keys()
                .filter(|k| k.starts_with('z'))
                .copied()
                .sorted_unstable()
                .enumerate()
                .map(|(i, k)| usize::from(self.resolve_gate(k)) << i)
                .sum()
        }
    }

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let mut gates = parse_input(input);
        let num = gates.solve();
        Ok(num.to_string())
    }

    fn parse_gate(line: &str) -> (&str, Gate) {
        let parts: Vec<_> = line.split(' ').collect();
        let op = parts[1];
        let a = parts[0];
        let b = parts[2];
        let key = parts[4];
        (
            key,
            match op {
                "AND" => Gate::And(a, b),
                "OR" => Gate::Or(a, b),
                "XOR" => Gate::Xor(a, b),
                _ => unimplemented!("Unknown op"),
            },
        )
    }

    fn parse_input(input: &str) -> Gates {
        let mut lines = input.lines();
        let resolved: HashMap<_, _> = lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .map(|l| {
                let key = &l[0..=2];
                let bit = l.chars().nth(5).unwrap().to_digit(10).expect("Gate bit") != 0;
                (key, bit)
            })
            .collect();
        let gates: HashMap<_, _> = lines.map(parse_gate).collect();
        Gates { resolved, gates }
    }

    #[tracing::instrument(skip(input))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        todo!("b")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "2024";
    const EXPECTED_B: &str = "todo_expected_b";

    #[test]
    #[traced_test]
    fn day_24_a() {
        let res = solution::part_a(TEST_INPUT);
        assert_eq!(EXPECTED_A, res.unwrap());
    }

    #[test]
    #[traced_test]
    fn day_24_b() {
        let res = solution::part_b(TEST_INPUT);
        assert_eq!(EXPECTED_B, res.unwrap());
    }
}
