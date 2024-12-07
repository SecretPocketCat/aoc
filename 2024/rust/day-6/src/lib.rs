pub mod solution {
    use std::collections::{HashMap, HashSet};

    use glam::{IVec2, UVec2};
    use tracing::warn;

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
    enum Dir {
        North,
        East,
        South,
        West,
    }
    impl Dir {
        #[must_use]
        fn rotate_cw(&self) -> Self {
            match self {
                Dir::North => Dir::East,
                Dir::East => Dir::South,
                Dir::South => Dir::West,
                Dir::West => Dir::North,
            }
        }
    }

    struct Guard {
        coords: IVec2,
        dir: Dir,
    }
    impl Guard {
        fn move_coords(&self) -> IVec2 {
            self.coords
                + match self.dir {
                    Dir::North => IVec2::NEG_Y,
                    Dir::East => IVec2::X,
                    Dir::South => IVec2::Y,
                    Dir::West => IVec2::NEG_X,
                }
        }

        fn rotate_cw(&mut self) {
            self.dir = self.dir.rotate_cw();
        }
    }

    struct Map {
        guard: Guard,
        walls: HashSet<IVec2>,
        size: UVec2,
    }
    impl Map {
        fn contains_coords(&self, coords: IVec2) -> bool {
            coords.min_element() >= 0
                && coords.x <= (self.size.x - 1) as _
                && coords.y <= (self.size.y - 1) as _
        }
    }

    struct WalkMapResult {
        visited: HashSet<IVec2>,
        wall_hits: HashMap<IVec2, HashSet<Dir>>,
    }
    impl WalkMapResult {
        fn new(guard: &Guard) -> Self {
            let visited: HashSet<_> = [guard.coords].into();
            Self {
                visited,
                wall_hits: Default::default(),
            }
        }
    }

    #[tracing::instrument(fields(input = format!("{:?}[...]", input.lines().next())))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let mut map = parse_map(input);
        let res = walk_map(&mut map);
        Ok(res.visited.len().to_string())
    }

    #[tracing::instrument(skip(input))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        let mut map = parse_map(input);
        let walk_res = walk_map(&mut map);
        let cycle_count: usize = walk_res
            .wall_hits
            .iter()
            .map(|(coords, hit_dirs)| {
                hit_dirs
                    .iter()
                    .filter(|dir| {
                        let mut cycle_guard = Guard {
                            coords: *coords,
                            dir: **dir,
                        };
                        let mut cycle = vec![*coords];
                        for _ in 0..2 {
                            let possible_coords: Vec<_> = match cycle_guard.dir {
                                Dir::North => ((cycle_guard.coords.x + 1)..map.size.x as _)
                                    .map(|x| IVec2::new(x, cycle_guard.coords.y + 1))
                                    .collect(),
                                Dir::East => ((cycle_guard.coords.y + 1)..map.size.y as _)
                                    .map(|y| IVec2::new(cycle_guard.coords.x - 1, y))
                                    .collect(),
                                Dir::South => (0..(cycle_guard.coords.x - 1))
                                    .rev()
                                    .map(|x| IVec2::new(x, cycle_guard.coords.y - 1))
                                    .collect(),
                                Dir::West => (0..(cycle_guard.coords.y - 1))
                                    .rev()
                                    .map(|y| IVec2::new(cycle_guard.coords.x + 1, y))
                                    .collect(),
                            };

                            if let Some(hit) =
                                possible_coords.into_iter().find_map(|c| map.walls.get(&c))
                            {
                                cycle_guard.coords = *hit;
                                cycle_guard.rotate_cw();
                                cycle.push(*hit);
                            } else {
                                return false;
                            }
                        }
                        // found 3 corners of a cycle
                        // check the 4th one isn't block by an existing wall
                        let empty_coords: Vec<_> = match cycle_guard.dir {
                            Dir::North => ((cycle_guard.coords.x + 1)..(coords.x - 1) as _)
                                .map(|x| IVec2::new(x, cycle_guard.coords.y + 1))
                                .collect(),
                            Dir::East => ((cycle_guard.coords.y + 1)..(coords.y - 1) as _)
                                .map(|y| IVec2::new(cycle_guard.coords.x - 1, y))
                                .collect(),
                            Dir::South => ((coords.x + 1)..(cycle_guard.coords.x - 1))
                                .rev()
                                .map(|x| IVec2::new(x, cycle_guard.coords.y - 1))
                                .collect(),
                            Dir::West => ((coords.y + 1)..(cycle_guard.coords.y - 1))
                                .rev()
                                .map(|y| IVec2::new(cycle_guard.coords.x + 1, y))
                                .collect(),
                        };
                        if empty_coords.into_iter().any(|c| map.walls.contains(&c)) {
                            false
                        } else {
                            // warn!(?dir, ?cycle, "found a cycle\n");
                            true
                        }
                    })
                    .count()
            })
            .sum();

        Ok(cycle_count.to_string())
    }

    fn parse_map(input: &str) -> Map {
        let rows: Vec<_> = input.lines().collect();
        let mut guard_coords = IVec2::default();
        let mut guard_dir = Dir::North;
        let mut walls = HashSet::new();
        for (y, line) in rows.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let coords = IVec2::new(x as _, y as _);
                match c {
                    '#' => {
                        walls.insert(coords);
                    }
                    '^' => {
                        guard_coords = coords;
                        guard_dir = Dir::North;
                    }
                    '>' => {
                        guard_coords = coords;
                        guard_dir = Dir::East;
                    }
                    'v' => {
                        guard_coords = coords;
                        guard_dir = Dir::South;
                    }
                    '<' => {
                        guard_coords = coords;
                        guard_dir = Dir::West;
                    }
                    _ => {}
                }
            }
        }
        Map {
            guard: Guard {
                coords: guard_coords,
                dir: guard_dir,
            },
            walls,
            size: UVec2::new(rows.first().unwrap().chars().count() as _, rows.len() as _),
        }
    }

    fn walk_map(map: &mut Map) -> WalkMapResult {
        let mut result = WalkMapResult::new(&map.guard);
        loop {
            let new_coords = map.guard.move_coords();
            if !map.contains_coords(new_coords) {
                // out of bounds
                break;
            } else if map.walls.contains(&new_coords) {
                // hit wall
                result
                    .wall_hits
                    .entry(new_coords)
                    .and_modify(|dirs| {
                        dirs.insert(map.guard.dir);
                    })
                    .or_insert_with(|| [map.guard.dir].into());
                map.guard.rotate_cw();
            } else {
                // moved into coords
                result.visited.insert(new_coords);
                map.guard.coords = new_coords;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "41";
    const EXPECTED_B: &str = "6";

    #[test]
    #[traced_test]
    fn day_6_a() {
        let res = solution::part_a(TEST_INPUT);
        assert_eq!(EXPECTED_A, res.unwrap());
    }

    #[test]
    #[traced_test]
    fn day_6_b() {
        let res = solution::part_b(TEST_INPUT);
        assert_eq!(EXPECTED_B, res.unwrap());
    }
}
