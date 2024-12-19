pub mod solution {
    use std::collections::HashSet;

    use glam::UVec2;
    use grid::Grid;

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        Ok(solve_a(input, 71, 1024).to_string())
    }

    pub(crate) fn solve_a(input: &str, size: u8, byte_count: usize) -> usize {
        let size = u32::from(size);
        let obstacles: HashSet<_> = input
            .lines()
            .take(byte_count)
            .flat_map(parse::parse_uvec2_res)
            .collect();
        let grid = Grid::<()>::from_obstacles(obstacles, (size, size));
        let path = grid
            .find_path_astar((0, 0), ((size - 1), (size - 1)))
            .expect("Path exists");
        path.len() - 1
    }

    #[tracing::instrument(skip(input))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        let tile = solve_b(input, 71, 1024);
        Ok(format!("{},{}", tile.x, tile.y))
    }

    pub(crate) fn solve_b(input: &str, size: u8, safe_byte_count: usize) -> UVec2 {
        let size = u32::from(size);
        let walls: Vec<_> = input.lines().flat_map(parse::parse_uvec2_res).collect();
        let mut floor = safe_byte_count + 1;
        let mut ceil = walls.len() - 1;
        loop {
            let mid = (ceil - floor) / 2 + floor;
            let obstacles: HashSet<_> = walls.iter().take(mid).copied().collect();
            let grid = Grid::<()>::from_obstacles(obstacles, (size, size));
            let reachable = grid
                .find_path_astar((0, 0), ((size - 1), (size - 1)))
                .is_some();
            if reachable {
                floor = mid;
            } else {
                ceil = mid;
            }
            if ceil - floor <= 1 {
                return walls[if reachable { mid } else { mid - 1 }];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::UVec2;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "22";

    #[test]
    #[traced_test]
    fn day_18_a() {
        let res = solution::solve_a(TEST_INPUT, 7, 12);
        assert_eq!(EXPECTED_A, res.to_string());
    }

    #[test]
    #[traced_test]
    fn day_18_b() {
        let res = solution::solve_b(TEST_INPUT, 7, 12);
        assert_eq!(UVec2::new(6, 1), res);
    }
}
