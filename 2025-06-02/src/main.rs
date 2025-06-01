fn main() {
    /*
    signed integer: i8, i16, i32, i64, i128, isize
    unsigned integer: u8, u16, u32, u64, u128, usize
    */
    println!("===integers===");
    // integer default: i32
    let default_n = 2_147_483_647;
    println!("default integer (i32) = {}", default_n);
    // isize: アーキテクチャにより決められた最大サイズでできるsigned integer
    let isize_n: isize = 9_223_372_036_854_775_807;
    println!("isize integer = {}", isize_n);

    // mut: mutable variableであることを示すkeyword
    println!("===mutable variables===");
    let mut mutable_n = 0;
    println!("mutable integer (before)= {}", mutable_n);
    mutable_n = 1;
    println!("mutable integer (after)= {}", mutable_n);

    // char = 4 bytes
    println!("===characters===");
    let first_letter = 'A';
    println!("first letter = {}", first_letter);
    let emoji = '😛';
    println!("emoji = {}", emoji);
    // type casting char to u8
    let char_from_u8 = 'A' as u8;
    println!("char from u8 = {}", char_from_u8);
    // type casting char to u32
    // charは4bytes=32bitsのため、安全にcastingするにはu32が良い。
    // しかしながら、charはu8にしかcastingできない。
    let emoji_from_u32 = '😛' as u32;
    println!("emoji from u32 = {}", emoji_from_u32);
    let max_u8: u8 = 255;
    println!("max_u8_char = {}", max_u8 as char);

    // string
    println!("===strings===");
    println!("Size of a char: {}", size_of::<char>());
    println!("Size of a string containing 'a': {}", "a".len()); // 1 byte
    println!("Size of a string containing 'α': {}", "α".len()); // 2 bytes
    println!("Size of a string containing '国': {}", "国".len()); // 3 bytes
    println!("Size of a string containing '😛': {}", "😛".len());// 4 bytes
    let target_string = "文字列の文字数をcountする";
    println!("count = {}, length = {}", target_string.chars().count(), target_string.len());
}
