use itertools::Itertools;

use crate::input;

pub fn solve() -> (Option<String>, Option<usize>) {
    let input = input::DAY_17_INPUT;

    let part_1_total = solve_part_1(input);
    let part_2_total = solve_part_2(input);

    (part_1_total, part_2_total)
}

fn solve_part_1(input: &str) -> Option<String> {
    let mut computer = Computer::from_str(input);
    let output = computer.run();

    let stdout: String = output.iter().join(",");

    Some(stdout)
}

fn solve_part_2(input: &str) -> Option<usize> {
    let computer = Computer::from_str(input);
    computer.print_program();

    //let a = computer.correct_register_a();
    //Some(a)
    None
}

#[derive(Debug, Clone, Copy)]
enum OpCodes {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

impl OpCodes {
    fn from_digit(d: usize) -> Self {
        match d {
            0 => OpCodes::Adv,
            1 => OpCodes::Bxl,
            2 => OpCodes::Bst,
            3 => OpCodes::Jnz,
            4 => OpCodes::Bxc,
            5 => OpCodes::Out,
            6 => OpCodes::Bdv,
            7 => OpCodes::Cdv,
            _ => panic!("Unknown opcode"),
        }
    }

    fn to_digit(&self) -> usize {
        match self {
            OpCodes::Adv => 0,
            OpCodes::Bxl => 1,
            OpCodes::Bst => 2,
            OpCodes::Jnz => 3,
            OpCodes::Bxc => 4,
            OpCodes::Out => 5,
            OpCodes::Bdv => 6,
            OpCodes::Cdv => 7,
        }
    }
}

#[derive(Debug, Clone)]
struct Computer {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    program: Vec<(OpCodes, usize)>,
}

impl Computer {
    fn from_str(input: &str) -> Self {
        let mut register_a = 0;
        let mut register_b = 0;
        let mut register_c = 0;
        let mut program = Vec::new();

        for line in input.lines() {
            if line.is_empty() {
                continue;
            }

            if line.contains("Register A:") {
                let (_, number) = line.split_once("Register A:").unwrap();
                register_a = number.trim().parse().unwrap();
            } else if line.contains("Register B:") {
                let (_, number) = line.split_once("Register B:").unwrap();
                register_b = number.trim().parse().unwrap();
            } else if line.contains("Register C:") {
                let (_, number) = line.split_once("Register C:").unwrap();
                register_c = number.trim().parse().unwrap();
            } else {
                let (_, instructions) = line.split_once("Program:").unwrap();

                // Program is comma separater opcodes/operand pairs
                let mut instructions = instructions.split(",");

                while let (Some(opc), Some(ope)) = (instructions.next(), instructions.next()) {
                    let opcode = OpCodes::from_digit(opc.trim().parse().unwrap());
                    let operand = ope.trim().parse().unwrap();

                    program.push((opcode, operand));
                }
            }
        }

        Self {
            register_a,
            register_b,
            register_c,
            program,
        }
    }

    /// Helping for part 2 with printing what the program does.
    ///
    /// This is the program I have :
    /// 0: B = A % 8;
    /// 1: B = B ^ 6;
    /// 2: C = A >> B;
    /// 3: B = B ^ C;
    /// 4: B = B ^ 4;
    /// 5: Print B % 8;
    /// 6: A = A >> 3;
    /// 7: If A != 0 {JMP 0};
    ///
    fn print_program(self) {
        println!("Program: ");
        for (opcode, operand) in self.program.iter() {
            let operand_string: String = match *operand {
                c if (0..=3).contains(&c) => c.to_string(),
                4 => String::from("A"),
                5 => String::from("B"),
                6 => String::from("C"),
                c => panic!("Unknown combo operand {c}"),
            };

            match opcode {
                OpCodes::Adv => println!("A = A >> {}", operand_string),
                OpCodes::Bxl => println!("B = B ^ {}", operand),
                OpCodes::Bst => println!("B = {} % 8", operand_string),
                OpCodes::Jnz => println!("If A != 0 {{JMP {}}}", operand),
                OpCodes::Bxc => println!("B = B ^ C"),
                OpCodes::Out => println!("Print {} % 8", operand_string),
                OpCodes::Bdv => println!("B = A >> {}", operand_string),
                OpCodes::Cdv => println!("C = A >> {}", operand_string),
            }
        }
        println!("");
    }

    /// Runs the program and returns its output
    ///
    fn run(&mut self) -> Vec<usize> {
        let mut output = Vec::new();

        // This is our instruction pointer
        let mut i = 0;

        loop {
            // Load up the current instruction
            let instruction = self.program[i];
            // Determine combo operand (even though it could be literal for the opcode)
            let combo: usize = match instruction.1 {
                c if (0..=3).contains(&c) => c,
                4 => self.register_a,
                5 => self.register_b,
                6 => self.register_c,
                c => panic!("Unknown combo operand {c}"),
            };

            // Execute the OpCode instruction
            match instruction.0 {
                OpCodes::Adv => {
                    self.register_a = self.register_a / 2usize.pow(combo as u32);
                }
                OpCodes::Bxl => {
                    self.register_b = self.register_b ^ instruction.1;
                }
                OpCodes::Bst => {
                    self.register_b = combo % 8;
                }
                OpCodes::Jnz => {
                    if self.register_a != 0 {
                        i = instruction.1 / 2;
                        continue;
                    }
                }
                OpCodes::Bxc => {
                    self.register_b = self.register_b ^ self.register_c;
                }
                OpCodes::Out => {
                    output.push(combo % 8);
                }
                OpCodes::Bdv => {
                    self.register_b = self.register_a / 2usize.pow(combo as u32);
                }
                OpCodes::Cdv => {
                    self.register_c = self.register_a / 2usize.pow(combo as u32);
                }
            }

            //Adjust the instruction pointer
            i += 1;
            if i >= self.program.len() {
                break;
            }
        }
        output
    }

    /// Tried brute forcing but it takes too long.
    /// Instead, we looked at the program itself (see `print_program`) try to
    /// trace back the values of the registeress based on the output.
    pub fn correct_register_a(&self) -> usize {
        // This is the wished output
        let target: Vec<usize> = self
            .program
            .iter()
            .flat_map(|(opcode, operand)| [opcode.to_digit(), *operand])
            .collect();

        // Final value of a is 0, because else we jump back to instruction 1 (at least in our
        // program)
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;

        // the target size tells us how many iterations as there is only 1 print statement
        // in our program (at least for my input)

        todo!("");
    }

    pub fn correct_program_from(
        program: &[usize],
        iteration: usize,
        a: usize,
        b: usize,
        c: usize,
    ) -> (usize, usize, usize) {
        todo!();
    }
}
