use crate::const_pool::CpInfo;

struct ClassFile<'a> {
    const_pool_count: u16,
    const_pool: &'a[CpInfo<'a>],
    this_class: u16,
    super_class: u16,
    interfaces_count: u16,
    interfaces: &'a[u16],
    fields_count: u16,
    fields: &'a[FieldInfo],
    methods_count: u16,
    methods: &'a[MethodInfo],
    attributes_count: u16,
    attributes: [AttributeInfo],
}

struct FieldInfo {}

struct MethodInfo {}

struct AttributeInfo {}