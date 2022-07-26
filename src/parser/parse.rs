use std::collections::HashMap;
use crate::parser::instruction::Instruction;

fn instructions() -> HashMap<&'static str, Vec<&'static str>>{
    let r_type = vec!["add", "sub", "slt", "and", "or"];
    let i_type = vec!["lw", "sw", "beq"];
    let j_type = vec!["jump"];

    let mut mnemonics = HashMap::new();
    mnemonics.insert("r", r_type);
    mnemonics.insert("i", i_type);
    mnemonics.insert("j", j_type);
    mnemonics
}

fn opcodes() -> HashMap<&'static str, &'static str> {
    let mut opcodes =HashMap::new();
    opcodes.insert("r", "000000");
    todo!()
}

fn parser(instruction: &str) -> Result<&str, &str> {
    let instruction: Vec<&str> = split_instruction(instruction);
    let parsed_mnemonic = instruction[0];
    let instructions = instructions();
    let mut found = false;
    let mut ins_format = "";
    for instruction_type in instructions.keys() {
        if found == true {break;}
        for mnemonic in instructions.get(instruction_type) {
            match mnemonic == &parsed_mnemonic {
                true => {
                    found = true;
                    ins_format = instruction_type;
                }
                false => ()
            }
            if found == true {
                break;
            }
        }
    }

    match found {
        true => {Ok(parsed_mnemonic)}
        false => {Err("Invalid instruction")}
    }
}

fn split_instruction(instruction: &str) -> Vec<&str> {
    instruction.split(' ').collect()
}

fn is_valid(instruction: Vec<&str>) -> Result<Instruction, &str>{
    let instructions = instructions();
    let parsed_mnemonic = instruction[0];
    let mut found = false;
    let mut ins_format = "";
    for instruction_type in instructions.keys() {
        if found == true {break;}
        for mnemonic in instructions.get(instruction_type) {
            match mnemonic == &parsed_mnemonic {
                true => {
                    found = true;
                    ins_format = instruction_type;
                }
                false => ()
            }
            if found == true {
                break;
            }
        }
    }
    match found {
        true => {
            let parsed_instruction = Instruction::RTypeInstruction{
                0: ()
            };
        }
        false => {Err("Invalid instruction")}
    }

   todo!()
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
        assert_eq!(vec!["add", "$r1" , "$r2" , "$r3"], mnemonic);
    }

    #[test]
    fn is_instruction_found() {
        let instruction = "lw $r1 2($rs)";
        let parsed = parser(instruction);
        match parsed {
            Ok(ins) => {
                assert_eq!("lw", ins);
            },
            Err(E) => {
                panic!("{}", E)
            }
        }
    }
}