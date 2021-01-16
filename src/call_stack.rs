use std::collections::HashMap;

use crate::const_pool::ConstPool;
use crate::method_area::{Method, MethodArea, Class};
use crate::Opcode;

// §2.5.2
struct JvmStack<'a> {
    frames: Vec<Frame<'a>>,
    method_table: &'a MethodArea<'a>,
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
    method: &'a Method<'a>,
}

#[derive(Debug, PartialEq)]
enum FrameResult<'a> {
    End,
    ReturnValue(i32),
    Invoke(&'a str),
}

impl JvmStack<'_> {
    fn new<'a>(
        max_size: usize,
        main: Frame<'a>,
        method_table: &'a MethodArea<'a>,
    ) -> JvmStack<'a> {
        let mut stack = Vec::with_capacity(max_size);
        stack.push(main);

        JvmStack { frames: stack, method_table }
    }

    fn run(&mut self) {
        while !self.frames.is_empty() {
            let mut frame = self.frames.pop().unwrap();
            match frame.run() {
                FrameResult::End => {}
                FrameResult::ReturnValue(v) => {
                    // last() does not gain ownership
                    let mut invoker = self.frames.pop().unwrap();
                    invoker.push(v);
                    self.frames.push(invoker);
                }
                FrameResult::Invoke(method_name) => {
                    let method: &Method = self.method_table.resolve_method(method_name);
                    let class: &Class = self.method_table.resolve_class(method.class);
                    let mut locals = Vec::with_capacity(method.local_size);

                    for i in 0..method.local_size {
                        let arg = frame.pop();
                        locals.push(arg);
                    }

                    let invoked = Frame::new(locals, method, &class.const_pool);
                    self.frames.push(frame);
                    self.frames.push(invoked);
                }
            }
        }
    }
}

impl Frame<'_> {
    fn new<'a>(locals: Vec<i32>,
               method: &'a Method<'a>,
               constant_pool: &'a ConstPool,
    ) -> Frame<'a> {
        Frame {
            pc: 0,
            locals,
            operand_stack: Vec::with_capacity(method.stack_size),
            constant_pool,
            method,
        }
    }
    fn push(&mut self, value: i32) {
        self.operand_stack.push(value);
    }

    fn pop(&mut self) -> i32 {
        self.operand_stack.pop().unwrap()
    }

    fn run(&mut self) -> FrameResult {
        let mut stack: &mut Vec<i32> = &mut self.operand_stack;

        loop {
            // println!("stack is {:?}, locals is {:?}", stack, locals);
            let code = match self.method.codes.get(self.pc) {
                Some(v) => v,
                None => break,
            };

            dbg!(code);

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
                Opcode::invokestatic(index) => {
                    let method = self.constant_pool.resolve_utf8(*index as u16);
                    return FrameResult::Invoke(method);
                }
                Opcode::iload_0 => stack.push(self.locals[0]),
                Opcode::iadd => {
                    let result = stack.pop().unwrap() + stack.pop().unwrap();
                    stack.push(result);
                }
                Opcode::ireturn => return FrameResult::ReturnValue(stack.pop().unwrap()),
                _ => unimplemented!("opcode {:?}", code)
            }
        }
        FrameResult::End
    }
}

#[cfg(test)]
mod tests {
    use crate::call_stack::{Frame, FrameResult, JvmStack};
    use crate::const_pool::ConstPool;
    use crate::const_pool::tests::{CONST_POOL_SAMPLE, test_const_pool};
    use crate::method_area::{Method, Class, MethodArea};
    use crate::Opcode;
    use crate::const_pool::CpInfo::Class as CpClass;

    #[test]
    fn create_new_stack() {
        let method_table = MethodArea::new();
        let const_pool = &ConstPool::new(&CONST_POOL_SAMPLE);
        let frame = Frame::new(Vec::new(), &Method {
            codes: &[],
            stack_size: 2,
            local_size: 0,
            class: "",
        }, const_pool);
        let stack = JvmStack::new(128, frame, &method_table);
    }

    #[test]
    fn run_single_frame() {
        let const_pool = &test_const_pool();
        let mut frame = Frame::new(
            vec![0; 3],
            &Method {
                stack_size: 3,
                local_size: 0,
                codes: &[
                    Opcode::iconst_0,     // 0
                    Opcode::istore_1,     // 1
                    Opcode::goto(4),      // 2
                    Opcode::iinc(1, 1),   // 3
                    Opcode::iload_1,      // 4
                    Opcode::bipush(10),   // 5
                    Opcode::if_icmplt(3), // 6
                ],
                class: "",
            },
            const_pool,
        );

        assert_eq!(frame.run(), FrameResult::End);
    }

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
    #[test]
    fn run_adder() {
        let const_pool = &test_const_pool();
        let main_method = &Method {
            stack_size: 2,
            local_size: 1,
            codes: &[
                Opcode::iconst_1,     // 0
                Opcode::iconst_1,
                Opcode::invokestatic(2),
                Opcode::istore_0,
                Opcode::r#return
            ],
            class: "Adder",
        };
        let add_method = Method {
            stack_size: 2,
            local_size: 2,
            codes: &[
                Opcode::iload_0,
                Opcode::iload_1,
                Opcode::iadd,
                Opcode::ireturn,
            ],
            class: "Adder",
        };

        let mut main_frame = Frame::new(vec![0; 1], main_method, const_pool);
        let mut method_area = MethodArea::new();
        method_area.put("Adder.add:(II)I", add_method);
        method_area.put_class("Adder", crate::method_area::Class {
            super_class: &None,
            const_pool: ConstPool::new(&CONST_POOL_SAMPLE),
            methods: Default::default(),
        });
        let mut jvm_stack = JvmStack::new(256, main_frame, &method_area);
        jvm_stack.run();
    }
}