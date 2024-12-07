pub mod solution {
    use std::collections::{HashMap, HashSet};

    use anyhow::Context;
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
        fn rotate_cw(self) -> Self {
            match self {
                Dir::North => Dir::East,
                Dir::East => Dir::South,
                Dir::South => Dir::West,
                Dir::West => Dir::North,
            }
        }
    }

    #[derive(Clone)]
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

    #[tracing::instrument(fields(input = format!("{:?}[...]", input.lines().next())))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let mut map = parse_map(input);
        let visited = walk_map(&mut map).context("Found a cycle")?;
        Ok(visited.len().to_string())
    }

    #[tracing::instrument(skip(input))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        let mut map = parse_map(input);
        let cycle_guard = map.guard.clone();
        let visited = walk_map(&mut map).context("Found a cycle")?;
        let mut cycle_count = 0;
        for c in visited {
            map.guard = cycle_guard.clone();
            map.walls.insert(c);
            if walk_map(&mut map).is_none() {
                cycle_count += 1;
            }
            map.walls.remove(&c);
        }

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

    #[must_use]
    fn walk_map(map: &mut Map) -> Option<HashSet<IVec2>> {
        let mut visited: HashSet<_> = [map.guard.coords].into();
        let mut wall_hits: HashMap<_, HashSet<_>> = HashMap::new();
        loop {
            let new_coords = map.guard.move_coords();
            if !map.contains_coords(new_coords) {
                // out of bounds
                break;
            } else if map.walls.contains(&new_coords) {
                // hit wall
                let mut cycle = false;
                wall_hits
                    .entry(new_coords)
                    .and_modify(|dirs| {
                        if !dirs.insert(map.guard.dir) {
                            cycle = true;
                        }
                    })
                    .or_insert_with(|| [map.guard.dir].into());
                if cycle {
                    return None;
                }
                map.guard.rotate_cw();
            } else {
                // moved into coords
                visited.insert(new_coords);
                map.guard.coords = new_coords;
            }
        }
        Some(visited)
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
