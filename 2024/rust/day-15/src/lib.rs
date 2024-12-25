pub mod solution {
    use std::collections::HashSet;

    use glam::{IVec2, UVec2};
    use grid::Grid;

    struct Map {
        grid: Grid,
        crates: HashSet<UVec2>,
        robot_tile: UVec2,
    }
    impl Map {
        fn move_robot(&mut self, dir: IVec2) {
            let Some((target_tile, ())) = self.grid.move_target(self.robot_tile, dir) else {
                return;
            };
            if self.shift_crates(target_tile, dir) {
                self.robot_tile = target_tile;
            }
        }

        fn shift_crates(&mut self, tile: UVec2, dir: IVec2) -> bool {
            if !self.crates.contains(&tile) {
                // free tile - nothing to move
                return true;
            }
            let Some((target_tile, ())) = self.grid.move_target(tile, dir) else {
                return false;
            };
            if self.shift_crates(target_tile, dir) {
                self.crates.remove(&tile);
                self.crates.insert(target_tile);
                true
            } else {
                false
            }
        }
    }

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let mut lines = input.lines();
        let (Some(robot_tile), obstacles, crates, size) = lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .enumerate()
            .fold(
                (None, HashSet::new(), HashSet::new(), UVec2::ZERO),
                |(mut robot_tile, mut obstacles, mut crates, _), (y, l)| {
                    let mut size = UVec2::ZERO;
                    for (x, c) in l.chars().enumerate() {
                        let tile = UVec2::new(x as _, y as _);
                        size = tile;
                        match c {
                            '@' => robot_tile = Some(tile),
                            'O' => _ = crates.insert(tile),
                            '#' => _ = obstacles.insert(tile),
                            _ => {}
                        }
                    }
                    (robot_tile, obstacles, crates, size)
                },
            )
        else {
            panic!("Invalid map - robot not found")
        };
        let mut map = Map {
            grid: Grid::from_obstacles(obstacles, size),
            crates,
            robot_tile,
        };
        for l in lines {
            for dir_c in l.chars() {
                let dir = match dir_c {
                    '^' => IVec2::NEG_Y,
                    '>' => IVec2::X,
                    'v' => IVec2::Y,
                    '<' => IVec2::NEG_X,
                    _ => unreachable!(),
                };
                map.move_robot(dir);
            }
        }
        let score: u32 = map
            .crates
            .into_iter()
            .map(|tile| tile.y * 100 + tile.x)
            .sum();
        Ok(score.to_string())
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
    const EXPECTED_A: &str = "2028";
    const EXPECTED_B: &str = "todo_expected_b";

    #[test]
    #[traced_test]
    fn day_15_a() {
        let res = solution::part_a(TEST_INPUT);
        assert_eq!(EXPECTED_A, res.unwrap());
    }

    #[test]
    #[traced_test]
    fn day_15_b() {
        let res = solution::part_b(TEST_INPUT);
        assert_eq!(EXPECTED_B, res.unwrap());
    }
}
