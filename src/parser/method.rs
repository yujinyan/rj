use crate::parser::Reader;
use crate::parser;
use crate::method_area::Method;

// pub(crate) fn read_method<'a>(reader: &mut Reader) -> Method<'a> {
//     let access_flag = reader.read_u16();
//     // dbg!(i, access_flag);
//     let name_index = reader.read_u16();
//     // dbg!(i, name_index);
//     let descriptor_index = reader.read_u16();
//     // dbg!(i, descriptor_index);
//     let attribute_count = reader.read_u16();
//     // dbg!(i, attribute_count);
//     for i in 0..attribute_count {
//         parser::reader::read_attribute(reader);
//     }
//     Method {
//         stack_size: 0,
//         local_size: 0,
//         codes: vec![],
//         class: "",
//     }
// }

