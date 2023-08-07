fn main() {
    let rect1 = Rectangle {
        width: 100,
        height: 80
    };
    let rect2 = Rectangle {
        width: 10,
        height: 8
    };
    let rect3 = Rectangle {
        width: 1000,
        height: 800
    };
    let square1 = Rectangle::square(800);
    println!("rect1's area is {}", rect1.area());
    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("square1's area is {}", square1.area());
    println!("can rect1 hold square1? {}", rect1.can_hold(&square1));
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, another: &Rectangle) -> bool {
        another.width <= self.width && another.height <= self.height
    }
}