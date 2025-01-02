fn main() {
    let true_value: bool = true;
    let _false_value: bool = false;
    println!("true in binary:  {:08b}", true_value as u8);

    let u8_true_value: u8 = 10;
    println!("u8_true_value: {}", u8_true_value == 0);

    let int_value: i32 = 42;
    println!("int_value as f32: {}", int_value as f32);
    let float_value: f64 = int_value as f64;
    println!("int_value as f64: {}", float_value);

    let float_value_2: f64 = 3.14159;
    let int_value_2: i32 = float_value_2 as i32;
    println!("float_value_2 as i32: {}", int_value_2);

    let char_value: char = 'A';
    let char_as_u8: u8 = char_value as u8;
    println!("char 'A' as u8: {}", char_as_u8);

    let str_value = "123";
    let parsed_value: i32 = str_value.parse().expect("Failed to parse");
    println!("Parsed string '123' as i32: {}", parsed_value);

    struct Point {
        x: i32,
        y: i32,
    }

    struct PointF {
        x: f64,
        y: f64,
    }

    impl From<Point> for PointF {
        fn from(p: Point) -> Self {
            PointF {
                x: p.x as f64,
                y: p.y as f64,
            }
        }
    }

    let p = Point { x: 5, y: 10 };
    let pf: PointF = p.into();
    println!("PointF: ({:.1}, {:.1})", pf.x, pf.y);

    let x = 42;
    let y = 100;
    
    let ptr_x: *const i32 = &x;
    let ptr_y: *mut i32 = &y as *mut i32;

    unsafe {
        let mutable_ptr_x: *mut i32 = ptr_x as *mut i32; 
        println!("mutable_ptr_x points to: {}", *mutable_ptr_x);
    }
}