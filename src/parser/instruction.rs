/*
This file will take instruction of mips32 as input and will convert it into binary.
Any parsing errors will be raised if found.
*/

pub enum Instruction {
    RTypeInstruction(RType),
    ITypeInstruction(IType),
    JTypeInstruction(JType)
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
    funct: String
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
    immediate: String
}

/*
opcode: 6 bits
pseudo_address = 26 bits
 */
struct JType {
    opcode: String,
    pseudo_address: String
}


impl RType {
    fn from(&self, instruction: Vec<&str>) -> RType {
        RType {
            opcode: String::from("000000"),
            rs: ,
            rt:,
            rd:,
            shamt: String::from("0"),
            funct:
        }
    }
    fn add(&self) {

    }

    fn sub(&self) {

    }

    fn slt(&self) {

    }

    fn and(&self) {

    }

    fn or(&self) {

    }
}

impl IType {
    fn lw(&self) {

    }

    fn sw(&self) {

    }

    fn beq(&self) {

    }
}

impl JType {
    fn jump(&self) {
        
    }
}