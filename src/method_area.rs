use std::collections::HashMap;

use crate::Opcode;

pub(crate) struct Method<'a> {
    pub codes: &'a [Opcode]
}

pub(crate) struct Class<'a> {
    super_class: &'a Option<Class<'a>>,
    const_pool: crate::const_pool::ConstPool<'a>,
    methods: HashMap<&'a str, Method<'a>>,
}

struct ClassTable<'a> {
    table: HashMap<&'a str, Class<'a>>
}

impl ClassTable<'_> {
    fn new() -> ClassTable<'static> {
        ClassTable {
            table: Default::default()
        }
    }

    // fn add(&mut self, name: &str, class: Class) {
    //     self.table.insert(name, class);
    // }

    fn resolve_method() {

    }
}

