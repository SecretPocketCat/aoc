pub mod solution {
    use std::collections::HashSet;

    use glam::UVec2;
    use grid_pathfinding::PathingGrid;
    use grid_util::{Grid, Point};

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        Ok(solve_a(input, 71, 1024).to_string())
    }

    pub(crate) fn solve_a(input: &str, size: u8, byte_count: usize) -> usize {
        let size = usize::from(size);
        let mut grid = PathingGrid::new(size, size, false);
        grid.allow_diagonal_move = false;
        let walls: HashSet<_> = input
            .lines()
            .take(byte_count)
            .flat_map(parse::parse_uvec2_res)
            .collect();
        let mut grid_str = String::new();
        for y in 0..size {
            for x in 0..size {
                let pos = UVec2::new(x as _, y as _);
                grid_str.push(if walls.contains(&pos) { '#' } else { '.' });
            }
            grid_str.push('\n');
        }

        // print!("{grid_str}");

        // todo: set obstacles
        for wall in walls {
            grid.set(wall.x as _, wall.y as _, true);
        }
        grid.generate_components();
        let path = grid
            .get_path_single_goal(
                Point::new(0, 0),
                Point::new((size - 1) as _, (size - 1) as _),
                false,
            )
            .expect("Path exists");
        path.len() - 1
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
    const EXPECTED_A: &str = "22";
    const EXPECTED_B: &str = "todo_expected_b";

    #[test]
    #[traced_test]
    fn day_18_a() {
        let res = solution::solve_a(TEST_INPUT, 7, 12);
        assert_eq!(EXPECTED_A, res.to_string());
    }

    #[test]
    #[traced_test]
    fn day_18_b() {
        let res = solution::part_b(TEST_INPUT);
        assert_eq!(EXPECTED_B, res.unwrap());
    }
}
