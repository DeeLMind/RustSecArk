fn main() {
    // Basic booleans
    let t: bool = true;
    let f: bool = false;

    // Logical operators
    let not_t = !t;
    let and_tf = t && f;
    let or_tf = t || f;

    // Comparison results
    let x = 7;
    let y = 10;
    let eq = x == y;
    let neq = x != y;
    let gt = x > y;
    let lt = x < y;

    // Bool to int (manual)
    let bool_val = true;
    let int_val = if bool_val { 1 } else { 0 };

    // Output everything
    println!("t: {}, f: {}", t, f);
    println!("!t = {}", not_t);
    println!("t && f = {}", and_tf);
    println!("t || f = {}", or_tf);

    println!("x == y: {}", eq);
    println!("x != y: {}", neq);
    println!("x > y:  {}", gt);
    println!("x < y:  {}", lt);

    println!("bool_val as int: {}", int_val);

    // Use in condition
    if t {
        println!("Condition is true!");
    } else {
        println!("Condition is false!");
    }
}