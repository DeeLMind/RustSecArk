fn main() {
    let true_value: bool = true;
    let false_value: bool = false;
    println!("true in binary:  {:08b}", true_value as u8);

    let u8_true_value: u8 = 10;
    println!("u8_true_value: {}", u8_true_value == 0);
}
