fn main() {
    let true_value: bool = true;
    let false_value: bool = false;

    println!("true in binary:  {:08b}", true_value as u8);
    println!("false in binary: {:08b}", false_value as u8);
    
    let result = format!("true in binary:  {:08b}\nfalse in binary: {:08b}\n", true_value as u8, false_value as u8);
    println!("{}", result);
}