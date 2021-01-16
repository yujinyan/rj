mod tests;
mod const_pool;
mod class_file;
mod class_parser;
mod call_stack;
mod method_area;

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum Opcode {
    aload_0,
    iconst_0,
    iconst_1,
    iload_0,
    iload_1,
    istore_0,
    istore_1,
    iadd,
    ireturn,
    goto(usize),
    iinc(usize, i32),
    bipush(i32),
    if_icmplt(usize),
    r#return,
    invokestatic(usize),
    invokespecial(u16),
}


fn main() {
    // todo
}
