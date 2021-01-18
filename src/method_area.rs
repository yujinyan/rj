use std::collections::HashMap;

use crate::{const_pool, Opcode};
use std::fmt::{Debug, Formatter, Error, Display};

// JLS §12.3.2, JVMS §2.5.4
pub struct MethodArea<'a> {
    methods: HashMap<&'a str, Method<'a>>,
    classes: HashMap<&'a str, Class<'a>>,
}

impl Debug for MethodArea<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        // write!(f, "MethodArea[classes:{}, methods{}]", )
        // let classes = self.classes.keys().collect()
        let classes = self.classes.keys().fold(String::from(""), |mut s, it| s + it + " ");
        let methods = self.methods.keys().fold(String::from(""), |s, it| s + it + " ");
        f.debug_struct("MethodArea")
            .field("classes", &classes)
            .field("methods", &methods)
            .finish()
    }
}

pub(crate) struct Method<'a> {
    pub stack_size: usize,
    pub local_size: usize,
    pub codes: Vec<Opcode>,
    pub class: &'a str,
}

impl Method<'_> {
    pub(crate) fn new(
        stack_size: usize, local_size: usize,
        codes: Vec<Opcode>, class: &str,
    ) -> Method {
        Method {
            stack_size,
            local_size,
            codes,
            class,
        }
    }
}

pub(crate) struct Class<'a> {
    pub(crate) super_class: &'a Option<Class<'a>>,
    pub const_pool: const_pool::ConstPool,
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
            None => panic!("Cannot find method {}", key),
            Some(m) => return m
        }
    }

    pub(crate) fn resolve_class(&self, key: &str) -> &Class {
        let class = self.classes.get(key);
        match class {
            None => panic!("{:?} class not available", key),
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
            codes: vec![],
            stack_size: 2,
            local_size: 0,
            class: "",
        });
        table.resolve_method("foo");
    }
}
