pub mod solution {
    enum ControlFlow {
        Advance,
        Jump(u8),
    }

    struct Computer {
        register_a: usize,
        register_b: usize,
        register_c: usize,
        instruction_pointer: usize,
        instructions: Vec<u8>,
        output: Vec<usize>,
    }
    impl Computer {
        fn run(&mut self) {
            while let (Some(op), Some(operand)) = (
                self.instructions.get(self.instruction_pointer),
                self.instructions.get(self.instruction_pointer + 1),
            ) {
                self.eval_op(*op, *operand);
            }
        }

        fn eval_combo_operand(&self, operand: u8) -> usize {
            match operand {
                4 => self.register_a,
                5 => self.register_b,
                6 => self.register_c,
                7 => unimplemented!("Reserved"),
                o => o as _,
            }
        }

        fn eval_op(&mut self, op: u8, operand: u8) {
            let flow = match op {
                0 => self.op_adv(operand),
                1 => self.op_bxl(operand),
                2 => self.op_bst(operand),
                3 => self.op_jnz(operand),
                4 => self.op_bxc(operand),
                5 => self.op_out(operand),
                6 => self.op_bdv(operand),
                7 => self.op_cdv(operand),
                _ => unimplemented!(),
            };
            self.instruction_pointer = match flow {
                ControlFlow::Advance => self.instruction_pointer + 2,
                ControlFlow::Jump(pointer) => usize::from(pointer),
            };
        }

        fn op_adv(&mut self, operand: u8) -> ControlFlow {
            self.register_a = self.div(operand);
            ControlFlow::Advance
        }

        fn op_bxl(&mut self, operand: u8) -> ControlFlow {
            self.register_b ^= usize::from(operand);
            ControlFlow::Advance
        }

        fn op_bst(&mut self, operand: u8) -> ControlFlow {
            self.register_b = self.eval_combo_operand(operand) % 8;
            ControlFlow::Advance
        }

        fn op_jnz(&mut self, operand: u8) -> ControlFlow {
            if self.register_a == 0 {
                ControlFlow::Advance
            } else {
                ControlFlow::Jump(operand)
            }
        }

        fn op_bxc(&mut self, _operand: u8) -> ControlFlow {
            self.register_b ^= self.register_c;
            ControlFlow::Advance
        }

        fn op_out(&mut self, operand: u8) -> ControlFlow {
            self.output.push(self.eval_combo_operand(operand) % 8);
            ControlFlow::Advance
        }

        fn op_bdv(&mut self, operand: u8) -> ControlFlow {
            self.register_b = self.div(operand);
            ControlFlow::Advance
        }

        fn op_cdv(&mut self, operand: u8) -> ControlFlow {
            self.register_c = self.div(operand);
            ControlFlow::Advance
        }

        fn div(&self, operand: u8) -> usize {
            self.register_a
                / 2usize
                    .checked_pow(self.eval_combo_operand(operand) as _)
                    .expect("Valid range operand")
        }

        fn parse(input: &str) -> Self {
            let lines: Vec<_> = input.lines().collect();
            Self {
                register_a: Self::parse_register(lines[0]),
                register_b: Self::parse_register(lines[1]),
                register_c: Self::parse_register(lines[2]),
                instructions: Self::split_input_line(lines[4])
                    .split(',')
                    .flat_map(str::parse)
                    .collect(),
                instruction_pointer: 0,
                output: Vec::new(),
            }
        }

        fn split_input_line(line: &str) -> &str {
            line.split_once(": ").expect("Valid parsable line").1
        }

        fn parse_register(line: &str) -> usize {
            Self::split_input_line(line)
                .parse()
                .expect("Valid register a value")
        }
    }

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let mut computer = Computer::parse(input);
        computer.run();
        Ok(computer
            .output
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(","))
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
    const EXPECTED_A: &str = "4,6,3,5,6,3,5,2,1,0";
    const EXPECTED_B: &str = "todo_expected_b";

    #[test]
    #[traced_test]
    fn day_17_a() {
        let res = solution::part_a(TEST_INPUT);
        assert_eq!(EXPECTED_A, res.unwrap());
    }

    #[test]
    #[traced_test]
    fn day_17_a_2() {
        let res = solution::part_a(include_str!("../inputs/example_2.txt"));
        assert_eq!("0,1,2", res.unwrap());
    }

    #[test]
    #[traced_test]
    fn day_17_a_3() {
        let res = solution::part_a(include_str!("../inputs/example_3.txt"));
        assert_eq!("4,2,5,6,7,7,7,7,3,1,0", res.unwrap());
    }

    #[test]
    #[traced_test]
    fn day_17_b() {
        let res = solution::part_b(TEST_INPUT);
        assert_eq!(EXPECTED_B, res.unwrap());
    }
}
