use crate::*;
use std::collections::VecDeque;
use std::slice::Iter;

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
    run(info, &codes, &mut locals);
    assert_eq!(1 + 1, 2)
}

#[test]
fn play() {
    let a = vec![1, 2, 3];
    println!("numbers is {}", add(&a));
    println!("a is {:?}", a)
}

fn add(numbers: &Vec<i32>) -> i32 {
    return numbers.iter().fold(0, |x, y| x + y);
}

fn add2(numbers: &Vec<i32>) -> i32 {
    return numbers.iter().sum()
}

trait Summable<T> {
    fn sum(self) -> i32;
}
impl<T> Summable<T> for Iter<'_, T> {
    fn sum(self) -> i32 {
        self.fold(0, |x, y| x + y)
    }
}
