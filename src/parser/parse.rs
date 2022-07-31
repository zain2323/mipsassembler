use std::collections::HashMap;
use std::cell::RefCell;
use std::process::id;

struct RegisterFile {
    registers: RefCell<Vec<Register>>,
    code: Vec<&'static str>

}

#[derive(Clone)]
struct Register {
    number: i8,
    data: i32,
    is_writeable: bool
}

impl Register {
    fn new(number: i8, data: i32, is_writeable: bool) -> Register {
        Register {
            number,
            data,
            is_writeable
        }
    }

    fn write(&mut self, data: i32) {
        if self.is_writeable {
            self.data = data;
        }
    }

    fn read(&self) -> i32 {
        self.data
    }
}

impl RegisterFile {
    fn new() -> RegisterFile {
        let mut registers: Vec<Register> = Vec::new();
        // Zero register -> hard coded zero value
        registers.push(Register::new(0,0, false));
        // One register -> Reserved for assembler
        registers.push(Register::new(1,0, false));
        // 2-3 -> Return value registers
        registers.push(Register::new(2,0, true));
        registers.push(Register::new(3,0, true));
        // 4-7 -> Argument registers
        registers.push(Register::new(4,0, true));
        registers.push(Register::new(5,0, true));
        registers.push(Register::new(6,0, true));
        registers.push(Register::new(7,0, true));
        // 8-15 -> Temporary registers (t0 - t7)
        // 24-25 -> (t8 - t9)
        registers.push(Register::new(8,0, true));
        registers.push(Register::new(9,0, true));
        registers.push(Register::new(10,0, true));
        registers.push(Register::new(11,0, true));
        registers.push(Register::new(12,0, true));
        registers.push(Register::new(13,0, true));
        registers.push(Register::new(14,0, true));
        registers.push(Register::new(15,0, true));
        // 16-23 -> Saved Registers (s0 - s7)
        registers.push(Register::new(16,0, true));
        registers.push(Register::new(17,0, true));
        registers.push(Register::new(18,0, true));
        registers.push(Register::new(19,0, true));
        registers.push(Register::new(20,0, true));
        registers.push(Register::new(21,0, true));
        registers.push(Register::new(22,0, true));
        registers.push(Register::new(23,0, true));
        // t8-t9 (24-25)
        registers.push(Register::new(23,0, true));
        registers.push(Register::new(24,0, true));
        // k0-k1 -> Kernel Registers(26 - 27)
        registers.push(Register::new(26,0, true));
        registers.push(Register::new(27,0, true));
        // gp -> Global data pointer
        registers.push(Register::new(28,0, true));
        // sp -> Stack pointer
        registers.push(Register::new(29,0, true));
        // fp -> Frame pointer
        registers.push(Register::new(30,0, true));
        // ra -> Return Register(31)
        registers.push(Register::new(31,0, true));

        // Register codes
        let code = vec!["zero", "at", "v0", "v1", "a0", "a1", "a2", "a3", "t0", "t1", "t2", "t3", "t4", "t5", "t6",
                                  "t7", "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7", "t8", "t9", "k0", "k1", "gp", "sp",
                                  "fp", "ra"];

        RegisterFile {
            registers: RefCell::new(registers),
            code
        }
    }

    fn write(&self, code: String, data: i32) {
        let index = self.get_reg_number(code);
        let mut reg_file = self.registers.borrow_mut();
        let reg = reg_file.get(index).unwrap();
        if reg.is_writeable {
            let new_reg = Register {
                number: reg.number,
                data,
                is_writeable: reg.is_writeable
            };
            reg_file[index] = new_reg;
        }
    }

    fn read(&self, code: String) -> i32 {
        let index = self.get_reg_number(code);
        self.registers.borrow().get(index).unwrap().read()
    }

    fn get_reg_number(&self, code: String) -> usize {
        self.code.iter().position(|&item| item == code).expect("Unrecognized register")
    }

}
#[derive(Clone)]
#[derive(Debug)]
struct ISA {
    opcode: String,
    mnemonic: String,
    funct: Option<String>,
}

impl ISA {
    fn new() -> HashMap<&'static str, Vec<ISA>> {
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

fn parser(instruction: &str) -> ISA {
    let instruction: Vec<&str> = split_instruction(instruction);
    let parsed_mnemonic = instruction[0];
    let parsed_instruction = match is_valid(parsed_mnemonic) {
        Ok(ins) => ins,
        Err(E) => panic!("{}", E),
    };
    parsed_instruction
}

// Finds the instruction in the ISA, returns Option Enum
fn find_instruction(parsed_mnemonic: &str) -> Option<ISA> {
    let isa = ISA::new();
    for format in isa.keys() {
        let instructions = isa.get(format).unwrap();
        for ins in instructions {
            if ins.mnemonic == parsed_mnemonic {
               return Some(ins.clone());
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
        None => Err("Instruction not found!"),
    };
    parsed_instruction
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_instructions() {
        let instructions = ISA::new();
        println!("{:?}", instructions);
    }

    #[test]
    fn instruction_split() {
        let mnemonic = split_instruction("add $r1 $r2 $r3");
        assert_eq!(vec!["add", "$r1", "$r2", "$r3"], mnemonic);
    }

    #[test]
    fn testing_isa() {
        // I format
        let sw = "sw $r1 2($rs)";
        let lw = "lw $r1,2($rs)";
        let beq = "beq $r1, $r2, label";
        // R format based Instructions
        let add = "add $r1, $r2, $r3";
        let sub = "sub $r1, $r2, $r3";
        let slt = "slt $r1, $r2, $r3";
        let and = "and $r1, $r2, $r3";
        let or = "or $r1, $r2, $r3";
        // J format
        let jump = "jump label";

        // All parsed instructions
        let sw_parsed = parser(sw);
        let lw_parsed = parser(lw);
        let beq_parsed = parser(beq);
        let add_parsed = parser(add);
        let sub_parsed = parser(sub);
        let slt_parsed = parser(slt);
        let and_parsed = parser(and);
        let or_parsed = parser(or);
        let jump_parsed = parser(jump);

        assert_eq!("sw", sw_parsed.mnemonic);
        assert_eq!("lw", lw_parsed.mnemonic);
        assert_eq!("beq", beq_parsed.mnemonic);
        assert_eq!("add", add_parsed.mnemonic);
        assert_eq!("sub", sub_parsed.mnemonic);
        assert_eq!("slt", slt_parsed.mnemonic);
        assert_eq!("and", and_parsed.mnemonic);
        assert_eq!("or", or_parsed.mnemonic);
        assert_eq!("jump", jump_parsed.mnemonic);
    }

    #[test]
    fn get_reg_from_code() {
        let reg_file = RegisterFile::new();
        let zero = reg_file.get_reg_number("zero".to_string());
        let t0 = reg_file.get_reg_number("t0".to_string());
        let t8 = reg_file.get_reg_number("t8".to_string());
        let s7 = reg_file.get_reg_number("s7".to_string());
        let v1 = reg_file.get_reg_number("v1".to_string());
        let ra = reg_file.get_reg_number("ra".to_string());
        assert_eq!(0, zero);
        assert_eq!(8, t0);
        assert_eq!(24, t8);
        assert_eq!(23, s7);
        assert_eq!(3, v1);
        assert_eq!(31, ra);
    }
    #[test]
    fn read_from_reg_file() {
        let reg_file = RegisterFile::new();
        let zero = reg_file.read("zero".to_string());
        let t8 = reg_file.read("t8".to_string());
        assert_eq!(0, zero);
        assert_eq!(0, t8);
    }

    #[test]
    fn write_to_reg_file() {
        let mut reg_file = RegisterFile::new();
        reg_file.write("t0".to_string(), 10);
        reg_file.write("zero".to_string(), 11);
        let t0 = reg_file.read("t0".to_string());
        let zero = reg_file.read("zero".to_string());
        assert_eq!(t0, 10);
        assert_eq!(zero, 0)
    }
}
