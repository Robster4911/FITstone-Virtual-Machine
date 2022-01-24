# FITstone-Virtual-Machine
 
[Problem Description](https://cs.fit.edu/~ryan/cse4250/projects/fit/)

### Quick Summary
The task for this project was to design a virtual machine interpreter for the "FITstone" assembly language. 
The assembly language for this assignment includes the following instructions:

```
Instr  length  opcode arguments
LOAD    6      0x00   R1  [immediate value; 4 bytes] # R1 := v
RLOAD   3      0x01   R1 R2      # R1 := R2
PUSH    2      0x02   R1         # push R1 into stack
POP     2      0x03   R1         # R1 gets top of stack; pop stack
ADD     4      0x04   R1 R2 R3   # R1 = R2 + R3
SUB     4      0x05   R1 R2 R3   # R1 = R2 - R3
MUL     4      0x06   R1 R2 R3   # R1 = R2 * R3
DIV     4      0x07   R1 R2 R3   # R1 = R2 / R3
JMP     5      0x08   [instr number; 4 bytes]   # goto l
CMP     3      0x09   R1 R2      # set condition code for future branch
BLT     5      0x0a   [instr number; 4 bytes]   # if < goto l
BEQ     5      0x0b   [instr number; 4 bytes]   # if = goto l
BGT     5      0x0c   [instr number; 4 bytes]   # if > goto l
BNE     5      0x0d   [instr number; 4 bytes]   # if not(=) goto l
```

### Project Requirements
- Must be written in Rust
- Must have eight 32 bit unsigned registers numbered R0-R7
- Must have a stack initialized to size 1000
- Must handle the following 3 errors:
  - Pushing to a full stack
  - Popping from an empty stack
  - Division by zero
- Machine will print the values of each register when it halts normally without error.

### Creating Test Cases
Input must be a compiled binary as opposed to ascii instructions. Because of this, and in order to be able to create my own test cases,
I wrote a short python script (compiler.py) that compiles the assembly code from ascii instructions into a bin file. Input for this script
is the name of the text file containing the assembly instructions. Upon completion, compiler.py will create a new file with the same name 
as the given text file, but in binary format and with the .bin extension.

**WARNING:** compiler.py does not handle syntax checking currently, so make sure your assembly code doesn't contain any typos or syntax errors.

### Running the Machine
(If you have not already, make sure to [install Rust](https://www.rust-lang.org/tools/install).)

First compile vm.rs to an executable with 
`rustc vm.rs`, then use file redirection to redirect the input stream to the desired .bin file.
`./vm.rs < testcase.bin`. 
