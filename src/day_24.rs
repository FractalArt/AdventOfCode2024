//! # Advent of Code 2024 - Day 24
//!
//! This module contains the solution of the [twentyfourth day's challenges](https://adventofcode.com/2024/day/24).
use itertools::Itertools;
use std::collections::HashMap as HM;

#[derive(Debug)]
enum Gate {
    And(String, String, String),
    Or(String, String, String),
    Xor(String, String, String),
}

fn parse_data(data: &str) -> (HM<String, usize>, Vec<Gate>) {
    let (wires, gates) = data.trim().split_once("\n\n").unwrap();
    (
        wires
            .split("\n")
            .map(|line| line.split_once(": ").expect("2"))
            .map(|(wire, value)| (wire.to_string(), value.parse::<usize>().unwrap()))
            .collect(),
        gates
            .split("\n")
            .map(|line| {
                let (input, output) = line.split_once(" -> ").unwrap();
                let split = input.split_whitespace().collect::<Vec<_>>();
                match (split[0], split[1], split[2]) {
                    (l, "AND", r) => Gate::And(l.to_string(), r.to_string(), output.to_string()),
                    (l, "OR", r) => Gate::Or(l.to_string(), r.to_string(), output.to_string()),
                    (l, _, r) => Gate::Xor(l.to_string(), r.to_string(), output.to_string()),
                }
            })
            .collect(),
    )
}

/// The solution to task 1 of day 24.
pub fn part_1(data: &str) -> usize {
    let (mut wires, gates) = parse_data(data);
    let mut treated_gates = std::collections::HashSet::new();
    let mut index = 0;

    while treated_gates.len() < gates.len() {
        index = (index + 1) % gates.len();
        if treated_gates.contains(&index) {
            continue;
        }
        match &gates[index] {
            Gate::And(l, r, o) if wires.contains_key(l) && wires.contains_key(r) => {
                *wires.entry(o.to_string()).or_default() = wires[l] & wires[r];
                treated_gates.insert(index);
            }
            Gate::Or(l, r, o) if wires.contains_key(l) && wires.contains_key(r) => {
                *wires.entry(o.to_string()).or_default() = wires[l] | wires[r];
                treated_gates.insert(index);
            }
            Gate::Xor(l, r, o) if wires.contains_key(l) && wires.contains_key(r) => {
                *wires.entry(o.to_string()).or_default() = wires[l] ^ wires[r];
                treated_gates.insert(index);
            }
            _ => {}
        }
    }

    wires
        .into_iter()
        .filter(|(key, _)| key.starts_with('z'))
        .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
        .map(|(_, val)| val)
        .enumerate()
        .fold(0, |sum, (pow, bit)| sum + bit * 2usize.pow(pow as u32))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
        assert_eq!(part_1(data), 2024);
    }
}
