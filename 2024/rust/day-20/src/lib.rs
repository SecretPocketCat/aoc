pub mod solution {
    use core::panic;
    use std::collections::{HashMap, HashSet};

    use glam::{IVec2, UVec2};
    use grid::{Grid, Neigbour, UVec2Ext, DIRS_4};

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
        let grid = Grid::<()>::from_obstacles(walls, size);
        // perf: instead of pathfinding, could just walk available neighbours because there's only 1 valid path
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
        let valid_cheat_count = solve_b(input, 20, 100);
        Ok(valid_cheat_count.to_string())
    }

    pub(crate) fn solve_b(input: &str, cheat_max_len: u32, cheat_shortcut_treshold: u32) -> usize {
        let path = find_path(input);
        let orig_path_len = path.len();
        path.iter()
            .enumerate()
            .map_while(|(start_i, cheat_start)| {
                // sub min cheat len, which has to be at least 2
                let from = start_i + cheat_shortcut_treshold as usize + 2;
                if from < path.len() {
                    Some(
                        path.iter()
                            .enumerate()
                            .skip(from)
                            .filter(|(end_i, cheat_end)| {
                                let cheat_len =
                                    cheat_start.manhattan_distance(**cheat_end) as usize;
                                if cheat_len > cheat_max_len as _ {
                                    return false;
                                }
                                let cheat_path_len = start_i + cheat_len + orig_path_len - end_i;
                                let saved = (orig_path_len - cheat_path_len) as u32;
                                saved >= cheat_shortcut_treshold
                            })
                            .count(),
                    )
                } else {
                    None
                }
            })
            .sum()
    }

    fn find_path(input: &str) -> Vec<UVec2> {
        let lines: Vec<_> = input.lines().collect();
        let size = UVec2::new(lines[0].chars().count() as _, lines.len() as _);
        let (Some(start), Some(end), walkable_tiles) = lines.into_iter().enumerate().fold(
            (None, None, Vec::with_capacity((size.x * size.y) as _)),
            |(mut start, mut end, mut walls), (y, l)| {
                for (x, c) in l.chars().enumerate() {
                    let tile = UVec2::new(x as _, y as _);
                    if let Some(tile) = match c {
                        '.' => Some(tile),
                        'S' => {
                            start = Some(tile);
                            Some(tile)
                        }
                        'E' => {
                            end = Some(tile);
                            Some(tile)
                        }
                        _ => None,
                    } {
                        walls.push((tile, ()));
                    }
                }
                (start, end, walls)
            },
        ) else {
            panic!("Invalid map - no start or end found");
        };
        let mut path = Vec::with_capacity(walkable_tiles.len());
        let grid = Grid::<()>::from_walkable_tiles(walkable_tiles.into_iter(), size);
        // walk single possible path
        let mut tile = start;
        let mut prev_dir = IVec2::ZERO;
        path.push(start);
        while tile != end {
            let neigbour = DIRS_4
                .iter()
                .filter(|d| -*d != prev_dir)
                .find_map(|d| {
                    grid.move_target(tile, *d)
                        .map(|(n, ())| Neigbour::new(n, *d))
                })
                .expect("Found next path tile");
            path.push(neigbour.tile);
            tile = neigbour.tile;
            prev_dir = neigbour.direction;
        }
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");

    #[test_case(1 => 44)]
    #[test_case(10 => 10)]
    #[test_case(64 => 1)]
    #[traced_test]
    fn day_20_a(treshold: usize) -> usize {
        solution::solve_a(TEST_INPUT, treshold)
    }

    // #[test_case(2, 2 => 44)]
    // #[test_case(2, 10 => 10)]
    // #[test_case(2, 64 => 1)]
    #[test_case(20, 70 => 41)]
    #[test_case(20, 76 => 3)]
    // todo: part b test cases
    #[traced_test]
    fn day_20_solve(cheat_max_len: u32, treshold: u32) -> usize {
        solution::solve_b(TEST_INPUT, cheat_max_len, treshold)
    }
}
