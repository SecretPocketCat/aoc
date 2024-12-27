pub mod solution {
    use std::collections::{HashMap, HashSet};

    use glam::{IVec2, UVec2};
    use grid::prelude::*;
    use itertools::Itertools;
    use tracing::warn;

    struct NeighbourCount(usize);

    type VisitedMap = HashMap<UVec2, NeighbourCount>;

    #[derive(Debug)]
    struct Map(HashMap<UVec2, char>);
    impl Map {
        fn new(input: &str) -> Self {
            Self(
                input
                    .lines()
                    .enumerate()
                    .flat_map(|(y, l)| {
                        l.chars()
                            .enumerate()
                            .map(move |(x, c)| (UVec2::new(x as _, y as _), c))
                    })
                    .collect(),
            )
        }

        fn move_by(&self, pos: UVec2, dir: IVec2) -> Option<(UVec2, char)> {
            let target = pos.as_ivec2() + dir;
            if target.min_element() < 0 {
                return None;
            }
            let target = target.as_uvec2();
            self.0.get(&target).map(|c| (target, *c))
        }

        fn drain_area(&mut self, pos: UVec2) -> VisitedMap {
            let val = self.0[&pos];
            let mut visited = HashMap::new();
            let mut q = Vec::with_capacity(4);
            q.push(pos);
            while let Some(pos) = q.pop() {
                if visited.contains_key(&pos) {
                    continue;
                }
                let mut neighbours: Vec<_> = DIRS_4
                    .iter()
                    .filter_map(|dir| match self.move_by(pos, *dir) {
                        Some((target, c)) if c == val => Some(target),
                        _ => None,
                    })
                    .collect();
                visited.insert(pos, NeighbourCount(neighbours.len()));
                q.append(&mut neighbours);
            }
            for pos in visited.keys() {
                self.0.remove(pos);
            }
            visited
        }
    }

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let mut map = Map::new(input);
        let mut price = 0;
        while let Some(pos) = map.0.keys().next() {
            let visited = map.drain_area(*pos);
            let area = visited.len();
            let region: usize = visited
                .values()
                .map(|neighbour_count| 4 - neighbour_count.0)
                .sum();
            price += area * region;
        }
        Ok(price.to_string())
    }

    #[tracing::instrument(skip(input))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        let mut map = Map::new(input);
        let mut price = 0;
        while let Some(pos) = map.0.keys().next() {
            let visited = map.drain_area(*pos);
            let visited: HashSet<_> = visited.keys().map(UVec2::as_ivec2).collect();
            let area = visited.len();
            let edges: usize = visited
                .iter()
                // sort the tiles from top-left to btm-right
                .sorted_unstable_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)))
                .map(|tile| {
                    DIRS_4
                        .iter()
                        .filter(|dir| {
                            let target = tile + *dir;
                            match visited.get(&target) {
                                // region tile in that direction => no edge
                                Some(_) => false,
                                _ => match **dir {
                                    IVec2::NEG_Y => {
                                        !visited.contains(&(tile + IVec2::NEG_X))
                                            || visited.contains(&(tile + IVec2::NEG_ONE))
                                    }
                                    IVec2::Y => {
                                        !visited.contains(&(tile + IVec2::NEG_X))
                                            || visited.contains(&(tile + IVec2::new(-1, 1)))
                                    }
                                    IVec2::NEG_X => {
                                        !visited.contains(&(tile + IVec2::NEG_Y))
                                            || visited.contains(&(tile + IVec2::NEG_ONE))
                                    }
                                    IVec2::X => {
                                        !visited.contains(&(tile + IVec2::NEG_Y))
                                            || visited.contains(&(tile + IVec2::new(1, -1)))
                                    }
                                    _ => unreachable!(),
                                },
                            }
                        })
                        .count()
                })
                .sum();
            price += area * edges;
        }
        Ok(price.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "140";
    const EXPECTED_B: &str = "80";

    #[test]
    #[traced_test]
    fn day_12_a() {
        let res = solution::part_a(TEST_INPUT);
        assert_eq!(EXPECTED_A, res.unwrap());
    }

    #[test]
    #[traced_test]
    fn day_12_b() {
        let res = solution::part_b(TEST_INPUT);
        assert_eq!(EXPECTED_B, res.unwrap());
    }

    #[test]
    #[traced_test]
    #[allow(non_snake_case)]
    fn day_12_b_E_map() {
        let res = solution::part_b(include_str!("../inputs/example_E_map.txt"));
        assert_eq!("236", res.unwrap());
    }
}
