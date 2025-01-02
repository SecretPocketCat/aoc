pub mod solution {
    use glam::U64Vec2;
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
        fn token_count(&self, limit: Option<u32>) -> Option<u64> {
            // use Cramer's rule
            let determinant = Self::determinant_mat2(self.button_a, self.button_b);
            let determinant_x = Self::determinant_mat2(self.prize, self.button_b);
            let determinant_y = Self::determinant_mat2(self.button_a, self.prize);
            if (determinant_x % determinant)
                .abs()
                .max((determinant_y % determinant).abs())
                > 0
            {
                return None;
            }
            let x = (determinant_x / determinant) as u64;
            let y = (determinant_y / determinant) as u64;
            if limit.is_none_or(|limit| x.max(y) < u64::from(limit)) {
                Some(x * 3 + y)
            } else {
                None
            }
        }

        fn determinant_mat2(row_a: U64Vec2, row_b: U64Vec2) -> i64 {
            (row_a.x * row_b.y) as i64 - (row_b.x * row_a.y) as i64
        }
    }

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let lines: Vec<_> = input.lines().collect();
        let token_count: u64 = lines
            .par_chunks(4)
            .filter_map(|machine_lines| {
                let machine = parse_machine(machine_lines).expect("Parsed machine");
                machine.token_count(Some(100))
            })
            .sum();
        Ok(token_count.to_string())
    }

    #[tracing::instrument(skip(input))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        let lines: Vec<_> = input.lines().collect();
        let token_count: u64 = lines
            .par_chunks(4)
            .filter_map(|machine_lines| {
                let mut machine = parse_machine(machine_lines).expect("Parsed machine");
                machine.prize += U64Vec2::ONE * 10_000_000_000_000;
                machine.token_count(None)
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
        let (_, button_a) =
            parse_btn(lines[0]).map_err(nom::Err::<nom::error::Error<&str>>::to_owned)?;
        let (_, button_b) =
            parse_btn(lines[1]).map_err(nom::Err::<nom::error::Error<&str>>::to_owned)?;
        let (_, prize) =
            parse_prize(lines[2]).map_err(nom::Err::<nom::error::Error<&str>>::to_owned)?;
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

    #[test]
    #[traced_test]
    fn day_13_a() {
        let res = solution::part_a(TEST_INPUT);
        assert_eq!(EXPECTED_A, res.unwrap());
    }
}
