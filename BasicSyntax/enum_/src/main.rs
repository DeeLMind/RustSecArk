
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
    let result = divide(a, b)?;  // 使用 ? 传播错误
    Ok(result)
}

fn find_item<'a>(items: &[&'a str], search: &'a str) -> Option<&'a str> {
    for &item in items {
        if item == search {
            return Some(item);
        }
    }
    None
}

fn safe_find_item<'a>(items: &[&'a str], search: &'a str) -> Option<&'a str> {
    let item = find_item(items, search)?;
    Some(item)
}

fn main() {
    match safe_divide(10, 0) {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    let items = ["apple", "banana", "cherry"];
    match safe_find_item(&items, "banana") {
        Some(item) => println!("Found: {}", item),
        None => println!("Item not found"),
    }
}