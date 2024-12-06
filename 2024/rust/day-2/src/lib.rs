pub mod solution {
    #[tracing::instrument(fields(input = format!("{:?}[...]", input.lines().next())))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let count = input
            .lines()
            .filter(|l| {
                let nums: Vec<i64> = l.split_whitespace().flat_map(str::parse).collect();
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
        fn report_valid(nums: &[i64], dampened: bool) -> bool {
            let mut sign = None;
            let mut i = 0;
            while let Some(b) = nums.get(i + 1).copied() {
                let a = nums[i];
                let delta = b - a;
                let mut invalid = false;
                if (1..=3).contains(&delta.abs()) {
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
                } else {
                    invalid = true;
                }

                match (invalid, dampened) {
                    (true, true) => {
                        return false;
                    }
                    (true, false) => {
                        // remove the initial num in case the sort direction was wrong
                        let mut n0 = nums.to_vec();
                        n0.remove(0);
                        // remove the current index
                        let mut n1 = nums.to_vec();
                        n1.remove(i);
                        // remove the next index
                        let mut n2 = nums.to_vec();
                        n2.remove(i + 1);
                        return report_valid(&n0, true)
                            || report_valid(&n1, true)
                            || report_valid(&n2, true);
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
                let nums: Vec<i64> = l.split_whitespace().flat_map(str::parse).collect();
                report_valid(&nums, false)
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
