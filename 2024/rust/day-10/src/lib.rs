pub mod solution {
    use std::collections::{HashMap, HashSet};

    use glam::{IVec2, UVec2};
    use tracing::warn;

    const DIRS: [IVec2; 4] = [IVec2::NEG_Y, IVec2::X, IVec2::Y, IVec2::NEG_X];

    #[derive(Debug)]
    struct Map(HashMap<UVec2, u32>);
    impl Map {
        fn move_by(&self, pos: UVec2, dir: IVec2) -> Option<(UVec2, u32)> {
            let target = pos.as_ivec2() + dir;
            if target.min_element() < 0 {
                return None;
            }
            let target = target.as_uvec2();
            self.0.get(&target).map(|height| (target, *height))
        }

        fn walk(
            &self,
            pos: UVec2,
            curr_height: u32,
            mut reached: HashSet<UVec2>,
        ) -> HashSet<UVec2> {
            for d in &DIRS {
                match self.move_by(pos, *d) {
                    Some((target, 9)) if curr_height == 8 => _ = reached.insert(target),
                    Some((target, height)) if height == curr_height + 1 => {
                        reached = self.walk(target, height, reached);
                    }
                    _ => {}
                };
            }
            reached
        }

        fn sum_paths(&self, pos: UVec2, curr_height: u32) -> usize {
            DIRS.iter()
                .filter_map(|d| match self.move_by(pos, *d) {
                    Some((_, 9)) if curr_height == 8 => Some(1),
                    Some((target, height)) if height == curr_height + 1 => {
                        Some(self.sum_paths(target, height))
                    }
                    _ => None,
                })
                .sum()
        }
    }

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let map = parse_map(input);
        let reachable_tails_count: usize = map
            .0
            .iter()
            .filter(|(_, v)| **v == 0)
            .map(|(pos, _)| map.walk(*pos, 0, HashSet::new()).len())
            .sum();
        Ok(reachable_tails_count.to_string())
    }

    #[tracing::instrument(skip(input))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        let map = parse_map(input);
        let trails_score: usize = map
            .0
            .iter()
            .filter(|(_, v)| **v == 0)
            .map(|(pos, _)| map.sum_paths(*pos, 0))
            .sum();
        Ok(trails_score.to_string())
    }

    fn parse_map(input: &str) -> Map {
        Map(input
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars().enumerate().map(move |(x, c)| {
                    (
                        UVec2::new(x as _, y as _),
                        c.to_digit(10).expect("Valid int"),
                    )
                })
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    const TEST_INPUT_SM: &str = include_str!("../inputs/example_sm.txt");
    const TEST_INPUT: &str = include_str!("../inputs/example.txt");

    #[test]
    #[traced_test]
    fn day_10_a_sm() {
        let res = solution::part_a(TEST_INPUT_SM);
        assert_eq!("1", res.unwrap());
    }

    #[test]
    #[traced_test]
    fn day_10_a() {
        let res = solution::part_a(TEST_INPUT);
        assert_eq!("36", res.unwrap());
    }

    #[test]
    #[traced_test]
    fn day_10_b() {
        let res = solution::part_b(TEST_INPUT);
        assert_eq!("81", res.unwrap());
    }
}
