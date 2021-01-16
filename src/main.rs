mod tests;
mod const_pool;
mod class_file;
mod class_parser;
mod call_stack;
mod method_area;

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum Opcode {
    // push int const 0
    iload_0,
    iconst_0,
    iconst_1,
    iload_1,
    istore_0,
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
    invokestatic(usize)
}


fn main() {
    // todo
}
