use crate::*;
use std::collections::VecDeque;
use std::slice::Iter;
use crate::call_stack::Class;
use crate::const_pool::ConstPool;

#[test]
fn simple_loop() {
    ///
    /// ```java
    /// int i;
    /// for (i = 0; i < 100; i++) {
    ///     ;
    /// }
    /// ```
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
    // run(info, &codes, &mut locals);
    assert_eq!(1 + 1, 2)
}

#[test]
fn adder() {
    let mut class_table = const_pool::ClassTable::new();
    // class_table.add("Adder", Class {
    //     super_class: None,
    //     const_pool: ConstPool {
    //         value: [],
    //         cache: Default::default(),
    //         method_table: &()
    //     },
    //     methods: Default::default()
    // })
}


