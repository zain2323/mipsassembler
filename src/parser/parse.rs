use std::collections::HashMap;

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

fn parser(instruction: &str) -> Vec<&str> {
    let instruction: Vec<&str> = split_instruction(instruction);
    let parsed_mnemonic = instruction[0];
    let mnemonics = instructions();
    let found = false;
    for menemonic in mnemonics {
        match menemonic == parsed_mnemonic {
            true => {
                found = true,
            }
            false => ()
        }
        if found == true {
            break;
        }
    }
    instruction
}

fn split_instruction(instruction: &str) -> Vec<&str> {
    instruction.split(' ').collect()
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
        let menemonic = split_instruction("add $r1 $r2 $r3");
        assert_eq!(vec!["add", "$r1" , "$r2" , "$r3"], menemonic);
    }
}