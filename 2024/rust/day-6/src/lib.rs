pub mod solution {
    use std::collections::HashSet;

    use glam::{IVec2, UVec2};
    use tracing::{debug, warn};

    #[derive(Debug, Clone, Copy)]
    enum Dir {
        North,
        East,
        South,
        West,
    }

    #[tracing::instrument(fields(input = format!("{:?}[...]", input.lines().next())))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
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
        let map_size = UVec2::new(rows.first().unwrap().chars().count() as _, rows.len() as _);
        let mut visited: HashSet<_> = [guard_coords].into();
        loop {
            // move
            let new_coords = guard_coords
                + match guard_dir {
                    Dir::North => IVec2::NEG_Y,
                    Dir::East => IVec2::X,
                    Dir::South => IVec2::Y,
                    Dir::West => IVec2::NEG_X,
                };
            if new_coords.min_element() < 0
                || new_coords.x > (map_size.x - 1) as _
                || new_coords.y > (map_size.y - 1) as _
            {
                // out of bounds
                debug!(?new_coords, "Out of bounds");
                break;
            } else if walls.contains(&new_coords) {
                // wall - rotate clockwise
                guard_dir = match guard_dir {
                    Dir::North => Dir::East,
                    Dir::East => Dir::South,
                    Dir::South => Dir::West,
                    Dir::West => Dir::North,
                };
                debug!(?new_coords, ?guard_dir, "Rotate");
            } else {
                debug!(?new_coords, "Visit");
                visited.insert(new_coords);
                guard_coords = new_coords;
            }
        }
        // todo: mark the hit walls
        // then look for loops by looking for squares that can be created by using the hit walls
        Ok(visited.len().to_string())
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
