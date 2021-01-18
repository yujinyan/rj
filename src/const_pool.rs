use std::collections::HashMap;
use std::convert::TryInto;
use std::ops::Add;

use crate::const_pool::CpInfo::Utf8;
use crate::method_area::{Class, Method};

/// const pool table entry
#[derive(Debug)]
pub enum CpInfo {
    /// Const pool items start with index 1.
    /// We insert an empty Placeholder at the head.
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
    Utf8(String),
    MethodHandle,
    MethodType,
    Dynamic,
    InvokeDynamic,
    Module,
    Package,
}

// todo: convert vec to array?
#[derive(Debug)]
pub(crate) struct ConstPool {
    value: Vec<CpInfo>,
    cache: HashMap<u16, CpInfo>,
}

impl ConstPool {
    pub(crate) fn resolve_utf8(&self, index: u16) -> &str {
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

    pub(crate) fn from_vec(
        data: Vec<CpInfo>
    ) -> ConstPool {
        return ConstPool {
            value: data,
            cache: Default::default(),
        };
    }
}


#[cfg(test)]
pub mod tests {
    use crate::const_pool::{ConstPool, CpInfo};
    use crate::method_area::Method;
    use crate::Opcode;

    pub(crate) fn sample_const_pool() -> ConstPool {
        let items = vec![
            CpInfo::Placeholder,
            CpInfo::MethodRef { class_index: 4, name_and_type_index: 14 },
            CpInfo::MethodRef { class_index: 3, name_and_type_index: 15 },
            CpInfo::Class { name_index: 16 },
            CpInfo::Class { name_index: 17 },
            CpInfo::Utf8("<init>".to_string()),
            CpInfo::Utf8("()V".to_string()),
            CpInfo::Utf8("Code".to_string()),
            CpInfo::Utf8("LineNumberTable".to_string()),
            CpInfo::Utf8("add".to_string()),
            CpInfo::Utf8("(II)I".to_string()),
            CpInfo::Utf8("main".to_string()),
            CpInfo::Utf8("SourceFile".to_string()),
            CpInfo::Utf8("Adder.java".to_string()),
            CpInfo::NameAndTuple { name_index: 5, descriptor_index: 6 },
            CpInfo::NameAndTuple { name_index: 9, descriptor_index: 10 },
            CpInfo::Utf8("Adder".to_string()),
            CpInfo::Utf8("java/lang/Object".to_string())
        ];
        ConstPool::from_vec(items)
    }
}



