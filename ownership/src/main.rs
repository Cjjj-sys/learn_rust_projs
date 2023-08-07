use std::vec;

fn main() {
    let mut a = 1; // a: R W O
    let mut ref_a = &mut a; // ref_a: R W O; a:
    *ref_a += 1;
    println!("{}", *ref_a);
    println!("{a}");
    //println!("{ref_a}");

    let mut s = vec![String::from("hello world")];
    let ss = s.remove(0);
    println!("{}", first_word(&ss));
    let mut a_box = Box::new(2);
    let b = **********&&&&&&&&&&mut(*&(*&mut((*&*&mut(*a_box-1)+1).clone()+1)-1)+1);
    println!("{}", b+*a_box);
}

fn first_word(str: &String) -> usize {
    let str = str.as_bytes();
    for (i, &item) in str.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    };
    return  str.len();
}


pub fn reverse(input: &str) -> String {
    let mut result = String::new();
    for item in input.chars().rev() {
        result.push(item);
    }
    return result;
}

