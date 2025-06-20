trait Speak {
    fn speak(&self);
}

trait Greet{
    fn greet(&self){
        println!("greet");
    }
}

struct Person;

impl Greet for Person {

}

struct Dog;

impl Speak for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

struct Cat;

impl Speak for Cat {
    fn speak(&self) {
        println!("Miao!");
    }
}

fn make_speak<T: Speak>(animal: T) {
    animal.speak();
}

fn complex<T: Greet + Speak>(item: T) {
    item.greet();
    item.speak();
}

fn complex1<T, U>(a: T, b: U)
where
    T: Greet,
    U: Speak,
{
    a.greet();
    b.speak();
}

trait Draw {
    fn draw(&self);
}

struct Circle;
impl Draw for Circle {
    fn draw(&self) {
        println!("Drawing a Circle");
    }
}

fn draw_object(obj: &dyn Draw) {
    obj.draw();
}

trait Iterator1 {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

fn foo() -> impl Iterator<Item = i32> {
    vec![1, 2, 3].into_iter()
}

fn main() {
    let dog = Dog;
    make_speak(dog);
    let cat = Cat;
    cat.speak();
    let people = Person;
    people.greet();
    let c = Circle;
    draw_object(&c);
}
