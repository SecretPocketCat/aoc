pub mod solution {
    use std::{
        cmp::Ordering,
        collections::{HashMap, HashSet},
    };

    type RulePair = (u32, u32);

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

    #[tracing::instrument(skip(input))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        let (processed_line_count, rules) = parse_rules(input);

        let res: u32 = input
            .lines()
            .skip(processed_line_count)
            .filter_map(|l| {
                let pages: Vec<u32> = l.split(',').flat_map(str::parse).collect();
                eval_update(&pages, &rules)
            })
            .sum();
        Ok(res.to_string())
    }

    #[tracing::instrument(skip(input))]
    pub fn parse_rules(input: &str) -> (usize, Vec<RulePair>) {
        let mut rules = Vec::new();
        for line in input.lines().skip_while(|l| l.is_empty()) {
            match line.split_once('|') {
                Some((a, b)) => {
                    let a: u32 = a.parse().expect("Valid number a");
                    let b: u32 = b.parse().expect("Valid number b");
                    rules.push((a, b));
                }
                None => {
                    break;
                }
            }
        }
        (rules.len() + 1, rules)
    }

    fn eval_update(pages: &[u32], all_rules: &[RulePair]) -> Option<u32> {
        let page_set: HashSet<_> = pages.iter().copied().collect();
        let rule_pairs: Vec<_> = all_rules
            .iter()
            .filter(|(a, b)| page_set.contains(a) || page_set.contains(b))
            .collect();
        let mut rule_map: HashMap<u32, HashSet<RulePair>> = HashMap::new();
        for rule in rule_pairs {
            rule_map
                .entry(rule.0)
                .and_modify(|r| {
                    r.insert(*rule);
                })
                .or_insert_with(|| [*rule].into());
            rule_map
                .entry(rule.1)
                .and_modify(|r| {
                    r.insert(*rule);
                })
                .or_insert_with(|| [*rule].into());
        }
        let mut sorted_pages = pages.to_vec();
        sorted_pages.sort_unstable_by(|a, b| {
            let rules = &rule_map[a];
            let rule = rules
                .iter()
                .find(|rule| rule.0 == *b || rule.1 == *b)
                // todo: this probly won't work
                .expect("Matching rule");
            if rule.0 == *a {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        if sorted_pages.iter().enumerate().all(|(i, p)| *p == pages[i]) {
            None
        } else {
            Some(sorted_pages[sorted_pages.len() / 2])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "143";
    const EXPECTED_B: &str = "123";

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
