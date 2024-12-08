pub mod solution {
    use std::collections::{HashMap, HashSet};

    use glam::{IVec2, UVec2};
    use itertools::Itertools;
    use tracing::warn;

    struct Map {
        antennas: HashMap<char, Vec<UVec2>>,
        size: UVec2,
    }
    impl Map {
        fn contains_ivec2_coords(&self, coords: IVec2) -> bool {
            coords.min_element() >= 0
                && coords.x <= (self.size.x - 1) as _
                && coords.y <= (self.size.y - 1) as _
        }

        fn parse(input: &str) -> Self {
            let rows: Vec<_> = input.lines().collect();
            let antennas = rows.iter().enumerate().fold(
                HashMap::new(),
                |mut map: HashMap<_, Vec<_>>, (y, row)| {
                    for (x, c) in row.chars().enumerate().filter(|(_, c)| *c != '.') {
                        let coord = UVec2::new(x as _, y as _);
                        map.entry(c)
                            .and_modify(|c| c.push(coord))
                            .or_insert_with(|| vec![coord]);
                    }
                    map
                },
            );
            Self {
                antennas,
                size: UVec2::new(rows.first().unwrap().chars().count() as _, rows.len() as _),
            }
        }
    }

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let map = Map::parse(input);
        let antinodes: HashSet<_> = map
            .antennas
            .values()
            .filter(|c| c.len() > 1)
            .flat_map(|coords| {
                coords.iter().combinations(2).flat_map(|coords| {
                    let a1 = coords[0];
                    let a2 = coords[1];
                    let (a1, a2) = if a1.y < a2.y || a1.x < a2.x {
                        (a1, a2)
                    } else {
                        (a2, a1)
                    };
                    let delta = a2.as_ivec2() - a1.as_ivec2();
                    let antinodes = [a1.as_ivec2() - delta, a2.as_ivec2() + delta];
                    antinodes
                        .into_iter()
                        .filter(|an| map.contains_ivec2_coords(*an))
                        .map(|an| an.as_uvec2())
                })
            })
            .collect();
        Ok(antinodes.len().to_string())
    }

    #[tracing::instrument(skip(input))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        let map = Map::parse(input);
        #[allow(clippy::cast_precision_loss)]
        let vec_capacity = ((map.size.x.pow(2) + map.size.y.pow(2)) as f32)
            .sqrt()
            .ceil() as usize;
        let antinodes: HashSet<_> = map
            .antennas
            .values()
            .filter(|c| c.len() > 1)
            .flat_map(|coords| {
                coords.iter().combinations(2).flat_map(|coords| {
                    let a1 = coords[0];
                    let a2 = coords[1];
                    let (a1, a2) = if a1.y < a2.y || a1.x < a2.x {
                        (a1, a2)
                    } else {
                        (a2, a1)
                    };
                    let delta = a2.as_ivec2() - a1.as_ivec2();
                    let mut antinodes = Vec::with_capacity(vec_capacity);
                    // up from earlier/higher antinode
                    let mut an = a1.as_ivec2();
                    loop {
                        if map.contains_ivec2_coords(an) {
                            antinodes.push(an);
                        } else {
                            break;
                        }
                        an -= delta;
                    }
                    // up from earlier/higher antinode
                    let mut an = a2.as_ivec2();
                    loop {
                        if map.contains_ivec2_coords(an) {
                            antinodes.push(an);
                        } else {
                            break;
                        }
                        an += delta;
                    }
                    antinodes.into_iter().map(|an| an.as_uvec2())
                })
            })
            .collect();
        Ok(antinodes.len().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "14";
    const EXPECTED_B: &str = "34";

    #[test]
    #[traced_test]
    fn day_8_a() {
        let res = solution::part_a(TEST_INPUT);
        assert_eq!(EXPECTED_A, res.unwrap());
    }

    #[test]
    #[traced_test]
    fn day_8_b() {
        let res = solution::part_b(TEST_INPUT);
        assert_eq!(EXPECTED_B, res.unwrap());
    }
}
