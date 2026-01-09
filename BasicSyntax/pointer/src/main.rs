fn box_(){
    let b = Box::new(5);
    println!("b = {} | p_b = {:p}", b, &b);
    let c = Box::new(10);
    println!("c = {} | p_c = {:p}", c, &c);
    let d = *b + *c;
    println!("d = {} | p_d = {:p}", d, &d);

    let e = String::from("Hello, Box!");
    println!("e stack addr      = {:p}", &e);
    println!("e.heap ptr        = {:p}", e.as_ptr());
    println!("e.len             = {}", e.len());
    println!("e.capacity        = {}", e.capacity());

    let x = Box::new(123);

    println!("box stack addr = {:p}", &x);
    println!("box heap  addr = {:p}", x.as_ref());

}

fn box_string(){
    use std::slice;
    let b = String::from("abc");
    let raw = &b as *const String as *const u8;
    let bytes = unsafe { slice::from_raw_parts(raw, 24) };

    println!("{:x?}", bytes);
}

fn box_trait(){
    trait Animal {
        fn speak(&self);
    }

    struct Dog;

    impl Animal for Dog {
        fn speak(&self) {
            println!("woof");
        }
    }

    let a: Box<dyn Animal> = Box::new(Dog);
    a.speak();
}

fn rc_(){
    use std::rc::Rc;
    let a = Rc::new(5);
    println!("a = {}", a);
    let b = Rc::clone(&a);
    println!("b = {}", b);
    let c = Rc::clone(&a);
    println!("c = {}", c);
    let d = *a + *b + *c;
    println!("d = {}", d);
}

fn basic_const_pointer(){
    let a : i32 = 10;
    println!("a = {} | a_address = {:p}", a, &a);
    let p_a : *const i32 = &a;
    println!("p_a = {:?} | *p_a = {}", p_a, unsafe { *p_a });
    let p_p_a : *const *const i32 = &p_a;
    println!("p_p_a = {:?} | *p_p_a = {:?} | **p_p_a = {}", p_p_a, unsafe { *p_p_a }, unsafe { **p_p_a });
}

fn basic_mut_pointer(){
    let mut b : i32 = 20;
    println!("b = {} | b_address = {:p}", b, &b);
    let mut p_b : *mut i32 = &mut b;
    println!("p_b = {:?} | *p_b = {}", p_b, unsafe { *p_b });
    unsafe {
        *p_b = 30;
    }
    println!("after modify: b = {} | *p_b = {}", b, unsafe { *p_b });
    let p_p_b : *mut *mut i32 = &mut p_b;
    println!("p_p_b = {:?} | *p_p_b = {:?} | **p_p_b = {}", p_p_b, unsafe { *p_p_b }, unsafe { **p_p_b });
    let c : i32 = 40;
    let p_c : *const i32 = &c;
    p_b = p_c as *mut i32;
    println!("after p_b point to c: p_b = {:?} | *p_b = {}", p_b, unsafe { *p_b });
}

fn fn_pointer(){
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    let f: fn(i32) -> i32 = add_one;
    let result = f(5);
    println!("result = {}", result);

    let fbox: Box<dyn Fn(i32) -> i32> = Box::new(add_one);
    let result_box = fbox(10);
    println!("result_box = {}", result_box);
}

fn main() {
    // basic_const_pointer();
    // basic_mut_pointer();
    // box_();
    // box_string();
    // box_trait();
    // fn_pointer();
}
