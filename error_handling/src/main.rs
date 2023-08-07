use std::fs::{File, set_permissions};
use std::io::Write;
use std::io::ErrorKind;
use std::fs::Permissions;


fn main() {
    let f = File::open("f.txt");
    let mut f_txt = match f {
        Ok(f) => f,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                match File::create("f.txt") {
                    Ok(f) => f,
                    Err(e) => panic!("{e}")
                }
            },
            other_errors => panic!("{other_errors}")
        }
    };
   
    //write!(f_txt, "sadasdasd").expect("?");
    println!("{:#?}", f_txt)
}
