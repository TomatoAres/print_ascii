// 需要特殊处理
pub fn print_1() {
    let from: u32 = 'Z' as u32;
    println!("a");
    for c in char::from_u32(from + 1).unwrap()..'a' {
        println!("{}", c);
    }
    println!("Z");
}
