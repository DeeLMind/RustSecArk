
fn get_str<'a>(s: & 'a String) -> &'a str {
    &s[..]
}

fn get_str1(s: &String) -> &str {
    &s[..]
}

// fn get_str2(s: String) -> &str {
//     &s[..]
// }


fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

struct Important<'a> {
    data: &'a str,
}

impl<'a> Important<'a> {
    fn show(&self) -> &str {
        self.data
    }
}

struct Container<'a, T> {
    data: &'a T,
}

fn main() {
    println!("lifetimes main");
    let binding = String::from("lifetimes");
    let a = get_str(&binding);
    println!("{:?}", a);
}