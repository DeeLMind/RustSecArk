fn main() {
    // ======================
    // Unsigned integers (only positive values)
    // ======================
    let u8_var: u8 = 255;                   // 0 to 255
    let u16_var: u16 = 65535;               // 0 to 65535
    let u32_var: u32 = 4_294_967_295;       // 0 to 2^32 - 1
    let u64_var: u64 = 18_446_744_073_709_551_615; // 0 to 2^64 - 1
    let u128_var: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455; // 0 to 2^128 - 1
    let usize_var: usize = 10;              // Platform-dependent: u32 on 32-bit, u64 on 64-bit

    // ======================
    // Signed integers (positive and negative values)
    // ======================
    let i8_var: i8 = -128;                  // -128 to 127
    let i16_var: i16 = -32_768;
    let i32_var: i32 = -2_147_483_648;
    let i64_var: i64 = -9_223_372_036_854_775_808;
    let i128_var: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728;
    let isize_var: isize = -10;             // Platform-dependent: i32 on 32-bit, i64 on 64-bit

    // ======================
    // Binary representation
    // ======================
    let bit: u8 = 0b0000_0001;

    // ======================
    // Print all values
    // ======================
    println!("u8_var: {}", u8_var);
    println!("u16_var: {}", u16_var);
    println!("u32_var: {}", u32_var);
    println!("u64_var: {}", u64_var);
    println!("u128_var: {}", u128_var);
    println!("usize_var: {}", usize_var);

    println!("i8_var: {}", i8_var);
    println!("i16_var: {}", i16_var);
    println!("i32_var: {}", i32_var);
    println!("i64_var: {}", i64_var);
    println!("i128_var: {}", i128_var);
    println!("isize_var: {}", isize_var);

    println!("bit (u8 binary): {:08b}", bit);
}
