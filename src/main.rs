#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

mod dense_numbers;
mod file_descriptor;
mod header;
mod procedure_descriptor;
use std::{env, fmt, fs::File};

use binrw::io::*;
use binrw::prelude::*;

// use strum::IntoEnumIterator;
use strum_macros::Display;
use strum_macros::FromRepr;

fn run(mut contents: BufReader<File>) {
    let header = header::HDRR::read(&mut contents).unwrap();
    println!("{}", header);

    // Dense numbers
    let dn_off = header.cbDnOffset as u64;
    let dn_count = header.idnMax;
    let mut i = 0;
    let mut dense_numbers_table = Vec::new();

    contents.seek(SeekFrom::Start(dn_off)).unwrap();
    while i < dn_count {
        dense_numbers_table.push(dense_numbers::DNR::read(&mut contents).unwrap());
        i += 1;
    }
    for dn in dense_numbers_table {
        println!("{},", dn);
    }

    let pdr_off = header.cbPdOffset as u64;
    let pdr_count = header.ipdMax;
    let mut i = 0;
    let mut procedure_descriptor_table = Vec::new();

    contents.seek(SeekFrom::Start(pdr_off)).unwrap();
    while i < pdr_count {
        procedure_descriptor_table.push(procedure_descriptor::PDR::read(&mut contents).unwrap());
        i += 1;
    }
    for pdr in procedure_descriptor_table {
        println!("{},", pdr);
    }

    let fd_off = header.cbFdOffset as u64;
    let fd_count = header.ifdMax;
    let mut i = 0;
    let mut file_descriptor_table = Vec::new();

    contents.seek(SeekFrom::Start(fd_off)).unwrap();
    while i < fd_count {
        file_descriptor_table.push(file_descriptor::FDR::read(&mut contents).unwrap());
        i += 1;
    }
    for fd in file_descriptor_table {
        println!("{},", fd);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).unwrap();

    let mut buf_reader = BufReader::new(file);

    buf_reader.rewind().unwrap();
    run(buf_reader)
}
