// struct OrderedNum {
//     after: HashSet<u8>,
//     before: HashSet<u8>,
// }

pub mod solution {
    use std::collections::{HashMap, HashSet};

    #[tracing::instrument(fields(input = format!("{:?}[...]", input.lines().next())))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let mut following = HashMap::new();
        let mut processed_line_count = 0;
        for line in input.lines() {
            processed_line_count += 1;
            match line.split_once('|') {
                Some((a, b)) => {
                    let a: u32 = a.parse().expect("Valid number a");
                    let b: u32 = b.parse().expect("Valid number b");
                    following
                        .entry(a)
                        .and_modify(|set: &mut HashSet<_>| _ = set.insert(b))
                        .or_insert_with(|| {
                            let set: HashSet<_> = [b].into();
                            set
                        });
                }
                None => {
                    break;
                }
            }
        }
        let res: u32 = input
            .lines()
            .skip(processed_line_count)
            .filter_map(|l| {
                let pages: Vec<u32> = l.split(',').flat_map(str::parse).collect();
                if pages.iter().enumerate().all(|(i, p)| {
                    let preceding: HashSet<_> = pages.iter().copied().take(i).collect();
                    match following.get(p) {
                        Some(following) => preceding.is_disjoint(following),
                        None => true,
                    }
                }) {
                    Some(pages[pages.len() / 2])
                } else {
                    None
                }
            })
            .sum();
        Ok(res.to_string())
    }

    #[tracing::instrument(fields(input = format!("{:?}[...]", input.lines().next())))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        todo!("b")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "143";
    const EXPECTED_B: &str = "todo_expected_b";

    #[test]
    #[traced_test]
    fn day_5_a() {
        let res = solution::part_a(TEST_INPUT);
        assert_eq!(EXPECTED_A, res.unwrap());
    }

    #[test]
    #[traced_test]
    fn day_5_b() {
        let res = solution::part_b(TEST_INPUT);
        assert_eq!(EXPECTED_B, res.unwrap());
    }
}
