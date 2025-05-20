fn main() {
    // ======================
    // f32 ‒ single-precision (32-bit IEEE-754)
    // ======================
    let f32_var: f32      = 3.1415;
    let f32_nan: f32      = f32::NAN;          // Not-a-Number
    let f32_inf: f32      = f32::INFINITY;     //  +∞
    let f32_neg_inf: f32  = f32::NEG_INFINITY; //  -∞

    // ======================
    // f64 ‒ double-precision (64-bit IEEE-754, default)
    // ======================
    let f64_var: f64      = 2.718281828459045;
    let f64_nan: f64      = f64::NAN;
    let f64_inf: f64      = f64::INFINITY;
    let f64_neg_inf: f64  = f64::NEG_INFINITY;
    
    // NaN (Not-a-Number) examples
    let nan1 = 0.0 / 0.0;        // undefined division
    let nan2 = (-1.0f64).sqrt(); // square root of negative number
    let nan3 = f32::INFINITY - f32::INFINITY; // invalid operation
    let value: f64 = 0.0 / 0.0;

    if value.is_nan() {
        println!("Value is NaN");
    }

    // ======================
    // Literal variants (all equal to 10.0)
    // ======================
    let f64_lit1 = 10.0;      // default f64
    let f64_lit2 = 10.;       // trailing dot
    let f64_lit3 = 10f64;     // type suffix, no decimal
    let f64_lit4 = 10.0f64;   // decimal + suffix
    let f64_lit5 = 10.0_f64;  // underscore before suffix

    // ======================
    // Conversions between ints and floats
    // ======================
    let from_int_to_f32: f32 = 42u32 as f32;   // widening, exact
    let from_f64_to_int: u32 = f64_var as u32; // fractional part truncated

    // ======================
    // Print all values
    // ======================
    println!("f32_var:      {}", f32_var);
    println!("f32_nan:      {}", f32_nan);
    println!("f32_inf:      {}", f32_inf);
    println!("f32_neg_inf:  {}", f32_neg_inf);

    println!("f64_var:      {}", f64_var);
    println!("f64_nan:      {}", f64_nan);
    println!("f64_inf:      {}", f64_inf);
    println!("f64_neg_inf:  {}", f64_neg_inf);

    println!(
        "f64 literals:  {}, {}, {}, {}, {}",
        f64_lit1, f64_lit2, f64_lit3, f64_lit4, f64_lit5
    );

    println!("from_int_to_f32:     {}", from_int_to_f32);
    println!("from_f64_to_int:     {} (fraction truncated)", from_f64_to_int);
}