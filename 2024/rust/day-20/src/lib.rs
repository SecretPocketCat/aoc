pub mod solution {
    use core::panic;
    use std::collections::{HashMap, HashSet};

    use glam::UVec2;
    use grid::Grid;

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let count = solve_a(input, 100);
        Ok(count.to_string())
    }

    pub(crate) fn solve_a(input: &str, treshold: usize) -> usize {
        let lines: Vec<_> = input.lines().collect();
        let size = UVec2::new(lines[0].chars().count() as _, lines.len() as _);
        let (Some(start), Some(end), walls) = lines.into_iter().enumerate().fold(
            (None, None, HashSet::with_capacity((size.x * size.y) as _)),
            |(mut start, mut end, mut walls), (y, l)| {
                for (x, c) in l.chars().enumerate() {
                    let tile = UVec2::new(x as _, y as _);
                    match c {
                        '#' => {
                            walls.insert(tile);
                        }
                        'S' => {
                            start = Some(tile);
                        }
                        'E' => {
                            end = Some(tile);
                        }
                        _ => {}
                    }
                }
                (start, end, walls)
            },
        ) else {
            panic!("Invalid map - no start or end found");
        };
        tracing::warn!(?start, ?end, "collected walls");
        let grid = Grid::<()>::from_obstacles(walls, size);
        let path = grid.find_path_astar(start, end).expect("Found path");
        let path_index_map: HashMap<_, _> = path.iter().enumerate().map(|(i, t)| (*t, i)).collect();
        path.into_iter()
            .map(|tile| {
                let start_i = path_index_map[&tile];
                grid.obstacle_neighbours(tile)
                    .into_iter()
                    .filter_map(|wall| {
                        match grid
                            .move_target(wall.tile, wall.direction)
                            .and_then(|(cheat_end, ())| path_index_map.get(&cheat_end))
                            .copied()
                        {
                            Some(end_i) => {
                                // remove 2 tiles that the cheater has to move
                                let distance = end_i.saturating_sub(start_i + 2);
                                if distance >= treshold {
                                    Some(distance)
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        }
                    })
                    .count()
            })
            .sum()
    }

    #[tracing::instrument(skip(input))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        todo!("b")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_B: &str = "todo_expected_a";

    #[test_case(1 => 44)]
    #[test_case(10 => 10)]
    #[test_case(64 => 1)]
    #[traced_test]
    fn day_20_a(treshold: usize) -> usize {
        solution::solve_a(TEST_INPUT, treshold)
    }

    #[test]
    #[traced_test]
    fn day_20_b() {
        let res = solution::part_b(TEST_INPUT);
        assert_eq!(EXPECTED_B, res.unwrap());
    }
}
