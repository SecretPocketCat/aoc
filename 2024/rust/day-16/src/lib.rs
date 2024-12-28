pub mod solution {
    use std::collections::HashSet;

    use anyhow::Context;
    use grid::prelude::*;

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let BuiltGrid::<()> {
            grid,
            start_tile: Some(start),
            end_tile: Some(end),
        } = GridBuilder::build_obstacle_grid()
            .input(input)
            .obstacle('#')
            .start_character('S')
            .end_character('E')
            .call()?
        else {
            panic!("Invalid grid");
        };
        let (_, cost) = pathfinding::astar(
            &Neigbour::new(start, IVec2::X), // East
            |n| {
                let cw = (Neigbour::new(n.tile, -n.direction.perp()), 1000);
                let ccw = (Neigbour::new(n.tile, n.direction.perp()), 1000);
                match grid.move_target(n.tile, n.direction) {
                    Some((target_tile, ())) => {
                        vec![(Neigbour::new(target_tile, n.direction), 1), cw, ccw]
                    }
                    None => vec![cw, ccw],
                }
            },
            |n| n.tile.manhattan_distance(end),
            |n| n.tile == end,
        )
        .context("Found valid path")?;
        Ok(cost.to_string())
    }

    #[tracing::instrument(skip(input))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        let BuiltGrid::<()> {
            grid,
            start_tile: Some(start),
            end_tile: Some(end),
        } = GridBuilder::build_obstacle_grid()
            .input(input)
            .obstacle('#')
            .start_character('S')
            .end_character('E')
            .call()?
        else {
            panic!("Invalid grid");
        };
        let (paths, _cost) = pathfinding::astar_bag(
            &Neigbour::new(start, IVec2::X), // East
            |n| {
                let cw = (Neigbour::new(n.tile, -n.direction.perp()), 1000);
                let ccw = (Neigbour::new(n.tile, n.direction.perp()), 1000);
                match grid.move_target(n.tile, n.direction) {
                    Some((target_tile, ())) => {
                        vec![(Neigbour::new(target_tile, n.direction), 1), cw, ccw]
                    }
                    None => vec![cw, ccw],
                }
            },
            |n| n.tile.manhattan_distance(end),
            |n| n.tile == end,
        )
        .context("Found valid path")?;
        let all_tiles: HashSet<_> = paths
            .into_iter()
            .flat_map(|p| p.into_iter().map(|n| n.tile))
            .collect();
        Ok(all_tiles.len().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "7036";
    const EXPECTED_B: &str = "45";

    #[test]
    #[traced_test]
    fn day_16_a() {
        let res = solution::part_a(TEST_INPUT);
        assert_eq!(EXPECTED_A, res.unwrap());
    }

    #[test]
    #[traced_test]
    fn day_16_b() {
        let res = solution::part_b(TEST_INPUT);
        assert_eq!(EXPECTED_B, res.unwrap());
    }
}
