use crate::const_pool::CpInfo;
use crate::Opcode;

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

struct ClassFile<'a> {
    const_pool_count: u16,
    const_pool: &'a [CpInfo<'a>],
    this_class: u16,
    super_class: u16,
    interfaces_count: u16,
    interfaces: &'a [u16],
    fields_count: u16,
    fields: &'a [FieldInfo],
    methods_count: u16,
    methods: &'a [MethodInfo],
    attributes_count: u16,
    attributes: [Attribute],
}

struct FieldInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<FieldInfoAttribute>,
}

struct FieldInfoAttribute {}

struct MethodInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<Attribute>,
}

struct MethodAttribute {}

pub(crate) struct Attribute {
    attribute_name_index: u16,

}

// §4.7
enum AttributeValue {
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