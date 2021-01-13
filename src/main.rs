use std::io::Read;

mod tests;
mod const_pool;
mod class_file;
mod class_parser;
mod call_stack;
mod method_area;

struct MethodInfo {
    // The maximum operand stack size
    max_stack: u8,
    // The maximum number of local variables
    frame_size: u8,
}


#[allow(non_camel_case_types)]
#[derive(Debug)]
enum Opcode {
    iload_0,
    // push int const 0
    iconst_0,
    iconst_1,
    iload_1,
    istore_0,
    // push local variable 1
    istore_1,
    // store into local variable 1
    iadd,
    ireturn,
    goto(usize),
    // goto
    iinc(usize, i32),
    // increment local variable
    bipush(i32),
    // push int constant
    if_icmplt(usize),
    //
    r#return,
    invokestatic(u16)
}

// fn run(info: MethodInfo, codes: &[Opcode], locals: &mut [i32]) {
//     println!("codes is {:?}, locals is {:?}", codes, locals);
//     // let mut stack = vec![0; info.max_stack.into()];
//     let mut stack: Vec<i32> = vec![];
//     let mut pc = 0;
//
//     loop {
//         // println!("stack is {:?}, locals is {:?}", stack, locals);
//         let code = match codes.get(pc) {
//             Some(v) => v,
//             None => break,
//         };
//
//         pc += 1;
//         match code {
//             Opcode::iconst_0 => stack.push(0),
//             Opcode::istore_1 => locals[1] = stack.pop().unwrap(),
//             Opcode::goto(index) => pc = *index,
//             Opcode::iinc(index, number) => locals[*index] += number,
//             Opcode::iload_1 => stack.push(locals[1]),
//             Opcode::bipush(number) => stack.push(*number),
//             Opcode::if_icmplt(index) => {
//                 let value2 = stack.pop();
//                 let value1 = stack.pop();
//                 if value1 < value2 {
//                     pc = *index;
//                     println!("pc is {:?}", pc);
//                 } else {
//                     println!("another branch");
//                 }
//             }
//             Opcode::r#return => break,
//         }
//     }
// }

fn main() {
    // let handle = std::fs::open("C:\\Users\\yujinyan\\code\\study\\java\\Adder.class");
    // let handle = std::fs::read("C:\\Users\\yujinyan\\code\\study\\java\\Adder.class").unwrap();;
    // let iter = handle.iter();
    // iter.take(4).
    // handle.
}
