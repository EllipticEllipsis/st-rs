mod header;
use std::{fs, env};

use binrw::{prelude::*, io::Cursor};

fn run(mut contents: Cursor<Vec<u8>>) {
    let header = header::HDRR::read(&mut contents).unwrap();
    // println!()
    println!("{}", header);
    // println!("{:?}", header);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[0];

    let mut contents = Cursor::new(fs::read(file).unwrap());
    // let header = header::HDRR::read(&mut contents).unwrap();
    run(contents)
}
