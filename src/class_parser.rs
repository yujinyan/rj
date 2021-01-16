use std::io::Read;
use std::fs::File;
use crate::const_pool::{ConstPool, CpInfo};
use crate::Opcode;

struct Reader<'a> {
    u1_buffer: [u8; 1],
    u2_buffer: [u8; 2],
    u4_buffer: [u8; 4],
    handle: &'a mut File,
}

impl Reader<'_> {
    fn new(file: &mut File) -> Reader {
        Reader {
            u1_buffer: [0; 1],
            u2_buffer: [0; 2],
            u4_buffer: [0; 4],
            handle: file,
        }
    }

    fn read_u8(&mut self) -> u8 {
        self.handle.read(&mut self.u1_buffer);
        u8::from_be_bytes(self.u1_buffer)
    }

    fn read_u16(&mut self) -> u16 {
        self.handle.read(&mut self.u2_buffer);
        // println!("{:04X?}", self.u2_buffer);
        u16::from_be_bytes(self.u2_buffer)
    }

    fn read_u32(&mut self) -> u32 {
        self.handle.read(&mut self.u4_buffer);
        // println!("{:04X?}", self.u4_buffer);
        u32::from_be_bytes(self.u4_buffer)
    }

    fn read_utf8(&mut self, length: u16) -> String {
        let mut handle = self.handle.take(length as u64);
        let mut string = String::new();
        handle.read_to_string(&mut string);
        string
    }
}

fn parse(handle: &mut File) {
    let mut reader = Reader::new(handle);

    // magic
    reader.read_u32();

    let minor = reader.read_u16();
    dbg!(minor);

    let major = reader.read_u16();
    dbg!(major);

    let const_pool_count = reader.read_u16();
    dbg!(const_pool_count);

    let mut parsed_items = Vec::with_capacity(const_pool_count as usize);
    parsed_items.push(CpInfo::Placeholder);
    for i in 0..const_pool_count - 1 {
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
                let string = Box::leak(string.into_boxed_str());
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
        dbg!(&info);
        parsed_items.push(info);
    }

    let access_flag = reader.read_u16();
    dbg!(access_flag);

    let this_class = reader.read_u16();
    dbg!(this_class);

    let super_class = reader.read_u16();
    dbg!(super_class);

    let interface_count = reader.read_u16();
    dbg!(interface_count);
    for i in 0..interface_count {
        let const_pool_index = reader.read_u16();
        dbg!(const_pool_index);
    }

    let fields_count = reader.read_u16();
    dbg!(fields_count);

    let methods_count = reader.read_u16();
    dbg!(methods_count);

    for i in 0..methods_count {
        let access_flag = reader.read_u16();
        dbg!(i, access_flag);
        let name_index = reader.read_u16();
        dbg!(i, name_index);
        let descriptor_index = reader.read_u16();
        dbg!(i, descriptor_index);
        let attribute_count = reader.read_u16();
        dbg!(i, attribute_count);
        for i in 0..attribute_count {
            read_attribute(&mut reader);
        }
    }
}

fn read_attribute(reader: &mut Reader) {
    let name_index = reader.read_u16();
    let length = reader.read_u32();
    let max_stack = reader.read_u16();
    let max_local = reader.read_u16();
    let mut code_length = reader.read_u32();
    dbg!(code_length);
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
        dbg!(code);
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
}

#[cfg(test)]
mod tests {
    use crate::class_parser::parse;

    #[test]
    fn parse_test_file() {
        let mut handle = std::fs::File::open("C:\\Users\\yujinyan\\code\\study\\java\\Adder.class").unwrap();
        parse(&mut handle);
    }
}