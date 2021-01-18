use std::fs;

use crate::call_stack::{Frame, JvmStack};
use crate::const_pool::ConstPool;
use crate::method_area::{Class, Method, MethodArea};
use crate::parser::Attribute;

mod tests;
mod const_pool;
mod call_stack;
mod method_area;
mod parser;

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

pub fn run(class_path: &str) {
    let mut file = fs::File::open(class_path).unwrap();
    let class_file = parser::parse(&mut file);
    let const_pool = class_file.const_pool;
    let mut method_area = MethodArea::new();
    let class_name = &class_file.this_class;
    for info in class_file.methods {
        let mut method: Option<Method> = None;
        for attribute in info.attributes {
            match attribute {
                Attribute::Code { max_stack, max_locals, codes } => {
                    method = Some(Method::new(max_stack as usize, max_locals as usize, codes, class_name))
                }
                _ => {}
            }
        }
        let method = method.unwrap();
        let name = const_pool.resolve_utf8(info.name_index);
        let descriptor = const_pool.resolve_utf8(info.descriptor_index);
        let key = String::from(&class_file.this_class) + "." + name + ":" + descriptor;
        method_area.put(Box::leak(key.into_boxed_str()), method);
    }
    method_area.put_class(&class_file.this_class, Class {
        super_class: &None,
        const_pool,
        methods: Default::default(),
    });
    let main_method = method_area.resolve_method(&*(
        String::from(&class_file.this_class)
            + "."
            + "main:([Ljava/lang/String;)V"
    ));
    dbg!(&method_area);
    let const_pool = &method_area.resolve_class(class_name).const_pool;
    let mut local = Vec::with_capacity(main_method.local_size);
    local.resize(main_method.local_size, 0);
    let main_frame = Frame::new(local, main_method, const_pool);
    let mut jvm_stack = JvmStack::new(256, main_frame, &method_area);
    jvm_stack.run()
}
