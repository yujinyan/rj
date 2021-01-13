use crate::Opcode;
use std::collections::HashMap;

// §2.6
struct Frame<'a> {
    pc: usize,
    // §2.6.1
    locals: &'a mut [i32],
    // §2.6.2
    operand_stack: Vec<i32>,
    // §2.5.5
    constant_pool: i32,
    method: Method<'a>,
}

struct Stack {}

pub(crate) struct Method<'a> {
    pub codes: &'a [Opcode]
}

pub struct Class<'a> {
    super_class: &'a Option<Class<'a>>,
    const_pool: crate::const_pool::ConstPool<'a>,
    methods: HashMap<&'a str, Method<'a>>,
}

enum FrameResult {
    End,
    Invoke,
}

impl Frame<'_> {
    // fn new() -> Frame {}
    fn run(&mut self) {
        let mut stack: Vec<i32> = vec![];
        // let mut pc = 0;

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
                Opcode::ireturn => break
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::call_stack::{Frame, Method};
    use crate::Opcode;

    // fn new(locals: Vec<i32>, stack_size: usize, method: Method) -> Frame {
    //     Frame {
    //         pc: 0,
    //         locals,
    //         operand_stack: Vec::with_capacity(stack_size),
    //         constant_pool: 0,
    //         method,
    //     }
    // }


    #[test]
    fn can_run_single_frame() {
        let mut f = Frame {
            pc: 0,
            locals: &mut [0; 3],
            operand_stack: vec![0; 3],
            constant_pool: 0,
            method: Method {
                codes: &[
                    Opcode::iconst_0,     // 0
                    Opcode::istore_1,     // 1
                    Opcode::goto(4),      // 2
                    Opcode::iinc(1, 1),   // 3
                    Opcode::iload_1,      // 4
                    Opcode::bipush(100),  // 5
                    Opcode::if_icmplt(3), // 6
                ]
            },
        };
        f.run()
    }

    #[test]
    fn simple_function_call() {
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
        let mut main_frame = Frame {
            pc: 0,
            locals: &mut [0; 1],
            operand_stack: vec![0; 2],
            constant_pool: 0,
            method: Method {
                codes: &[
                    Opcode::iconst_1,
                    Opcode::iconst_1,
                    Opcode::invokestatic(2),
                    Opcode::istore_0,
                    Opcode::r#return
                ]
            },
        };
    }
}