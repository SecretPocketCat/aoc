pub mod solution {
    use std::{
        collections::{HashMap, HashSet},
        ops::Range,
    };

    use glam::{IVec2, UVec2};
    use nom::{
        bytes::complete::tag,
        character::complete::{self, space1},
        sequence::{preceded, separated_pair},
        IResult, Parser,
    };

    type QuadrantRange = (Range<u32>, Range<u32>);
    struct Map {
        size: UVec2,
        quadrants: Vec<QuadrantRange>,
    }
    impl Map {
        pub(crate) fn new(size: UVec2) -> Self {
            let half_size = size / 2;
            Self {
                size,
                quadrants: vec![
                    (0..half_size.x, 0..half_size.y),
                    ((half_size.x + 1)..size.x, 0..half_size.y),
                    (0..half_size.x, (half_size.y + 1)..size.y),
                    ((half_size.x + 1)..size.x, (half_size.y + 1)..size.y),
                ],
            }
        }

        pub(crate) fn quadrant(&self, position: UVec2) -> Option<u8> {
            self.quadrants
                .iter()
                .enumerate()
                .find(|(_, q)| q.0.contains(&position.x) && q.1.contains(&position.y))
                .map(|(i, _)| i as _)
        }
    }

    struct Robot {
        position: UVec2,
        velocity: IVec2,
    }
    impl Robot {
        pub fn parse(input: &str) -> IResult<&str, Self> {
            separated_pair(
                preceded(tag("p="), parse_ivec2),
                space1,
                preceded(tag("v="), parse_ivec2),
            )
            .map(|(position, velocity)| Self {
                position: position.as_uvec2(),
                velocity,
            })
            .parse(input)
        }

        pub fn quadrant(&self, step_count: u8, map: &Map) -> Option<u8> {
            let final_pos = (self.position.as_ivec2() + self.velocity * i32::from(step_count))
                .rem_euclid(map.size.as_ivec2())
                .as_uvec2();
            map.quadrant(final_pos)
        }
    }

    pub(crate) fn solve_a(input: &str, map_size: UVec2) -> String {
        let map = Map::new(map_size);
        let lines: Vec<_> = input.lines().collect();
        let quadrants: Vec<_> = lines
            .into_iter()
            .filter_map(|l| {
                let (_, robot) = Robot::parse(l).expect("Valid robot line");
                robot.quadrant(100, &map).map(usize::from)
            })
            .collect();
        let quadrants: HashMap<_, _> =
            quadrants
                .into_iter()
                .fold(HashMap::new(), |mut map, quadrant| {
                    map.entry(quadrant).and_modify(|val| *val += 1).or_insert(1);
                    map
                });
        let res: u32 = quadrants.values().product();
        res.to_string()
    }

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        Ok(solve_a(input, UVec2::new(101, 103)))
    }

    #[tracing::instrument(skip(input))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        let map = Map::new(UVec2::new(101, 103));
        let lines: Vec<_> = input.lines().collect();
        let mut robots: Vec<_> = lines
            .into_iter()
            .map(|l| {
                let (_, robot) = Robot::parse(l).expect("Valid robot line");
                robot
            })
            .collect();

        let mut robot_positions = HashSet::with_capacity(robots.len());
        for i in 1..100_000 {
            robot_positions.clear();
            for r in &mut robots {
                r.position = (r.position.as_ivec2() + r.velocity)
                    .rem_euclid(map.size.as_ivec2())
                    .as_uvec2();
                robot_positions.insert(r.position);
                if (1..10)
                    .all(|offset| robot_positions.contains(&(r.position + (UVec2::ONE * offset))))
                {
                    return Ok(i.to_string());
                }
            }
        }

        unreachable!();
    }

    fn parse_ivec2(input: &str) -> IResult<&str, IVec2> {
        let (input, (x, y)) = separated_pair(complete::i32, tag(","), complete::i32)(input)?;
        Ok((input, IVec2::new(x, y)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::UVec2;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "12";
    const EXPECTED_B: &str = "7774";

    #[test]
    #[traced_test]
    fn day_14_a() {
        let res = solution::solve_a(TEST_INPUT, UVec2::new(11, 7));
        assert_eq!(EXPECTED_A, res);
    }

    #[test]
    #[traced_test]
    fn day_14_b() {
        let res = solution::part_b(include_str!("../../target/inputs/day-14/input.txt"));
        assert_eq!(EXPECTED_B, res.unwrap());
    }
}
