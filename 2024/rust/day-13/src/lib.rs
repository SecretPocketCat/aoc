pub mod solution {
    use glam::U64Vec2;
    use grid::prelude::pathfinding::dijkstra;
    use nom::{
        bytes::complete::{tag, take_till1},
        character::complete::{self},
        sequence::{preceded, separated_pair},
        IResult,
    };
    use rayon::prelude::*;

    #[derive(Debug)]
    struct Machine {
        button_a: U64Vec2,
        button_b: U64Vec2,
        prize: U64Vec2,
    }
    impl Machine {
        fn token_count(&self, limit: u32) -> Option<u32> {
            dijkstra(
                &(U64Vec2::ZERO, 0, 0),
                |(pos, presses_a, presses_b)| {
                    if *presses_a < limit && *presses_b < limit {
                        vec![
                            ((pos + self.button_a, *presses_a + 1, *presses_b), 3),
                            ((pos + self.button_b, *presses_a, *presses_b + 1), 1),
                        ]
                    } else {
                        vec![]
                    }
                },
                |(pos, ..)| *pos == self.prize,
            )
            .map(|(_, cost)| cost)

            // fn solve(
            //     machine: &Machine,
            //     position: U64Vec2,
            //     tokens: u32,
            //     // todo: the presses are per each btn not cumulative
            //     btn_presses: u8,
            //     min: &mut u32,
            // ) -> Option<u32> {
            //     // tracing::warn!(?position);
            //     if btn_presses >= 150
            //         || position.x > machine.prize.x
            //         || position.y > machine.prize.y
            //         || tokens >= *min
            //     {
            //         return None;
            //     }
            //     if position == machine.prize {
            //         *min = tokens;
            //         return Some(tokens);
            //     }
            //     match (
            //         solve(
            //             machine,
            //             position + machine.button_b,
            //             tokens + 1,
            //             btn_presses + 1,
            //             min,
            //         ),
            //         solve(
            //             machine,
            //             position + machine.button_a,
            //             tokens + 3,
            //             btn_presses + 1,
            //             min,
            //         ),
            //     ) {
            //         (None, None) => None,
            //         (None, Some(tokens)) | (Some(tokens), None) => Some(tokens),
            //         (Some(b), Some(a)) => Some(b.min(a)),
            //     }
            // }
            // let mut min = 400;
            // solve(self, U64Vec2::ZERO, 0, 0, &mut min)
        }
    }

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let lines: Vec<_> = input.lines().collect();
        let token_count: u32 = lines
            .par_chunks(4)
            .filter_map(|machine_lines| {
                let machine = parse_machine(machine_lines).expect("Parsed machine");
                machine.token_count(100)
            })
            .sum();
        Ok(token_count.to_string())
    }

    #[tracing::instrument(skip(input))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        let lines: Vec<_> = input.lines().collect();
        let token_count: u32 = lines
            .par_chunks(4)
            .filter_map(|machine_lines| {
                let mut machine = parse_machine(machine_lines).expect("Parsed machine");
                machine.prize += U64Vec2::ONE * 10000000000000;
                machine.token_count(10_000)
            })
            .sum();
        Ok(token_count.to_string())
    }

    fn parse_btn(input: &str) -> IResult<&str, U64Vec2> {
        preceded(
            till_incl_digit,
            separated_pair(complete::u64, tag(", Y+"), complete::u64),
        )(input)
        .map(|(input, (x, y))| (input, U64Vec2::new(x, y)))
    }

    fn parse_prize(input: &str) -> IResult<&str, U64Vec2> {
        preceded(
            till_incl_digit,
            separated_pair(complete::u64, tag(", Y="), complete::u64),
        )(input)
        .map(|(input, (x, y))| (input, U64Vec2::new(x, y)))
    }

    fn parse_machine(lines: &[&str]) -> anyhow::Result<Machine> {
        let (_, button_a) = parse_btn(lines[0]).map_err(|e| e.to_owned())?;
        let (_, button_b) = parse_btn(lines[1]).map_err(|e| e.to_owned())?;
        let (_, prize) = parse_prize(lines[2]).map_err(|e| e.to_owned())?;
        Ok(Machine {
            button_a,
            button_b,
            prize,
        })
    }

    fn till_incl_digit(input: &str) -> IResult<&str, &str> {
        take_till1(|c: char| c.is_ascii_digit())(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "480";
    const EXPECTED_B: &str = "todo_expected_b";

    #[test]
    #[traced_test]
    fn day_13_a() {
        let res = solution::part_a(TEST_INPUT);
        assert_eq!(EXPECTED_A, res.unwrap());
    }

    #[test]
    #[traced_test]
    fn day_13_b() {
        let res = solution::part_b(TEST_INPUT);
        assert_eq!(EXPECTED_B, res.unwrap());
    }
}
