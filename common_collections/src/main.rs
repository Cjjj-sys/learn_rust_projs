use std::collections::{hash_map, HashMap};

fn main() {
    let s = String::from("hello world");
    let cs = String::from("你好 世界");
    let ss = String::from("h");
    let scs = String::from("你");
    println!("[{s}]'s len is {}\n[{cs}]'s len is {}", s.len(), cs.len());
    println!("[{ss}]'s len is {}\n[{scs}]'s len is {}", ss.len(), scs.len());
    println!("[{ss}]:{:?}", ss.as_bytes());
    println!("[{scs}]:{:?}", scs.as_bytes());
    println!("chars[{s}]:{:?}", s.chars());
    println!("chars[{cs}]:{:?}", cs.chars());
    // let mut m = HashMap::new();
    // m.insert(s, cs);
    // println!("{s}");
}

fn reverse(v: &mut Vec<String>) {
    let n = v.len();
    for i in 0 .. n / 2 {
        let p1 = &mut v[i] as *mut String;
        let p2 = &mut v[n - i - 1] as *mut String;
        unsafe { std::ptr::swap_nonoverlapping(p1, p2, 1); }
    }
}