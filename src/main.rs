mod tests;
mod const_pool;

struct MethodInfo {
    // The maximum operand stack size
    max_stack: u8,
    // The maximum number of local variables
    frame_size: u8,
}


#[allow(non_camel_case_types)]
#[derive(Debug)]
enum Opcode {
    iconst_0,         // push int const 0
    iload_1,          // push local variable 1
    istore_1,         // store into local variable 1
    goto(usize),      // goto
    iinc(usize, i32), // increment local variable
    bipush(i32),      // push int constant
    if_icmplt(usize), //
    r#return,
}

fn run(info: MethodInfo, codes: &[Opcode], locals: &mut [i32]) {
    println!("codes is {:?}, locals is {:?}", codes, locals);
    // let mut stack = vec![0; info.max_stack.into()];
    let mut stack = vec![];
    let mut pc = 0;

    loop {
        // println!("stack is {:?}, locals is {:?}", stack, locals);
        let code = match codes.get(pc) {
            Some(v) => v,
            None => break,
        };

        pc += 1;
        match code {
            Opcode::iconst_0 => stack.push(0),
            Opcode::istore_1 => locals[1] = stack.pop().unwrap(),
            Opcode::goto(index) => pc = *index,
            Opcode::iinc(index, number) => locals[*index] += number,
            Opcode::iload_1 => stack.push(locals[1]),
            Opcode::bipush(number) => stack.push(*number),
            Opcode::if_icmplt(index) => {
                let value2 = stack.pop();
                let value1 = stack.pop();
                if value1 < value2 {
                    pc = *index;
                    println!("pc is {:?}", pc);
                } else {
                    println!("another branch");
                }
            }
            Opcode::r#return => break,
        }
    }
}

fn main() {
    let info = MethodInfo {
        max_stack: 2,
        frame_size: 3,
    };
    let codes = [
        Opcode::iconst_0,     // 0
        Opcode::istore_1,     // 1
        Opcode::goto(4),      // 2
        Opcode::iinc(1, 1),   // 3
        Opcode::iload_1,      // 4
        Opcode::bipush(100),  // 5
        Opcode::if_icmplt(3), // 6
    ];
    let mut locals = vec![0; info.frame_size.into()];
    run(info, &codes, &mut locals);
}
