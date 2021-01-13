use std::io::Read;
use std::fs::File;

fn parse(handle: &mut File) {
    let mut u4_buffer: [u8; 4] = [0; 4];
    let mut u2_buffer: [u8; 2] = [0; 2];
    handle.read(&mut u4_buffer);
    println!("{:#04X?}", u4_buffer); // magic

    handle.read(&mut u2_buffer);
    println!("{:#04X?}", u2_buffer); // minor

    handle.read(&mut u2_buffer);
    println!("{:#04X?}", u2_buffer); //major
    // let major_version: i32 = u2_buffer.into();
    // println!("major version is {}", major_version);

    handle.read(&mut u2_buffer);
    println!("{:#04X?}", u2_buffer); // const pool count
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