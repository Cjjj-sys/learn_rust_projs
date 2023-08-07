fn main() {
    println!("Hello, world!");
    let user1 = User {
        active: true,
        username: String::from("Newton"),
        email: String::from("Newton@heaven.com")
    };
    let user2 = User {
        active: true,
        username: user1.username.clone(),
        email: user1.email

    };
    println!("{}", &user1.username);
    println!("{}", &user2.username);
        struct Point { x: i32, y: i32 }

        let mut p = Point { x: 0, y: 0 };
        let x = &mut p.x;
        *x += 1;
        println!("{}, {}", p.x, p.y);
}

struct User {
    active: bool,
    username: String,
    email: String
}