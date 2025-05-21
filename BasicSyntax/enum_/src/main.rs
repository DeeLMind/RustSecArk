fn main() {
    // 1. Custom enum definition
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    impl Direction {
        // Method to describe the direction as a string
        fn describe(&self) -> &str {
            match self {
                Direction::Up => "Going up",
                Direction::Down => "Going down",
                Direction::Left => "Going left",
                Direction::Right => "Going right",
            }
        }
    }

    // Using the Direction enum
    let dir = Direction::Left;
    println!("Direction: {}", dir.describe());

    // 2. Option<T> - Standard enum representing optional values
    let some_number: Option<i32> = Some(42);
    let no_number: Option<i32> = None;

    // Matching on Option values
    match some_number {
        Some(n) => println!("Got a number: {}", n),
        None => println!("No number found"),
    }

    match no_number {
        Some(n) => println!("Got a number: {}", n),
        None => println!("No number found"),
    }

    // 3. Result<T, E> - Standard enum representing success or error
    // Define a custom error type
    #[derive(Debug)]
    enum MyError {
        NotFound,
        PermissionDenied,
    }

    // Function that returns a Result type
    fn might_fail(success: bool) -> Result<String, MyError> {
        if success {
            Ok("Operation succeeded".to_string())
        } else {
            Err(MyError::NotFound)
        }
    }

    // Using the Result enum with pattern matching
    match might_fail(true) {
        Ok(msg) => println!("Success: {}", msg),
        Err(e) => println!("Error: {:?}", e),
    }

    match might_fail(false) {
        Ok(msg) => println!("Success: {}", msg),
        Err(e) => println!("Error: {:?}", e),
    }
}
