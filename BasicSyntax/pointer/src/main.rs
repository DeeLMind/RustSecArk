fn box_(){
    let b = Box::new(5);
    println!("b = {}", b);
    let c = Box::new(10);
    println!("c = {}", c);
    let d = *b + *c;
    println!("d = {}", d);
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

fn main() {
    rc_();
}
