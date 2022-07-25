use std::{fs, fs::File, io, io::ErrorKind, io::Read};
fn main() {
    // panic!("Crash program!");
    //let vec = vec![1, 2, 3, 4];
    //vec[99];
    let fs: Result<std::fs::File, std::io::Error> = File::open("hello.txt");
    let _fs = match fs {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file {:?}", e),
            },
            other_error => panic!("problem opening file: {:?}", other_error),
        },
    };
    expect();
}

fn expect() {
    let r = File::open("herllo.text").expect("Failed to open");
}
fn read_username_file_long_way() -> Result<String, io::Error> {
    let f = File::open("Hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
fn read_username_from_file_short_way() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
fn read_username_from_file_shorter_way() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
fn read_username_from_file_shortest_way() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
