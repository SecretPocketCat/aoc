pub mod solution {
    #[tracing::instrument(fields(input = format!("{:?}[...]", input.lines().next())))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let count = input
            .lines()
            .filter(|l| {
                let nums: Vec<_> = l
                    .split_whitespace()
                    .flat_map(|num| num.parse::<i64>())
                    .collect();
                let mut inc = None;
                nums.windows(2).all(|nums| {
                    let delta = nums[1] - nums[0];
                    let sign = delta > 0;
                    match inc {
                        Some(inc) if inc != sign => {
                            return false;
                        }
                        None => {
                            inc = Some(delta > 0);
                        }
                        _ => {}
                    }

                    (1..=3).contains(&delta.abs())
                })
            })
            .count();
        Ok(count.to_string())
    }

    #[tracing::instrument(fields(input = format!("{:?}[...]", input.lines().next())))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        fn report_valid(nums: Vec<i64>, dampened: bool) -> bool {
            let mut sign = None;
            let mut i = 0;
            while let Some(b) = nums.get(i + 1).cloned() {
                let a = nums[i];
                let delta = b - a;
                let mut invalid = false;
                if !(1..=3).contains(&delta.abs()) {
                    invalid = true;
                } else {
                    let curr_sign = delta.signum();
                    match sign {
                        Some(sign) if sign != curr_sign => {
                            invalid = true;
                        }
                        None => {
                            sign = Some(curr_sign);
                        }
                        _ => {}
                    }
                }

                match (invalid, dampened) {
                    (true, true) => {
                        return false;
                    }
                    (true, false) => {
                        let mut n1 = nums.clone();
                        n1.remove(i);
                        let mut n2 = nums.clone();
                        n2.remove(i + 1);
                        let res = report_valid(n1, true) || report_valid(n2, true);
                        // if !res {
                        //     tracing::warn!(?nums, a, b, "nope");
                        // }
                        return res;
                    }
                    _ => {
                        i += 1;
                    }
                }
            }

            true
        }

        let count = input
            .lines()
            .filter(|l| {
                let nums: Vec<_> = l
                    .split_whitespace()
                    .flat_map(|num| num.parse::<i64>())
                    .collect();
                report_valid(nums, false)
            })
            .count();
        Ok(count.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "2";
    const EXPECTED_B: &str = "4";

    #[test]
    fn day_2_a() {
        let res = solution::part_a(TEST_INPUT);
        assert_eq!(EXPECTED_A, res.unwrap());
    }

    #[test]
    fn day_2_b() {
        let res = solution::part_b(TEST_INPUT);
        assert_eq!(EXPECTED_B, res.unwrap());
    }

    #[test]
    fn day_2_b_repro_1() {
        let res = solution::part_b("72 73 76 75 77 80 81");
        assert_eq!("1", res.unwrap());
    }

    #[test]
    fn day_2_b_repro_2() {
        let res = solution::part_b("85 86 87 85");
        assert_eq!("1", res.unwrap());
    }
}
