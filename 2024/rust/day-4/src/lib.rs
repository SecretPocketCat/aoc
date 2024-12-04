pub mod solution {
    #[tracing::instrument(fields(input = format!("{:?}[...]", input.lines().next())))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let char_grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let dirs = [
            [(0isize, -1isize), (0, -2), (0, -3)], // North
            [(1, -1), (2, -2), (3, -3)],           // NE
            [(1, 0), (2, 0), (3, 0)],              // East
            [(1, 1), (2, 2), (3, 3)],              // SE
            [(0, 1), (0, 2), (0, 3)],              // South
            [(-1, 1), (-2, 2), (-3, 3)],           // SW
            [(-1, 0), (-2, 0), (-3, 0)],           // West
            [(-1, -1), (-2, -2), (-3, -3)],        // NW
        ];
        let needle_chars: Vec<char> = "MAS".chars().collect();
        let count: usize = char_grid
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, c)| match c {
                        'X' => dirs
                            .iter()
                            .filter(|dir| {
                                dir.iter().enumerate().all(|(char_i, coord)| {
                                    let x = x as isize + coord.0;
                                    let y = y as isize + coord.1;
                                    if x < 0 || y < 0 {
                                        return false;
                                    }
                                    match char_grid
                                        .get(y as usize)
                                        .and_then(|row| row.get(x as usize))
                                    {
                                        Some(dir_c) => *dir_c == needle_chars[char_i],
                                        None => false,
                                    }
                                })
                            })
                            .count(),
                        _ => 0,
                    })
                    .sum::<usize>()
            })
            .sum();
        Ok(count.to_string())
    }

    #[tracing::instrument(fields(input = format!("{:?}[...]", input.lines().next())))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        let char_grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let count: usize = char_grid
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(x, c)| match c {
                        // look for 'A' which is always in the center
                        'A' => {
                            let Some(x_left) = x.checked_add_signed(-1) else {
                                return false;
                            };
                            let Some(y_top) = y.checked_add_signed(-1) else {
                                return false;
                            };
                            let Some(tl) = char_grid.get(y_top).and_then(|row| row.get(x_left))
                            else {
                                return false;
                            };
                            let Some(tr) = char_grid.get(y_top).and_then(|row| row.get(x + 1))
                            else {
                                return false;
                            };
                            let Some(bl) = char_grid.get(y + 1).and_then(|row| row.get(x_left))
                            else {
                                return false;
                            };
                            let Some(br) = char_grid.get(y + 1).and_then(|row| row.get(x + 1))
                            else {
                                return false;
                            };
                            tracing::trace!(x, y, ?tl, ?tr, ?br, ?bl);
                            // look at the diagonal for either 'M' or 'S' and then look for the other in the opposite corner
                            if !match tl {
                                'M' => br == &'S',
                                'S' => br == &'M',
                                _ => false,
                            } {
                                return false;
                            }
                            // and the same for the other diagonal
                            if !match tr {
                                'M' => bl == &'S',
                                'S' => bl == &'M',
                                _ => false,
                            } {
                                return false;
                            }
                            true
                        }
                        _ => false,
                    })
                    .count()
            })
            .sum();
        Ok(count.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "18";
    const EXPECTED_B: &str = "9";

    #[test]
    #[traced_test]
    fn day_4_a() {
        let res = solution::part_a(TEST_INPUT);
        assert_eq!(EXPECTED_A, res.unwrap());
    }

    #[test]
    #[traced_test]
    fn day_4_b() {
        let res = solution::part_b(TEST_INPUT);
        assert_eq!(EXPECTED_B, res.unwrap());
    }
}
