use std::io::Read;
use std::fs::File;
use crate::const_pool::{ConstPool, CpInfo};
use crate::{Opcode, parser};
use crate::parser::{ClassFile, MethodInfo};

pub(crate) struct Reader<'a> {
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

    pub fn read_u8(&mut self) -> u8 {
        self.handle.read(&mut self.u1_buffer);
        u8::from_be_bytes(self.u1_buffer)
    }

    pub fn read_u16(&mut self) -> u16 {
        self.handle.read(&mut self.u2_buffer);
        // println!("{:04X?}", self.u2_buffer);
        u16::from_be_bytes(self.u2_buffer)
    }

    pub fn read_u32(&mut self) -> u32 {
        self.handle.read(&mut self.u4_buffer);
        // println!("{:04X?}", self.u4_buffer);
        u32::from_be_bytes(self.u4_buffer)
    }

    pub fn read_utf8(&mut self, length: u16) -> String {
        let mut handle = self.handle.take(length as u64);
        let mut string = String::new();
        handle.read_to_string(&mut string);
        string
    }
}

fn parse(handle: &mut File) -> ClassFile {
    let mut reader = Reader::new(handle);

    // magic
    reader.read_u32();

    let minor = reader.read_u16();
    dbg!(minor);

    let major = reader.read_u16();
    dbg!(major);

    let const_pool = parser::const_pool::read_const_pool(&mut reader);
    dbg!(&const_pool);

    let access_flag = reader.read_u16();
    dbg!(access_flag);

    let this_class: u16 = reader.read_u16();
    let this_class: &str = const_pool.resolve_utf8(this_class);
    let this_class: String = String::from(this_class);

    let super_class: u16 = reader.read_u16();
    let super_class: &str = const_pool.resolve_utf8(super_class);
    let super_class: String = String::from(super_class);

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
    let mut methods = Vec::with_capacity(methods_count as usize);

    for _ in 0..methods_count {
        let access_flags = reader.read_u16();
        let name_index = reader.read_u16();
        let descriptor_index = reader.read_u16();
        let attribute_count = reader.read_u16();

        let mut attributes = Vec::new();

        for _ in 0..attribute_count {
            let attribute = parser::reader::read_attribute(&mut reader, &const_pool);
            attributes.push(attribute)
        }
        let method = MethodInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        };
        methods.push(method);
    }

    ClassFile {
        const_pool,
        this_class,
        super_class,
        interfaces: &[],
        fields: vec![],
        methods,
        attributes: vec![],
    }
}


#[cfg(test)]
mod tests {
    use crate::parser::class_parser::parse;

    #[test]
    fn parse_test_file() {
        let mut handle = std::fs::File::open("C:\\Users\\yujinyan\\code\\study\\java\\Adder.class").unwrap();
        let class_file = parse(&mut handle);
        dbg!(class_file);
    }
}