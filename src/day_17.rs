//! # Advent of Code 2024 - Day 17
//!
//! This module contains the solution of the [seventeenth day's challenges](https://adventofcode.com/2024/day/17).

#[derive(Debug, PartialEq)]
struct State {
    ra: usize,
    rb: usize,
    rc: usize,
    ip: usize,
    out: Vec<String>,
    program: Vec<usize>,
}

impl State {
    fn combo(&self, operand: usize) -> usize {
        match operand {
            4 => self.ra,
            5 => self.rb,
            6 => self.rc,
            _ => operand,
        }
    }

    fn step(&mut self) {
        let opcode = self.program[self.ip];
        let operand = self.program[self.ip + 1];

        if !(opcode == 3 && self.ra != 0) {
            self.ip += 2;
        }

        match opcode {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxc(),
            5 => self.out(operand),
            6 => self.bdv(operand),
            _ => self.cdv(operand),
        }
    }

    fn adv(&mut self, operand: usize) {
        self.ra /= 2usize.pow(self.combo(operand) as u32);
    }

    fn bxl(&mut self, operand: usize) {
        self.rb ^= operand;
    }

    fn bst(&mut self, operand: usize) {
        self.rb = self.combo(operand) % 8;
    }

    fn bxc(&mut self) {
        self.rb ^= self.rc;
    }

    fn bdv(&mut self, operand: usize) {
        self.rb = self.ra / 2usize.pow(self.combo(operand) as u32);
    }

    fn cdv(&mut self, operand: usize) {
        self.rc = self.ra / 2usize.pow(self.combo(operand) as u32);
    }

    fn jnz(&mut self, operand: usize) {
        if self.ra != 0 {
            self.ip = operand;
        }
    }

    fn out(&mut self, operand: usize) {
        self.out.push((self.combo(operand) % 8).to_string());
    }
}

/// The solution to task 1 of day 17.
pub fn part_1(program: Vec<usize>, ra: usize, rb: usize, rc: usize) -> String {
    let mut state = State {
        ra,
        rb,
        rc,
        ip: 0,
        out: vec![],
        program,
    };

    while state.ip < state.program.len() - 1 {
        state.step();
    }

    state.out.join(",")
}

/// The solution to task 2 of day 17.
///
/// Converting the program code of the input:
/// 2,4,1,5,7,5,1,6,0,3,4,3,5,5,3,0
/// to assembly code, one gets:
///
/// BST 4;  B = COMBO(4) % 8 = A % 8
/// BXL 5;  B = B ^ 5 = (A % 8) ^ 5
/// CDV 5;  C = A / 2**(COMBO(5)) = A / 2**B = A / 2**(A%8 ^ 5)
/// BXL 6;  B = B ^ 6 = (A % 8) ^ 5 ^ 6 = (A % 8) ^ 3
/// ADV 3;  A = A / 2 ** (COMBO(3)) = A / 2**3 = A / 8
/// BXC 3;  B = B ^ C = (A % 8) ^ 3 ^ A / 2**(A%8 ^ 5)
/// OUT 5;  COMBO(5) % 8 = B % 8 = ((A % 8) ^ 3 ^ A / 2**((A%8) ^ 5)) % 8
///
/// The reversed function output for a given initial A register value
/// is thus
///
/// ((A % 8) ^ 3 ^ A / 2**(A % 8) ^ 5) % 8
///
/// Notice that for the example input from the problem, the reversed function
/// is simply:
///
/// (A / 8) % 8
///
/// This program has the nice property that the output value only depends
/// on the initial register entry A in every iteration.
/// The jump only occurs at the end, so we have an iterative function.
/// The current iteration depends on the value of the A register at the
/// end of the previous iteration.
///
/// We can see that in every iteration, A is decreased by a factor 8 (A / 3).
/// We thus need to find the smallest value of A that gives 0 in one iteration.
/// This is the last A value that will end the program.
/// From there we start at A * 8 and find the value of A that gives the second but last program
/// program byte.
/// Then we iterate until we find the value of A that yields the first program byte.
/// This is the return value.
pub fn part_2<F>(program: Vec<usize>, reversed: F) -> usize
where
    F: Fn(usize) -> usize,
{
    program
        .into_iter()
        .rev()
        .scan(0, |start, target| loop {
            if reversed(*start) == target {
                *start *= 8;
                return Some(*start / 8);
            } else {
                *start += 1;
            }
        })
        .last()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(vec![0, 1, 5, 4, 3, 0], 729, 0, 0),
            "4,6,3,5,6,3,5,2,1,0".to_string()
        );
        assert_eq!(
            part_1(vec![0, 3, 5, 4, 3, 0], 117440, 0, 0),
            "0,3,5,4,3,0".to_string()
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(vec![0, 3, 5, 4, 3, 0], |a| (a / 8) % 8), 117440);
    }
}
