
// This file contains all operations relating to register manipulation
use crate::exec::Umi;

/// A function that performs the conidional move operation
/// 
/// # Arguments
/// 
/// * `ra` - The index of the register to store the result in
/// * `rb` - The index of the register to check
/// * `rc` - The index of the register to move
/// * `registers` - The array of registers
pub fn cmove (ra: usize, rb: usize, rc: usize, registers: &mut [Umi; 8]) {
    if registers[rc] != 0 {
        registers[ra] = registers[rb];
    }
}

/// A function that performs the load value operation
/// 
/// # Arguments
/// 
/// * `rl` - The index of the register to store the result in
/// * `value` - The value to load
/// * `registers` - The array of registers
pub fn loadval (rl: usize, value: Umi, registers: &mut [Umi; 8]) {
    registers[rl] = value;
}

/// A function that performs the bitwise NAND operation
/// 
/// # Arguments
/// 
/// * `ra` - The index of the register to store the result in
/// * `rb` - The index of the register to NAND
/// * `rc` - The index of the register to NAND
/// * `registers` - The array of registers
pub fn nand (ra: usize, rb: usize, rc: usize, registers: &mut [Umi; 8]) {
    registers[ra] = !(registers[rb] & registers[rc]);
}

/// A function that performs the addition operation
/// 
/// # Arguments
/// 
/// * `ra` - The index of the register to store the result in
/// * `rb` - The index of the register to add
/// * `rc` - The index of the register to add
/// * `registers` - The array of registers

pub fn add (ra: usize, rb: usize, rc: usize, registers: &mut [Umi; 8]) {
    //registers[ra] = ((registers[rb] as u64 + registers[rc] as u64) % (1_u64 << 32)) as u32;

    registers[ra] = (((registers[rb] as u64).wrapping_add(registers[rc] as u64)) % (1_u64 << 32)) as u32;
}

/// A function that performs the multiplication operation
/// 
/// # Arguments
/// 
/// * `ra` - The index of the register to store the result in
/// * `rb` - The index of the register to multiply
/// * `rc` - The index of the register to multiply
/// * `registers` - The array of registers
pub fn mult (ra: usize, rb: usize, rc: usize, registers: &mut [Umi; 8]) {
    //registers[ra] = ((registers[rb] as u64 * registers[rc] as u64) % (1_u64 << 32)) as u32;


    registers[ra] = (((registers[rb] as u64).wrapping_mul(registers[rc] as u64)) % (1_u64 << 32)) as u32;
}

/// A function that performs the division operation
/// 
/// # Arguments
/// 
/// * `ra` - The index of the register to store the result in
/// * `rb` - The index of the register to divide
/// * `rc` - The index of the register to divide
/// * `registers` - The array of registers
pub fn div (ra: usize, rb: usize, rc: usize, registers: &mut [Umi; 8]) {
    registers[ra] = registers[rb] / registers[rc];
}
