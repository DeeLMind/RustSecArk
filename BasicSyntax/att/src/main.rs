#[derive(Debug)]
enum Token {
    Number(i32),
    Text(String),
}

fn handle(token: Token) {
    // match token {
    //     num @ Token::Number(n @ 1..=10) => {
    //         println!("Small number {} in {:?}", n, num);
    //     }
    //     Token::Number(n) => {
    //         println!("Other number: {}", n);
    //     }
    //     _ => {}
    // }
}


#[derive(Debug)]
enum Message {
    Hello { id: i32 },
    Bye,
}

fn process(msg: Message) {
    match msg {
        // 如果 id 在 3 到 7 之间，绑定整个 msg 到 m
        m @ Message::Hello { id: 3..=7 } => {
            println!("id in range: {:?}", m);
        }
        Message::Hello { id } => {
            println!("other id: {}", id);
        }
        Message::Bye => println!("bye"),
    }
}

fn check(x: i32) {
    match x {
        val @ 1..=5 => println!("In range, got: {}", val),
        _ => println!("Out of range"),
    }
}

fn main() {
    check(3);
}