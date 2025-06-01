fn main() {
    /*
    signed integer: i8, i16, i32, i64, i128, isize
    unsigned integer: u8, u16, u32, u64, u128, usize
    */

    // integer default: i32
    let default_n = 2_147_483_647;
    println!("default integer (i32) = {}", default_n);
    // isize: アーキテクチャにより決められた最大サイズでできるsigned integer
    let isize_n: isize = 9_223_372_036_854_775_807;
    println!("isize integer = {}", isize_n);

    // mut: mutable variableであることを示すkeyword
    let mut mutable_n = 0;
    println!("mutable integer (before)= {}", mutable_n);
    mutable_n = 1;
    println!("mutable integer (after)= {}", mutable_n);
}
