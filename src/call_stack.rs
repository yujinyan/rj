use std::collections::HashMap;

use crate::method_area::Method;
use crate::Opcode;
use crate::const_pool::ConstPool;

// 2.5.2
struct JvmStack<'a> {
    frames: Vec<Frame<'a>>
}

// §2.6
struct Frame<'a> {
    pc: usize,
    // §2.6.1
    locals: Vec<i32>,
    // §2.6.2
    operand_stack: Vec<i32>,
    // §2.5.5
    constant_pool: &'a ConstPool<'a>,
    method: Method<'a>,
}

#[derive(Debug, PartialEq)]
enum FrameResult {
    End,
    ReturnValue(i32),
    Invoke,
}

impl JvmStack<'_> {
    fn new(max_size: usize, main: Frame) -> JvmStack {
        let mut stack = Vec::with_capacity(max_size);
        stack.push(main);

        JvmStack { frames: stack }
    }

    fn run(&mut self) {
        while !self.frames.is_empty() {
            let mut frame = self.frames.pop().unwrap();
            match frame.run() {
                FrameResult::End => {}
                FrameResult::ReturnValue(_) => {}
                FrameResult::Invoke => {}
            }
        }
    }
}

impl Frame<'_> {
    fn new<'a>(locals: Vec<i32>,
               stack_size: usize,
               method: Method<'a>,
               constant_pool: &'a ConstPool,
    ) -> Frame<'a> {
        Frame {
            pc: 0,
            locals,
            operand_stack: Vec::with_capacity(stack_size),
            constant_pool,
            method,
        }
    }
    fn run(&mut self) -> FrameResult {
        let mut stack: &mut Vec<i32> = &mut self.operand_stack;

        loop {
            // println!("stack is {:?}, locals is {:?}", stack, locals);
            let code = match self.method.codes.get(self.pc) {
                Some(v) => v,
                None => break,
            };

            self.pc += 1;
            match code {
                Opcode::iconst_0 => stack.push(0),
                Opcode::iconst_1 => stack.push(1),
                Opcode::istore_0 => self.locals[0] = stack.pop().unwrap(),
                Opcode::istore_1 => self.locals[1] = stack.pop().unwrap(),
                Opcode::goto(index) => self.pc = *index,
                Opcode::iinc(index, number) => self.locals[*index] += number,
                Opcode::iload_1 => stack.push(self.locals[1]),
                Opcode::bipush(number) => stack.push(*number),
                Opcode::if_icmplt(index) => {
                    let value2 = stack.pop();
                    let value1 = stack.pop();
                    if value1 < value2 {
                        self.pc = *index;
                    }
                }
                Opcode::r#return => break,
                Opcode::invokestatic(_) => {}
                Opcode::iload_0 => stack.push(self.locals[0]),
                Opcode::iadd => {
                    let result = stack.pop().unwrap() + stack.pop().unwrap();
                    stack.push(result);
                }
                Opcode::ireturn => return FrameResult::ReturnValue(stack.pop().unwrap())
            }
        }
        FrameResult::End
    }
}

#[cfg(test)]
mod tests {
    use crate::call_stack::{Frame, FrameResult};
    use crate::method_area::Method;
    use crate::Opcode;
    use crate::const_pool::ConstPool;
    use crate::const_pool::tests::CONST_POOL_SAMPLE;

    #[test]
    fn run_single_frame() {
        ///```java
        /// public class Adder {
        ///    public static int add(int x, int y) {
        ///        return x + y;
        ///    }
        ///
        ///    public static void main() {
        ///        int s = add(1, 1);
        ///    }
        ///}
        /// ```
        // let empty_const_pool_entries = [];
        // let empty_const_pool = &ConstPool::new(&empty_const_pool_entries);
        let const_pool = &ConstPool::new(&CONST_POOL_SAMPLE);
        let mut frame = Frame::new(
            vec![0; 3],
            3,
            Method {
                codes: &[
                    Opcode::iconst_0,     // 0
                    Opcode::istore_1,     // 1
                    Opcode::goto(4),      // 2
                    Opcode::iinc(1, 1),   // 3
                    Opcode::iload_1,      // 4
                    Opcode::bipush(100),  // 5
                    Opcode::if_icmplt(3), // 6
                ],
            },
            const_pool,
        );

        assert_eq!(frame.run(), FrameResult::End);
    }
}