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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(vec![0, 1, 5, 4, 3, 0], 729, 0, 0),
            "4,6,3,5,6,3,5,2,1,0".to_string()
        );
    }
}
