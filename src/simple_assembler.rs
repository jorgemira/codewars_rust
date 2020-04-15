// https://www.codewars.com/kata/58e24788e24ddee28e000053

use std::collections::HashMap;

pub fn simple_assembler(program: Vec<&str>) -> HashMap<String, i64> {
    let mut registers = HashMap::new();
    let mut p = 0;
    while p < program.len() {
        let instr = program[p].split_whitespace()
            .map(|x| String::from(x))
            .collect::<Vec<String>>();

        match &instr[0][..] {
            "mov" => {
                registers.insert(instr[1].clone(),
                                 instr[2].parse::<i64>()
                                     .unwrap_or_else(|_| *registers.get(&instr[2]).unwrap()));
            }
            "inc" => {
                registers.insert(instr[1].clone(),
                                 registers.get(&instr[1]).unwrap() + 1);
            }
            "dec" => {
                registers.insert(instr[1].clone(),
                                 registers.get(&instr[1]).unwrap() - 1);
            }
            "jnz" => {
                if instr[1].parse::<i64>()
                    .unwrap_or_else(|_| *registers.get(&instr[1]).unwrap()) != 0 {
                    p = (p as i32 + instr[2].parse::<i32>().unwrap() - 1) as usize;
                }
            }
            _ => ()
        }
        p += 1;
    }

    registers
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! map {
( $ ( $ key: expr => $ value: expr), * ) => {{
let mut map = HashMap::new();
$ (
map.insert( $ key.to_string(), $ value);
) *
map
}};
}

    #[test]
    fn short_tests() {
        let program = vec!["mov a 5", "inc a", "dec a", "dec a", "jnz a -1", "inc a"];
        let expected = map! { "a" => 1 };
        compare_registers(expected, simple_assembler(program));

        let program = vec![
            "mov c 12",
            "mov b 0",
            "mov a 200",
            "dec a",
            "inc b",
            "jnz a -2",
            "dec c",
            "mov a b",
            "jnz c -5",
            "jnz 0 1",
            "mov c a",
        ];
        let expected = map! { "a" => 409600, "c" => 409600, "b" => 409600};
        compare_registers(expected, simple_assembler(program));
    }

    fn compare_registers(expected: HashMap<String, i64>, actual: HashMap<String, i64>) {
        let result = expected
            .iter()
            .all(|(key, value)| actual.get(key).map(|v| v == value).unwrap_or(false));
        assert!(
            result,
            "Expected the registers to be like that:\n{:#?}\n\nBut got this:\n{:#?}\n",
            expected, actual
        )
    }
}