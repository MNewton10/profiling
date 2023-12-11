use crate::iomemfuns;
use crate::regfuns;

/// A struct representing the memory of the UM
/// 
/// # Fields
/// 
/// `memvec` is a vector of vectors of u32s, representing the memory of the UM
/// `free` is a vector of u32s, representing the indices of the free memory segments
/// `registers` is an array of u32s, representing the registers of the UM
/// `inst_ptr` is a u32, representing the instruction pointer of the UM
#[derive(Debug, Clone)]
pub struct UM {
    pub memvec: Vec<Vec<Umi>>,
    pub free: Vec<Umi>,
    pub registers: [Umi; 8],
    pub inst_ptr: Umi,
}
impl UM {
    /// A function that creates the UM
    /// 
    /// # Arguments
    /// 
    /// * `instructions` - A vector of u32s representing the instructions of the given program
    /// 
    /// # Returns
    /// 
    /// * A UM struct
    pub fn new(instructions: Vec<u32>) -> UM {
        UM {
            memvec: vec![instructions],
            free: vec![],
            registers: [0; 8],
            inst_ptr: 0,
        }
    }

    /// A function that adds an index to the vector of free indices
    /// 
    /// # Arguments
    /// 
    /// * `index` - The index to add to the vector of free indices
    pub fn free(&mut self, index: Umi) {
        self.memvec[index as usize] = vec![];
        self.free.push(index);
    }

    /// A function that allocates a new segment of memory
    /// 
    /// # Arguments
    /// 
    /// * `index` - The index of the vector to grab from within the vectors of memory
    /// * `offset` - The offset within the vector to grab from
    /// 
    /// # Returns
    /// 
    /// * The value at the given index and offset
    pub fn get (&self, index: Umi, offset: Umi) -> Umi {
        unsafe {
            *self.memvec.get_unchecked(index as usize).get_unchecked(offset as usize) as Umi
        }
    }

    /// A function that sets a value at a given index and offset
    /// 
    /// # Arguments
    /// 
    /// * `index` - The index of the vector
    /// * `offset` - The offset within the vector to grab from
    pub fn set (&mut self, index: Umi, offset: Umi, value: Umi) {
        unsafe {
            *self.memvec.get_unchecked_mut(index as usize).get_unchecked_mut(offset as usize) = value as Umi;
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, FromPrimitive)]
#[repr(u32)]
// An enumeration statement representing the 14 possible opcodes
enum Opcode {
    CMov,
    Load,
    Store,
    Add,
    Mult,
    Div,
    Nand,
    Halt,
    MapSeg,
    UnmapSeg,
    Output,
    Input,
    LoadProg,
    LoadVal,
}


use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
pub type Umi = u32;
pub struct Field {
    width: u32,
    lsb: u32,
}

static RA: Field = Field {width: 3, lsb: 6};
static RB: Field = Field {width: 3, lsb: 3};
static RC: Field = Field {width: 3, lsb: 0};
static RL: Field = Field {width: 3, lsb: 25};
static VL: Field = Field {width: 25, lsb: 0};
static OP: Field = Field {width: 4, lsb: 28};
fn mask(bits: u32) -> u32 { (1 << bits) - 1 }
/// Given a `field` and `instruction`, extract
/// that field from the instruction as a u32
pub fn get(field: &Field, instruction: Umi) -> usize {
    ((instruction >> field.lsb) & mask(field.width)) as usize
}
/// Given an instruction word, extract the opcode
fn op(instruction: Umi) -> Option<Opcode> {
    FromPrimitive::from_u32((instruction >> OP.lsb) & mask(OP.width))
}

pub fn execute(mut memory: UM) {
    //let mut inst_ctr = 0;
    loop{
    // INVARIANT: inst_ptr is a valid index into memory
    // INVARIANT: memory[0] is the program
    let inst: Umi = memory.get(0, memory.inst_ptr);
    memory.inst_ptr += 1;
    match op(inst) {
        Some(Opcode::CMov) => {
            let ra = get(&RA, inst);
            let rb = get(&RB, inst);
            let rc = get(&RC, inst);
            regfuns::cmove(ra, rb, rc, &mut memory.registers)
        }
        Some(Opcode::Load) => {
            let ra = get(&RA, inst);
            let rb = get(&RB, inst);
            let rc = get(&RC, inst);
            iomemfuns::load(ra, rb, rc, &mut memory)
        }
        Some(Opcode::Store) => {
            let ra = get(&RA, inst);
            let rb = get(&RB, inst);
            let rc = get(&RC, inst);
            iomemfuns::store(ra, rb, rc, &mut memory)
        }
        Some(Opcode::Add) => {
            let ra = get(&RA, inst);
            let rb = get(&RB, inst);
            let rc = get(&RC, inst);
            regfuns::add(ra, rb, rc, &mut memory.registers);
        }
        Some(Opcode::Mult) => {
            let ra = get(&RA, inst);
            let rb = get(&RB, inst);
            let rc = get(&RC, inst);
            regfuns::mult(ra, rb, rc, &mut memory.registers);
        }
        Some(Opcode::Div) => {
            let ra = get(&RA, inst);
            let rb = get(&RB, inst);
            let rc = get(&RC, inst);
            regfuns::div(ra, rb, rc, &mut memory.registers);
        }
        Some(Opcode::Nand) => {
            let ra = get(&RA, inst);
            let rb = get(&RB, inst);
            let rc = get(&RC, inst);
            regfuns::nand(ra, rb, rc, &mut memory.registers);
        }
        Some(Opcode::Halt) => {
            //eprintln!("Instructions: {}", inst_ctr);
            std::process::exit(0);
        }
        Some(Opcode::MapSeg) => {
            let rb = get(&RB, inst);
            let rc = get(&RC, inst);
            iomemfuns::map(rb, rc, &mut memory)
        }
        Some(Opcode::UnmapSeg) => {
            let rc = get(&RC, inst);
            iomemfuns::unmap(rc, &mut memory)
        }
        Some(Opcode::Output) => {
            let rc = get(&RC, inst);
            iomemfuns::output(rc, &mut memory);
        }
        Some(Opcode::Input) => {
            let rc = get(&RC, inst);
            iomemfuns::input(rc, &mut memory.registers);
        }
        Some(Opcode::LoadProg) => {
            let rb = get(&RB, inst);
            let rc = get(&RC, inst);
            iomemfuns::loadprog(rb, rc, &mut memory);
        }
        Some(Opcode::LoadVal) => {
            let rl = get(&RL, inst);
            let vl = get(&VL, inst) as Umi;
            regfuns::loadval(rl, vl, &mut memory.registers);
        }
        None => {
            panic!("Invalid opcode")
        }
    }
    //inst_ctr += 1;
    }
}