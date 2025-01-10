fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn find_element(vec: &Vec<i32>, target: i32) -> Option<usize> {
    vec.iter().position(|&x| x == target)
}

fn create_large_object() -> Box<[i32; 100]> {
    Box::new([0; 100])
}

fn get_even_numbers(numbers: Vec<i32>) -> Vec<i32> {
    numbers.into_iter().filter(|&x| x % 2 == 0).collect()
}

fn print_hello() -> () {
    println!("Hello, World!");
}


fn main() {
    print!("{:?}",find_element(&vec![1,2,3], 2));
}
