// mod print_az;

fn main() {
    // println!("print a to Z");
    // print_Az();

    println!("print a to Z");
    print_a_to_Z();
}

pub fn print_A_to_z() {
    // let from:u32 = 'Z';
    for c in 'A'..'z' {
        println!("{}", c);
    }
    println!("z");
}

// 需要特殊处理
pub fn print_a_to_Z() {
    let from: u32 = 'Z' as u32;
    for c in (char::from_u32(from + 1).unwrap()..'a') {
        println!("{}", c);
    }
    // println!("Z");
}
