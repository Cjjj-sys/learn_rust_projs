pub mod garden;

use garden::vegetables::Apple;

fn main() {
    let my_vegetable = Apple {};
    println!("Hello, world! {:#?}", my_vegetable);
}
