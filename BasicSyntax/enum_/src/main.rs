
#[derive(Debug)]
pub enum TryLockError {
    Error(std::io::Error),
    WouldBlock,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move(i32, i32),
    Write(String),
}

#[derive(Debug)]
enum Event {
    Click { x: i32, y: i32 },
    Key { code: u32 },
}

#[repr(u8)]
enum Status {
    Ok = 0,
    Error = 1,
}

enum List {
    Nil,
    Cons(i32, Box<List>),
}


fn main() {
    // Example usage of TryLockError
    let error = TryLockError::WouldBlock;
    println!("{:?}", error);

    // Example usage of Message
    let msg = Message::Move(10, 20);
    println!("{:?}", msg);

    match msg {
        Message::Quit => {}
        Message::Move(x, y) => {
            println!("Moving to ({}, {})", x, y);
        }
        Message::Write(text) => {}
    }

    // if let Message::Write(text) = msg {
    //     println!("{:?}", msg);
    // }

    // let Message::Write(text) = msg else {
    //     return;
    // };

    // Example usage of Event
    let event = Event::Click { x: 100, y: 200 };
    println!("{:?}", event);
}
