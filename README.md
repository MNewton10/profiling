# Profiling
Universal Machine Profiling Assignment Fall 2023
This project is an updated version of the original implementation of the Universal Machine. In this implementation, the sole focus of the changes made to the code were focused on optimizing speed. Code was altered with the knowledge and assumption that only valid input would be provided, and only valid data. Therefore things such as unsafe indexing could be performed.
## Organization
All of the programs are within rum/src. main.rs is the host of the original function call into the main file of the execution, exec.rs. This file contains the struct representing the universal machine UM, as well as the key to opcode enumerations that was used in this project. It also holds static bitshifting keys for grabbing various fields from the bitwords, and functions to fetch the data properly. However, this code is all generally support code. The main function within exec.rs, which is called by main.rs, is the function execute. This function loops through instruction files through a match statement that pairs with the enumeration of opcodes that is also in the file. Each opcode initializes the proper fields for the given instruction, and depending on the opcode, a function will be called from either regfuns.rs or iomemfuns.rs. The pointer to the current instruction is incremented or changed properly based on the instruction provided, and the program will terminate when a "Halt" opcode is provided.
### Regfuns
This file contains all of the functions concerning direct operations with memory registers. The functions involved are Conditional Move, Load Value, Bitwise NAND, Addition, Multiplication, and Division. These functions are only focused on direct operations with memory registers, not with the entire memory structure.
### IOMemFuns
This file contains all of the functions concerning operations with the memory structure and operations for input or output. These functions include Segmented Load, Segmented Store, Map Segment, Unmap Segment, Load Program, Input, and Output. All functions besides input and output involve directly manipulating segments of memory rather than individual registers within memory.
### Rumload
This code was provided by the professor as a method of properly loading the instructions from the .um or .umz files as a vector of u32s that are read throughout the program.
### Midmark.um and Sandmark.umz
These are two provided programs for the implementation that contain instructions for the universal machine to perform. Midmark is significantly smaller than sandmark, but both were tested and benchmarked for this implementation.
## Performance
On my local machine, midmark was able to fully complete 8.51 x 10^7 instructions in an average time of 0.341 seconds, which is the amount of instructions contained in midmark.um. It was able to complete 2.11 x 10^9 instructions in an average of 8.376 seconds, which is the amount of instructions in sandmark.umz.
## Compiling, Building, and Running the Code
For these instructions, the assumed directory is **profiling/rum/src**
**cargo build --release** will work properly
However, if you want to build specifically to see some of the changes provided, you can run **cargo build --profile profiling**
Once this has been completed, the program can be run with either of the following:
**cargo run ../midmark.um**
**cargo run ../sandmark.umz**
Depending on which file you want to see the output of.
