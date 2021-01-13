use std::collections::HashMap;
use std::convert::TryInto;
use std::ops::Add;

use crate::call_stack::{Class, Method};
use crate::const_pool::CpInfo::Utf8;

/// const pool table entry
pub enum CpInfo<'a> {
    // empty placeholder
    Placeholder,
    Class { name_index: u16 },
    FieldRef { class_index: u16, name_and_type_index: u16 },
    MethodRef { class_index: u16, name_and_type_index: u16 },
    InterfaceMethodRef { class_index: u16, name_and_type_index: u16 },
    String { string_index: u16 },
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    NameAndTuple { name_index: u16, descriptor_index: u16 },
    Utf8(&'a str),
    MethodHandle,
    MethodType,
    Dynamic,
    InvokeDynamic,
    Module,
    Package,
}

pub(crate) struct ConstPool<'a> {
    value: &'a [CpInfo<'a>],
    cache: HashMap<u16, CpInfo<'a>>,
    // method_table: &'a MethodTable<'a>,
}

impl ConstPool<'_> {
    fn resolve_utf8(&self, index: u16) -> &str {
        return match self.value.get(index as usize).unwrap() {
            CpInfo::Utf8(v) => v,
            CpInfo::Class { name_index } => self.resolve_utf8(*name_index),
            CpInfo::NameAndTuple { name_index, descriptor_index } => Box::leak(format!(
                "{}:{}",// https://stackoverflow.com/a/51286293/6627776
                self.resolve_utf8(*name_index), self.resolve_utf8(*descriptor_index),
            ).into_boxed_str()),
            CpInfo::MethodRef { class_index, name_and_type_index } => Box::leak(format!(
                "{}.{}",
                self.resolve_utf8(*class_index), self.resolve_utf8(*name_and_type_index),
            ).into_boxed_str()),
            _ => panic!("not supported")
        };
    }

    // fixme
    #[allow(clippy::needless_lifetimes)]
    fn new<'a>(data: &'a [CpInfo]) -> ConstPool<'a> {
        return ConstPool {
            value: data,
            cache: Default::default(),
        };
    }
}

// 12.3.2
struct MethodTable<'a> {
    value: HashMap<&'a str, Method<'a>>
}

impl<'a> MethodTable<'a> {
    fn add(&mut self, key: &'a str, method: Method<'a>) {
        self.value.insert(key, method);
    }

    fn new() -> MethodTable<'static> {
        return MethodTable {
            value: HashMap::new()
        };
    }
}

pub(crate) struct ClassTable<'a> {
    value: HashMap<&'a str, Class<'a>>
}

impl<'a> ClassTable<'a> {
    pub(crate) fn add(&mut self, key: &'a str, class: Class<'a>) {
        self.value.insert(key, class);
    }

    pub(crate) fn new() -> ClassTable<'static> {
        return ClassTable {
            value: HashMap::new()
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::call_stack::Method;
    use crate::const_pool::{ConstPool, CpInfo, MethodTable};
    use crate::Opcode;

    const CONST_POOL_SAMPLE: [CpInfo; 18] = [
        CpInfo::Placeholder,
        CpInfo::MethodRef { class_index: 4, name_and_type_index: 14 },
        CpInfo::MethodRef { class_index: 3, name_and_type_index: 15 },
        CpInfo::Class { name_index: 16 },
        CpInfo::Class { name_index: 17 },
        CpInfo::Utf8("<init>"),
        CpInfo::Utf8("()V"),
        CpInfo::Utf8("Code"),
        CpInfo::Utf8("LineNumberTable"),
        CpInfo::Utf8("add"),
        CpInfo::Utf8("(II)I"),
        CpInfo::Utf8("main"),
        CpInfo::Utf8("SourceFile"),
        CpInfo::Utf8("Adder.java"),
        CpInfo::NameAndTuple { name_index: 5, descriptor_index: 6 },
        CpInfo::NameAndTuple { name_index: 9, descriptor_index: 10 },
        CpInfo::Utf8("Adder"),
        CpInfo::Utf8("java/lang/Object")
    ];


    #[test]
    fn resolve_utf8_from_const_pool() {
        let const_pool = ConstPool::new(&CONST_POOL_SAMPLE);
        assert_eq!(r#"java/lang/Object.<init>:()V"#, const_pool.resolve_utf8(1));
        assert_eq!(r#"Adder.add:(II)I"#, const_pool.resolve_utf8(2));
    }

    #[test]
    fn say_hi() {
        let mut method_table = MethodTable::new();
        method_table.add(r#"java/lang/Object."<init>":()V"#, Method {
            codes: &[],
        });
        method_table.add(r#"Adder.add:(II)I"#, Method {
            codes: &[
                Opcode::iload_0,
                Opcode::iload_1,
                Opcode::iadd,
                Opcode::ireturn
            ]
        });
    }
}



