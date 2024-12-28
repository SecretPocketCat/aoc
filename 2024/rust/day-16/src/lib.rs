pub mod solution {
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
        let path = Grid::<()>::find_path_astar_with_successors(
            Neigbour::new(start, IVec2::X), // East
            &Neigbour::new(end, IVec2::ZERO),
            |n| {
                grid.neighbours(n.tile)
                    .into_iter()
                    .flat_map(|neighbour| {
                        DIRS_4
                            .iter()
                            .filter(|dir| grid.move_target(n.tile, **dir).is_some())
                            .map(move |direction| {
                                let cost = if direction.abs() == n.direction.abs() {
                                    // moving in the same dir or backwards
                                    1
                                } else {
                                    // rotating to the side
                                    1001
                                };
                                (Neigbour::new(neighbour.tile, *direction), cost)
                            })
                    })
                    .collect::<Vec<_>>()
            },
            |n| n.tile.manhattan_distance(end),
            |n| n.tile == end,
        )
        .context("Found valid path")?;
        Ok(path.cost.to_string())
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
    const EXPECTED_A: &str = "7036";
    const EXPECTED_B: &str = "todo_expected_b";

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
