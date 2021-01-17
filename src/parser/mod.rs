use crate::const_pool::ConstPool;
use crate::Opcode;

pub(crate) mod reader;
pub(crate) mod const_pool;
pub(crate) mod method;
pub(crate) mod class_file;
mod class_parser;

#[derive(Debug)]
pub(crate) struct ClassFile<'a> {
    pub(crate) const_pool: ConstPool,
    pub(crate) this_class: String,
    pub(crate) super_class: String,
    pub(crate) interfaces: &'a [u16],
    pub(crate) fields: Vec<FieldInfo>,
    pub(crate) methods: Vec<MethodInfo>,
    pub(crate) attributes: Vec<AttributeValue>,
}

#[repr(u16)]
enum AccessFlags {
    PUBLIC = 0x0001,
    FINAL = 0x0010,
    SUPER = 0x0020,
    INTERFACE = 0x0200,
    ABSTRACT = 0x0400,
    SYNTHETIC = 0x1000,
    ANNOTATION = 0x2000,
    ENUM = 0x4000,
    MODULE = 0x8000,
}

#[derive(Debug)]
pub(crate) struct FieldInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<AttributeValue>,
}


#[derive(Debug)]
pub(crate) struct MethodInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<AttributeValue>,
}

struct MethodAttribute {}

pub(crate) struct Attribute {
    attribute_name_index: u16,

}

// §4.7
#[derive(Debug)]
pub(crate) enum AttributeValue {
    ConstValue,
    Code {
        max_stack: u16,
        max_locals: u16,
        codes: Vec<Opcode>,
    },
    StackMapTable,
    BootstrapMethods,
    NestHost,
    NestMembers,
}
