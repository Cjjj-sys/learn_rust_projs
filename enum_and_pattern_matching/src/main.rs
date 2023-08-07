use std::io;
use std::ptr::null;

fn main() {
    let join_msg = Msg::Join;
    let mov_msg = Msg::Move {x: 114, y:514};
    let quit_msg = Msg::Quit;
    let say_msg = Msg::Say(String::from("Hello"));
/*    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("please retry.");*/
    join_msg.execute();
    mov_msg.execute();
    quit_msg.execute();
    say_msg.execute();

    if let Msg::Say(text) = say_msg {
        println!("Debug msg: {}", text);
    }


}

#[derive(Debug)]
enum Msg {
    Join,
    Move {x: i32, y: i32},
    Quit,
    Say(String)
}

impl Msg {
    fn execute(&self) {
        //dbg!(self);
        match self {
            Msg::Join => println!("Mi has joined."),
            Msg::Move { x, y } => println!("Mi has moved to ({x}, {y})"),
            Msg::Say(text) => println!("Mi said '{text}'"),
            Msg::Quit => println!("Mi has quited.")
        };
    }
}