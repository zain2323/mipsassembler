use crate::parser::instruction::Instruction;
use core::panicking::panic;
use std::collections::HashMap;

struct ISA {
    opcode: String,
    mnemonic: String,
    funct: Option<String>,
}

impl ISA {
    fn new() -> HashMap<&str, Vec<ISA>> {
        let add = ISA {
            opcode: String::from("000000"),
            mnemonic: String::from("add"),
            funct: Some(String::from("100000")),
        };

        let sub = ISA {
            opcode: String::from("000000"),
            mnemonic: String::from("sub"),
            funct: Some(String::from("100010")),
        };

        let slt = ISA {
            opcode: String::from("000000"),
            mnemonic: String::from("slt"),
            funct: Some(String::from("101010")),
        };

        let and = ISA {
            opcode: String::from("000000"),
            mnemonic: String::from("and"),
            funct: Some(String::from("100100")),
        };

        let or = ISA {
            opcode: String::from("000000"),
            mnemonic: String::from("or"),
            funct: Some(String::from("100101")),
        };

        let lw = ISA {
            opcode: String::from("100011"),
            mnemonic: String::from("lw"),
            funct: None,
        };

        let sw = ISA {
            opcode: String::from("101011"),
            mnemonic: String::from("sw"),
            funct: None,
        };

        let beq = ISA {
            opcode: String::from("000100"),
            mnemonic: String::from("beq"),
            funct: None,
        };

        let jump = ISA {
            opcode: String::from("000010"),
            mnemonic: String::from("jump"),
            funct: None,
        };
        let mut isa_hashmap: HashMap<&str, Vec<ISA>> = HashMap::new();
        isa_hashmap.insert("r", vec![add, sub, slt, and, or]);
        isa_hashmap.insert("i", vec![lw, sw, beq]);
        isa_hashmap.insert("j", vec![jump]);
        isa_hashmap
    }
}

fn parser(instruction: &str) -> Result<&str, &str> {
    let instruction: Vec<&str> = split_instruction(instruction);
    let parsed_mnemonic = instruction[0];
    let parsed_instruction = match is_valid(parsed_mnemonic) {
        Ok(ins) => ins,
        Err(E) => panic!(E),
    };

    todo!()
}

// Finds the instruction in the ISA, returns Option Enum
fn find_instruction(parsed_mnemonic: &str) -> Option<ISA> {
    let isa = ISA::new();
    for format in isa.keys() {
        let instructions = isa.get(format).unwrap();
        for ins in instructions {
            if ins.mnemonic == parsed_mnemonic {
                Some(insa)
            }
        }
    }
    None
}

fn split_instruction(instruction: &str) -> Vec<&str> {
    instruction.split(' ').collect()
}

fn is_valid(parsed_mnemonic: &str) -> Result<ISA, &str> {
    let parsed_instruction = match find_instruction(parsed_mnemonic) {
        Some(ins) => Ok(ins),
        None() => Err("Instruction not found!"),
    };
    parsed_instruction
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_instructions() {
        let mnemonics = instructions();
        println!("{:?}", mnemonics);
    }

    #[test]
    fn instruction_split() {
        let mnemonic = split_instruction("add $r1 $r2 $r3");
        assert_eq!(vec!["add", "$r1", "$r2", "$r3"], mnemonic);
    }

    #[test]
    fn is_instruction_found() {
        let instruction = "lw $r1 2($rs)";
        let parsed = parser(instruction);
        match parsed {
            Ok(ins) => {
                assert_eq!("lw", ins);
            }
            Err(E) => {
                panic!("{}", E)
            }
        }
    }
}
