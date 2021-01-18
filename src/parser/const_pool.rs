use crate::parser::Reader;
use crate::const_pool::{ConstPool, CpInfo};

pub(crate) fn read_const_pool<'a>(reader: &mut Reader) -> ConstPool {
    let const_pool_count = reader.read_u16();
    let mut parsed_items = Vec::with_capacity(const_pool_count as usize);

    parsed_items.push(CpInfo::Placeholder);
    for _ in 0..const_pool_count - 1 {
        let tag = reader.read_u8();
        let info: CpInfo = match tag {
            7 => CpInfo::Class { name_index: reader.read_u16() },
            9 => CpInfo::FieldRef { class_index: 0, name_and_type_index: 0 },
            10 => CpInfo::MethodRef { class_index: reader.read_u16(), name_and_type_index: reader.read_u16() },
            11 => CpInfo::InterfaceMethodRef { class_index: 0, name_and_type_index: 0 },
            8 => CpInfo::String { string_index: 0 },
            3 => CpInfo::Integer(0),
            4 => CpInfo::Float(0.0),
            5 => CpInfo::Long(0),
            6 => CpInfo::Double(0.0),
            12 => CpInfo::NameAndTuple { name_index: reader.read_u16(), descriptor_index: reader.read_u16() },
            1 => {
                let length = reader.read_u16();
                let string = reader.read_utf8(length);
                // let string = Box::leak(string.into_boxed_str());
                CpInfo::Utf8(string)
            }
            15 => CpInfo::MethodHandle,
            16 => CpInfo::MethodType,
            17 => CpInfo::Dynamic,
            18 => CpInfo::InvokeDynamic,
            19 => CpInfo::Module,
            20 => CpInfo::Package,
            _ => unimplemented!("{}", tag)
        };
        parsed_items.push(info);
    }

    ConstPool::from_vec(parsed_items)
}