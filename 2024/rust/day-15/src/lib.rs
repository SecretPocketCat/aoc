pub mod solution {
    use std::collections::{HashMap, HashSet};

    use glam::{IVec2, UVec2};
    use grid::{grid_iter, Grid};

    struct MapA {
        grid: Grid,
        crates: HashSet<UVec2>,
        robot_tile: UVec2,
    }
    impl MapA {
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
        let mut map = MapA {
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

    struct MapB {
        grid: Grid,
        pub(crate) crates: HashMap<UVec2, UVec2>,
        robot_tile: UVec2,
    }
    impl MapB {
        fn move_robot(&mut self, dir: IVec2) {
            let Some((target_tile, ())) = self.grid.move_target(self.robot_tile, dir) else {
                return;
            };
            let mut tiles_to_shift = Vec::new();
            if self.shift_crates(target_tile, dir, None, &mut tiles_to_shift) {
                self.robot_tile = target_tile;
                match dir {
                    IVec2::Y => {
                        tiles_to_shift.sort_unstable_by_key(|t| -(t.y as i32));
                    }
                    IVec2::NEG_Y => {
                        tiles_to_shift.sort_unstable_by_key(|t| (t.y as i32));
                    }
                    IVec2::X => {
                        tiles_to_shift.sort_unstable_by_key(|t| -(t.x as i32));
                    }
                    IVec2::NEG_X => {
                        tiles_to_shift.sort_unstable_by_key(|t| (t.x as i32));
                    }
                    _ => {}
                }
                for tile in tiles_to_shift {
                    let Some(other) = self.crates.get(&tile).copied() else {
                        continue;
                    };
                    self.crates.remove(&tile);
                    self.crates.remove(&other);
                    let (Some((target_tile, ())), Some((other_target_tile, ()))) = (
                        self.grid.move_target(tile, dir),
                        self.grid.move_target(other, dir),
                    ) else {
                        unreachable!();
                    };
                    self.crates.insert(target_tile, other_target_tile);
                    self.crates.insert(other_target_tile, target_tile);
                }
            }
        }

        fn shift_crates(
            &mut self,
            tile: UVec2,
            dir: IVec2,
            other_crate_tile: Option<UVec2>,
            crates_to_shift: &mut Vec<UVec2>,
        ) -> bool {
            let Some(other_tile) = self.crates.get(&tile).copied() else {
                // free tile - nothing to move
                return true;
            };
            let (Some((target_tile, ())), Some((other_target_tile, ()))) = (
                self.grid.move_target(tile, dir),
                self.grid.move_target(other_tile, dir),
            ) else {
                return false;
            };
            if let Some(prev) = other_crate_tile {
                // tile already processed
                if prev == target_tile {
                    return true;
                }
            }
            if self.shift_crates(target_tile, dir, Some(other_target_tile), crates_to_shift)
                && self.shift_crates(other_target_tile, dir, Some(target_tile), crates_to_shift)
            {
                crates_to_shift.push(tile);
                true
            } else {
                false
            }
        }

        #[allow(dead_code)]
        fn debug_map(&self) -> String {
            let size = self.grid.size();
            let mut dbg_map = String::with_capacity(size.element_product() as _);
            let x_axis = (0..size.x)
                .map(|i| (i % 10).to_string())
                .collect::<String>();
            dbg_map.push_str(&format!(" {}\n", &x_axis));
            dbg_map.push('0');
            let mut prev_y = 0;
            for tile in grid_iter(size) {
                if tile.y != prev_y {
                    prev_y = tile.y;
                    dbg_map.push_str(&(tile.y - 1).to_string());
                    dbg_map.push('\n');
                    dbg_map.push_str(&tile.y.to_string());
                }

                if self.robot_tile == tile {
                    dbg_map.push('@');
                } else if let Some(other_crate_tile) = self.crates.get(&tile) {
                    {}
                    dbg_map.push(if tile.x < other_crate_tile.x {
                        '['
                    } else {
                        ']'
                    });
                } else if self.grid.walkable_tiles().contains_key(&tile) {
                    dbg_map.push('.');
                } else {
                    dbg_map.push('#');
                }
            }
            dbg_map.push_str(&format!("\n {}", &x_axis));
            dbg_map
        }
    }

    #[tracing::instrument(skip(input))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        let mut lines = input.lines();
        let (Some(robot_tile), obstacles, crates, size) = lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .enumerate()
            .fold(
                (None, HashSet::new(), HashMap::new(), UVec2::ZERO),
                |(mut robot_tile, mut obstacles, mut crates, _), (y, l)| {
                    let mut size = UVec2::ZERO;
                    for (x, c) in l.chars().enumerate() {
                        let tile_l = UVec2::new(x as u32 * 2, y as _);
                        let tile_r = tile_l + UVec2::X;
                        size = tile_r + UVec2::ONE;
                        match c {
                            '@' => robot_tile = Some(tile_l),
                            'O' => {
                                crates.extend([(tile_l, tile_r), (tile_r, tile_l)]);
                            }
                            '#' => {
                                obstacles.extend([tile_l, tile_r]);
                            }
                            _ => {}
                        }
                    }
                    (robot_tile, obstacles, crates, size)
                },
            )
        else {
            panic!("Invalid map - robot not found")
        };
        let mut map = MapB {
            grid: Grid::from_obstacles(obstacles, size),
            crates,
            robot_tile,
        };
        // println!("Start:\n{}\n", map.debug_map());
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
                // println!("Move {dir_c}:\n{}\n", map.debug_map());
            }
        }
        // println!("Done:\n{}\n", map.debug_map());
        let score: u32 = map
            .crates
            .iter()
            .filter(|(tile, other_tile)| tile.x < other_tile.x)
            .map(|(tile, _)| tile.y * 100 + tile.x)
            .sum();
        Ok(score.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "10092";
    const EXPECTED_B: &str = "9021";

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

    #[test]
    #[traced_test]
    fn day_15_b_cone() {
        let res = solution::part_b(include_str!("../inputs/example_cone.txt"));
        assert_eq!("2339", res.unwrap());
    }
}
