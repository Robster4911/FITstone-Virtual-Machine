/*
 * Author: Robert Heine, rheine2019@my.fit.edu
 * Course: CSE 4250, Spring 2022
 * Project: Project #1, FITstone Virtual Machine
 */

/*
 * This rust tutorial on tutorialspoint helped me
 * walk through the basics of the Rust language.
 * https://www.tutorialspoint.com/rust/index.htm
 *
 * Once I was familiar with how Rust worked, I
 * referenced the official Rust documentation for
 * more detailed explanations and functions that
 * Tutorialspoint did not have.
 * https://doc.rust-lang.org/std/index.html
 */

// imports
use std::io;
use std::io::Read;

/*
 * Stores information about a given instruction.
 * Capable of handling single byte parameters as
 * well as a 32 bit integer. 
 */
struct Instruction{
   // specifies instruction type
   opcode:u8,
   reg_args:[u8;3],
   arg_32:u32
}



/* 
 * Converts 4 bytes into a 32 bit integer.
 * Used for interpreting 32 bit input values.
 */
fn convert_to_32(big_arg:&mut u32, small_args:[u8;4]){
   let mut temp:u32;
   for x in 0..4{
      temp = small_args[x] as u32;
      *big_arg = *big_arg << 8;
      *big_arg += temp;
   }
}



/*
 * Reads instructions from stdin and pushes them
 * onto the instruction vector.
 */
fn read_instructions(instruction_vec:&mut Vec<Instruction>){
   let mut has_read_byte;
   let mut op:[u8;1] = [0;1];
   let mut go = true;

   // loop terminates when no more bytes are read
   while go {
      // reads in opcode
      has_read_byte = io::stdin().read(&mut op);
      
      // Checks result for err before calling unsafe function
      // This block should only execute if the program is
      // given invalid input.
      if has_read_byte.is_err() {
         go = false;
         println!("Err result on call read().");
         break;
      }
      
      // determines if there is an instruction to read
      if unsafe { has_read_byte.unwrap_unchecked() } == 0 {
         go = false;
      }
      // does not execute if there is nothing to read
      else{
         // create the temp struct
         let mut tmp_i = Instruction{
            opcode: op[0],
            reg_args: [0;3],
            arg_32: 0
         };
         // LOAD
         // args: R1, (4 byte int)
         if op[0] == 0 {
            // read in args
            let mut buff:[u8;4] = [0;4];
            let mut val_big:u32 = 0;
            let mut reg:[u8;1] = [0;1];
            io::stdin().read(&mut reg);
            io::stdin().read(&mut buff);
            convert_to_32(&mut val_big, buff);
            tmp_i.arg_32 = val_big;
            tmp_i.reg_args[0] = reg[0];
         }
         // RLOAD or CMP
         // args: R1, R2
         else if op[0] == 1 || op[0] == 9{
            let mut buff:[u8;2] = [0;2];
            io::stdin().read(&mut buff);
            tmp_i.reg_args[0] = buff[0];
            tmp_i.reg_args[1] = buff[1];
         }
         // PUSH or POP
         // args: R1
         else if op[0] == 2 || op[0] == 3{
            let mut buff:[u8;1] = [0;1];
            io::stdin().read(&mut buff);
            tmp_i.reg_args[0] = buff[0];
         }
         // ADD, SUB, MUL, or DIV
         // args: R1, R2, R3
         else if op[0] >=4 && op[0] <= 7{
            io::stdin().read(&mut tmp_i.reg_args);
         }
         // Everything else
         // args: (4 byte int)
         else {
            let mut buff:[u8;4] = [0;4];
            let mut val_big:u32 = 0;
            io::stdin().read(&mut buff);
            convert_to_32(&mut val_big, buff);
            tmp_i.arg_32 = val_big;
         }

         // push the instruction onto the vector
         instruction_vec.push(tmp_i);
      }
   }
}



/*
 * main
 */
fn main(){
   // registers
   let mut regs:[u32;8] = [0;8];
   
   // stack
   let mut stk:[u32;1000] = [0;1000];
   let mut top:usize = 0;
   
   /* condition code
    * -1 means less than, 
    * 0 means equals,
    * +1 means greater than.
    */
   let mut cc:i8 = 0;

   // Holds a list of all instructions in order.
   let mut instructions:Vec<Instruction> = Vec::new();
   read_instructions(&mut instructions);
   
   /* Begin execution, halting when there is an
    * error or the program has reached the end
    * of the instructions.
    */
   let mut is_error = false;
   let mut i_num:usize = 0;
   while i_num < instructions.len() {
      let mut no_jmp = true;
      let R1:usize = instructions[i_num].reg_args[0] as usize;
      let R2:usize = instructions[i_num].reg_args[1] as usize;
      let R3:usize = instructions[i_num].reg_args[2] as usize;
      let v:u32 = instructions[i_num].arg_32;
      
      // Determine which instruction to
      // execute based on the op code
      
      // LOAD
      if instructions[i_num].opcode == 0 {
         regs[R1] = v;
      }
      // RLOAD
      else if instructions[i_num].opcode == 1 {
         regs[R1] = regs[R2];
      }
      // PUSH
      else if instructions[i_num].opcode == 2 {
         // checks if stack is full
         if top >= 1000 {
            is_error = true;
            println!("ERROR:  full stack");
            break;
         }
         
         stk[top] = regs[R1];
         top += 1;
      }
      // POP
      else if instructions[i_num].opcode == 3 {
         // checks if stack is empty
         if top <= 0 {
            is_error = true;
            println!("ERROR:  empty stack");
            break;
         }
      
         top -= 1;
         regs[R1] = stk[top];
         stk[top] = 0;
      }
      // ADD
      else if instructions[i_num].opcode == 4 {
         // if register goes above u32 max value, it overflows back to 0
         regs[R1] = ((regs[R2] as u64 + regs[R3] as u64) % u32::MAX as u64) as u32;
      }
      // SUB
      else if instructions[i_num].opcode == 5 {
         // if result is going to be negative, underflow to u32 max
         if regs[R2] < regs[R3] {
            regs[R1] = ((u32::MAX as u64 + regs[R2] as u64) - regs[R3] as u64) as u32;
         }
         else{
            regs[R1] = regs[R2] - regs[R3];
         }
      }
      // MUL
      else if instructions[i_num].opcode == 6 {
         // handles overflow in the same way as ADD
         regs[R1] = ((regs[R2] as u64 * regs[R3] as u64) % u32::MAX as u64) as u32;
      }
      // DIV
      else if instructions[i_num].opcode == 7 {
         // checks to see if divides by zero
         if regs[R3] == 0 {
            is_error = true;
            println!("ERROR:  division by zero");
            break;
         }
         
         regs[R1] = regs[R2] / regs[R3];
      }
      // JMP
      else if instructions[i_num].opcode == 8 {
         no_jmp = false;
         i_num = v as usize;
      }
      // CMP
      else if instructions[i_num].opcode == 9 {
         if regs[R1] < regs[R2] {
            cc = -1;
         }
         else if regs[R1] == regs[R2] {
            cc = 0;
         }
         else {
            cc = 1;
         }
      }
      // BLT
      else if instructions[i_num].opcode == 10 {
         if cc < 0 {
            no_jmp = false;
            i_num = v as usize;
         }
      }
      // BEQ
      else if instructions[i_num].opcode == 11 {
         if cc == 0 {
            no_jmp = false;
            i_num = v as usize;
         }
      }
      // BGT
      else if instructions[i_num].opcode == 12 {
         if cc > 0 {
            no_jmp = false;
            i_num = v as usize;
         }
      }
      // BNE
      else if instructions[i_num].opcode == 13 {
         if cc != 0 {
            no_jmp = false;
            i_num = v as usize;
         }
      }
      
      // does not execute if jumped to another instruction
      if no_jmp{
         i_num += 1;
      }
   }
   
   // if an error was previously detected, does not print registers
   if is_error == false {
      println!("({}, {}, {}, {}, {}, {}, {}, {})", regs[0], regs[1], regs[2], regs[3], regs[4], regs[5], regs[6], regs[7]);
   }
}
