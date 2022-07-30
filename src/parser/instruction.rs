/*
This file will take instruction of mips32 as input and will convert it into binary.
Any parsing errors will be raised if found.
*/

pub enum Instruction {
    RTypeInstruction(RType),
    ITypeInstruction(IType),
    JTypeInstruction(JType),
}

/*
Struct for representing R-type instructions in Mips.
opcode = 6 bits
rs = 5 bits
rt = 5 bits
rd = 5 bits
shamt = 5 bits
funct = 6 bits
*/
struct RType {
    opcode: String,
    rs: String,
    rt: String,
    rd: String,
    shamt: String,
    funct: String,
}

/*
Struct for representing R-type instructions in Mips.
opcode = 6 bits
rs = 5 bits
rt = 5 bits
immediate = 16 bits
*/
struct IType {
    opcode: String,
    rs: String,
    rt: String,
    immediate: String,
}

/*
opcode: 6 bits
pseudo_address = 26 bits
 */
struct JType {
    opcode: String,
    pseudo_address: String,
}

impl RType {
    fn from(&self, instruction: Vec<&str>) -> RType {
        // Instruction = [opcode, rs, rt, rd, shamt, funct]
        RType {
            opcode: String::from("000000"),
            rs: instruction[1].to_string(),
            rt: instruction[2].to_string(),
            rd: instruction[3].to_string(),
            shamt: String::from("0"),
            funct: instruction[4].to_string(),
        }
    }
    fn add(&self) {}

    fn sub(&self) {}

    fn slt(&self) {}

    fn and(&self) {}

    fn or(&self) {}
}

impl IType {
    fn from(&self, instruction: Vec<&str>) -> IType {
        todo!()
    }

    fn lw(&self) {}

    fn sw(&self) {}

    fn beq(&self) {}
}

impl JType {
    fn jump(&self) {}
}
