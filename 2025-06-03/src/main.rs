fn main() {
    // float: 32bit(f32), 64bit(f64)
    println!("===floats===");
    let n_64 = 1.8f64;
    let n_32 = 1.8f32;

    println!("n_32 / n_64 = {}", n_32 as f64 / n_64);
    println!("n_32 + 3iszie = {}", n_32 as isize + 3_isize);

    println!("===functions===");
    println!("3! = {}", factorial(3_u32));

    println!("===macros===");
    let f = "first";
    let s = "second";
    println!("{}, {}", f, s);
    // string interpolation
    println!("{s}, {f}");
    println! {
        "{first}, {second}",
        first = "first_arg",
        second = "second_arg",
    }

    println!("{0}, {2}, {1}", "zero", "one", "two");

    println!("===return types===");
    let empty_tuple_1 = empty_tuple_1();
    println!("empty tuple 1 = {:?}", empty_tuple_1);
    let empty_tuple_2 = empty_tuple_2();
    empty_tuple_2
}

fn factorial(n: u32) -> u32 {
    /*
    kotlinみたいに最後のラインがリターンの値となる
    リターンにしたいなら、「;」をつけない。
    「;」は次のラインがあることを示す
     */

    if n == 0 { 1 } else { n * factorial(n - 1) }
}

/*
    リターンとして可能なタイプ
    - (): empty tuple, unit type(void)
    - その他のタイプ
*/
fn empty_tuple_1() -> () {
    1;
}
fn empty_tuple_2() {
    2;
}
