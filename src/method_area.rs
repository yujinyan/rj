use std::collections::HashMap;

use crate::Opcode;

pub(crate) struct Method<'a> {
    pub stack_size: usize,
    pub local_size: usize,
    pub codes: &'a [Opcode],
    pub class: &'a str,
}

pub(crate) struct MethodTable<'a> {
    methods: HashMap<&'a str, Method<'a>>,
    classes: HashMap<&'a str, Class<'a>>,
}

pub(crate) struct Class<'a> {
    super_class: &'a Option<Class<'a>>,
    pub const_pool: crate::const_pool::ConstPool<'a>,
    methods: HashMap<&'a str, Method<'a>>,
}

impl MethodTable<'_> {
    pub(crate) fn new<'a>() -> MethodTable<'a> {
        MethodTable {
            methods: Default::default(),
            classes: Default::default(),
        }
    }

    fn put<'a>(&mut self, key: &'a str, method: Method<'a>) {
        self.methods.insert(key, method);
    }

    pub(crate) fn resolve_method<'a>(&mut self, key: &str) -> &Method {
        let method = self.methods.get(key);
        match method {
            None => panic!("not available"),
            Some(m) => return m
        }
    }

    pub(crate) fn resolve_class<'a>(&mut self, key: &str) -> &Class {
        let class = self.classes.get(key);
        match class {
            None => panic!("not available"),
            Some(c) => return c
        }
    }
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

    fn resolve_method() {}
}

#[cfg(test)]
mod tests {
    use crate::method_area::{Method, MethodTable};

    #[test]
    fn can_put_and_resolve_from_method_table() {
        let mut table = MethodTable::new();
        table.put("foo", Method {
            codes: &[],
            stack_size: 2,
            local_size: 0,
            class: "",
        });
        table.resolve_method("foo");
    }
}
