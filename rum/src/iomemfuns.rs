// This file contains all functions relating to memory and IO operations
use std::io::{Read};
use crate::exec::{Umi, UM};


/// A function that performs the segmented load operation
/// 
/// # Arguments
/// * `ra` - The index of the register to store the result in
/// * `rb` - The index of the register to load from
/// * `rc` - The value to offset by
/// * `memory` - The UM instance to load into
pub fn load (ra: usize, rb: usize, rc: usize, memory: &mut UM) {
    memory.registers[ra] = memory.get(memory.registers[rb], memory.registers[rc]);
}

/// A function that performs the segmented store operation
/// 
/// # Arguments
/// * `ra` - The index of the register to store the result in
/// * `rb` - The index of the register to load from
/// * `rc` - The value to offset by
/// * `memory` - The UM instance to store to
pub fn store (ra: usize, rb: usize, rc: usize, memory: &mut UM) {
    memory.set(memory.registers[ra], memory.registers[rb], memory.registers[rc]);
}



/// A function that performs the Map segment operation
/// 
/// # Arguments
/// 
/// * 'rc' - the value that is the size of a new created segment
/// * 'registers' - Array of registers
/// * 'memory' - Instance of memory to map
pub fn map (rb: usize, rc: usize, memory: &mut UM){
    let val = memory.registers[rc];
    if memory.free.len() == 0 {
        // INVARIANT: If free is empty, then the new segment is added to the end of the memory vector
        memory.memvec.push(vec![0; val as usize]);
        memory.registers[rb] = (memory.memvec.len() - 1) as Umi;
    } else {
        // INVARIANT: If free is not empty, then the new segment is added to the vector at the value popped from the collection of free indices
        let index = memory.free.pop().unwrap();
        memory.memvec[index as usize].resize(val as usize, 0);
        memory.registers[rb] = index;
    }
}

/// A function that performs the Unmap segment operation
/// 
/// # Arguments
/// 
/// * `rc` - The index of the register to unmap
/// * `memory` - The instance of memory to unmap from
pub fn unmap (rc: usize, memory: &mut UM) {
    memory.memvec[memory.registers[rc] as usize] = Vec::new();
    memory.free.push(memory.registers[rc] as Umi);
}

/// A function that performs the output operation
/// 
/// # Arguments
/// 
/// * `rc` - The index of the register to output
/// * `registers` - The array of registers
/// * `output` - The output buffer
pub fn output (rc: usize, memory: &mut UM) {
    print!("{}", char::from_u32(memory.registers[rc]).unwrap());
}

/// A function that performs the input operation
/// 
/// # Arguments
/// 
/// * `rc` - The index of the register to input
/// * `registers` - The array of registers
pub fn input(rc: usize, registers: &mut [Umi; 8]) {
    let input = std::io::stdin().bytes().next().unwrap();
    match input {
        Ok(byte) => {
            registers[rc] = byte.try_into().unwrap();
        } 
        Err(_) => {
            registers[rc] = u32::MAX;
        }
    }
}


/// A function that performs the load program operation
/// 
/// # Arguments
/// 
/// * `rb` - The index of the register to load from
/// * `rc` - The value to offset by
/// * `registers` - The array of registers
/// * `memory` - The instance of memory to load from
pub fn loadprog (rb: usize, rc: usize, memory: &mut UM) {
    let index = memory.registers[rb] as usize;
    let rb = memory.registers[rb];
    let rc = memory.registers[rc];
    if rb != 0 {
        memory.memvec[0] = memory.memvec[index].clone();
    }
    memory.inst_ptr = rc;
}