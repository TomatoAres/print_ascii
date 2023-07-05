mod layer1;
mod layer2;

fn main() {
    println!("print a to Z");
    layer1::print_1();

    layer2::layer2::print_2();
}
