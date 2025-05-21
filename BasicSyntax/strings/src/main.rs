fn main() {
    // ======================
    // &str: string slice, immutable
    // ======================
    let s1: &str = "hello";              // string literal
    let s2: &'static str = "world";      // inferred as &str

    // ======================
    // String: heap-allocated, growable
    // ======================
    let mut s3 = String::from("hello");  // from &str
    let s4 = " world".to_string();       // method form

    // ======================
    // Append and Concatenate
    // ======================
    s3.push('!');                        // append char
    s3.push_str(" Welcome");             // append &str
    let s5 = s3.clone() + &s4;           // `+` moves left, borrows right
    let s6 = format!("{}{}", s3, s4);    // safer concat (non-destructive)

    // ======================
    // Length and Slicing
    // ======================
    let len = s3.len();                  // byte length
    let slice = &s3[0..5];               // slicing: careful with UTF-8!

    // ======================
    // Iterating characters and bytes
    // ======================
    println!("Characters:");
    for c in s3.chars() {
        print!("{} ", c);
    }

    println!("\nBytes:");
    for b in s3.bytes() {
        print!("{} ", b);
    }

    // ======================
    // Comparison
    // ======================
    let is_equal = s1 == s2;
    let contains = s3.contains("come");

    // ======================
    // Output all results
    // ======================
    println!("\n\n--- Outputs ---");
    println!("s1 (&str): {}", s1);
    println!("s2 (&str): {}", s2);
    println!("s3 (String): {}", s3);
    println!("s4 (String): {}", s4);
    println!("s5 (s3 + s4): {}", s5);
    println!("s6 (format!): {}", s6);
    println!("Length of s3: {}", len);
    println!("Slice of s3[0..5]: {}", slice);
    println!("s1 == s2: {}", is_equal);
    println!("s3 contains \"come\": {}", contains);
}
