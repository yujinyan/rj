use std::collections::HashMap;

use crate::{const_pool, Opcode};

pub(crate) struct Method<'a> {
    pub stack_size: usize,
    pub local_size: usize,
    pub codes: &'a [Opcode],
    pub class: &'a str,
}

pub(crate) struct MethodArea<'a> {
    methods: HashMap<&'a str, Method<'a>>,
    classes: HashMap<&'a str, Class<'a>>,
}

pub(crate) struct Class<'a> {
    pub(crate) super_class: &'a Option<Class<'a>>,
    pub const_pool: const_pool::ConstPool<'a>,
    pub(crate) methods: HashMap<&'a str, Method<'a>>,
}

impl<'a> MethodArea<'a> {
    pub(crate) fn new() -> MethodArea<'a> {
        MethodArea {
            methods: Default::default(),
            classes: Default::default(),
        }
    }

    pub(crate) fn put(&mut self, key: &'a str, method: Method<'a>) {
        self.methods.insert(key, method);
    }

    pub(crate) fn put_class(&mut self, key: &'a str, class: Class<'a>) {
        self.classes.insert(key, class);
    }

    pub(crate) fn resolve_method(&self, key: &str) -> &Method {
        let method = self.methods.get(key);
        match method {
            None => panic!("not available"),
            Some(m) => return m
        }
    }

    pub(crate) fn resolve_class(&self, key: &str) -> &Class {
        let class = self.classes.get(key);
        match class {
            None => panic!("{:?} not available", key),
            Some(c) => return c
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::method_area::{Method, MethodArea};

    #[test]
    fn can_put_and_resolve_from_method_table() {
        let mut table = MethodArea::new();
        table.put("foo", Method {
            codes: &[],
            stack_size: 2,
            local_size: 0,
            class: "",
        });
        table.resolve_method("foo");
    }
}
