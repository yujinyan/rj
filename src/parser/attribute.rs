use crate::const_pool::ConstPool;
use crate::method_area::Method;
use crate::Opcode;
use crate::parser::{Attribute};
use crate::parser::Reader;

pub(crate) fn read_attribute(reader: &mut Reader, const_pool: &ConstPool) -> Attribute {
    let name_index = reader.read_u16();
    let name = const_pool.resolve_utf8(name_index);
    match name {
        "Code" => read_code_attribute(reader),
        _ => unimplemented!("name: {}", name)
    }
}

fn read_code_attribute(reader: &mut Reader) -> Attribute {
    let length = reader.read_u32();
    let max_stack = reader.read_u16();
    let max_locals = reader.read_u16();
    let mut code_length = reader.read_u32();
    let mut codes: Vec<Opcode> = Vec::new();

    while code_length > 0 {
        let opcode = reader.read_u8();
        code_length -= 1;
        let code = match opcode {
            0x03 => Opcode::iconst_0,
            0x04 => Opcode::iconst_1,
            0x1a => Opcode::iload_0,
            0x1b => Opcode::iload_1,
            0x2a => Opcode::aload_0,
            0x3b => Opcode::istore_0,
            0x3c => Opcode::istore_1,
            0x60 => Opcode::iadd,
            0xa7 => {
                code_length -= 2;
                Opcode::goto(reader.read_u16() as usize)
            }
            0xa1 => {
                code_length -= 2;
                Opcode::if_icmplt(reader.read_u16() as usize)
            }
            0xac => Opcode::ireturn,
            0xb1 => Opcode::r#return,
            0xb7 => {
                code_length -= 2;
                Opcode::invokespecial(reader.read_u16())
            }
            0xb8 => {
                code_length -= 2;
                Opcode::invokestatic(reader.read_u16() as usize)
            }
            _ => panic!("cannot parse opcode {:04X?}", opcode)
        };
        codes.push(code)
    };

    let exception_table_length = reader.read_u16();
    dbg!(exception_table_length);

    let attributes_count = reader.read_u16();
    dbg!(attributes_count);

    for _ in 0..attributes_count {
        let name_index = reader.read_u16();
        dbg!(name_index);
        let length = reader.read_u32();
        // fixme
        let content = reader.read_utf8(length as u16);
        dbg!(length, content);
    }

    Attribute::Code { max_stack, max_locals, codes }
}
