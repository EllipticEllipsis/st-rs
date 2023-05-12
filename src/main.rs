#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]

mod dense_numbers;
mod file_descriptor;
mod header;
mod procedure_descriptor;
mod symbol;
use std::{env, fmt, fs::File};

use binrw::{io::*, NullString};
use binrw::prelude::*;

// use strum::IntoEnumIterator;
use strum_macros::Display;
use strum_macros::FromRepr;

const indexNil: u32 = 0xFFFFF;
const ST_EXTIFD: u32 = 0x7fffffff; /* ifd for externals */
#[allow(dead_code)]
const ST_RFDESCAPE: u32 = 0xfff; /* rndx.rfd escape says next aux is rfd */
const ST_ANONINDEX: u32 = 0xfffff; /* rndx.index for anonymous names */

fn bits_get(bitfield: u32, width: u32, position: u32) -> u32 {
    (bitfield << position) >> (32 - width)
}

fn run(mut contents: BufReader<File>) {
    let header = header::HDRR::read(&mut contents).unwrap();
    
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
    
    // Procedure descriptor table
    
    let pdr_off = header.cbPdOffset as u64;
    let pdr_count = header.ipdMax;
    let mut i = 0;
    let mut procedure_descriptor_table = Vec::new();

    contents.seek(SeekFrom::Start(pdr_off)).unwrap();
    while i < pdr_count {
        procedure_descriptor_table.push(procedure_descriptor::PDR::read(&mut contents).unwrap());
        i += 1;
    }
    
    
    // File descriptor table
    
    let fd_off = header.cbFdOffset as u64;
    let fd_count = header.ifdMax;
    let mut i = 0;
    let mut file_descriptor_table = Vec::new();

    contents.seek(SeekFrom::Start(fd_off)).unwrap();
    while i < fd_count {
        file_descriptor_table.push(file_descriptor::FDR::read(&mut contents).unwrap());
        i += 1;
    }
    
    
    // Symbols
    
    let syms_off = header.cbSymOffset as u64;
    let syms_count = header.isymMax;
    let mut i = 0;
    let mut local_syms_table = Vec::new();
    
    contents.seek(SeekFrom::Start(syms_off)).unwrap();
    while i < syms_count {
        local_syms_table.push(symbol::SYMR::read(&mut contents).unwrap());
        i += 1;
    }
    
    
    // External Symbols
    
    let ext_off = header.cbExtOffset as u64;
    let ext_count = header.iextMax;
    let mut i = 0;
    let mut external_syms_table = Vec::new();
    
    contents.seek(SeekFrom::Start(ext_off)).unwrap();
    while i < ext_count {
        external_syms_table.push(symbol::EXTR::read(&mut contents).unwrap());
        i += 1;
    }
    
    println!("{}", header);
    for dn in dense_numbers_table {
        println!("{},", dn);
    }
    println!();
    for pdr in procedure_descriptor_table {
        let sym = &local_syms_table[pdr.isym as usize];
        contents.seek(SeekFrom::Start((header.cbSsOffset + sym.iss) as u64)).unwrap();
        let name = contents.read_be::<NullString>().unwrap().to_string();
        print!("{}: ", name);
        println!("{},", pdr);
    }
    println!();
    for fd in file_descriptor_table {
        contents.seek(SeekFrom::Start((header.cbSsOffset + fd.rss) as u64)).unwrap();
        let name = contents.read_be::<NullString>().unwrap().to_string();
        // let sym = &local_syms_table[pdr.isym as usize];
        // contents.seek(SeekFrom::Start((header.cbSsOffset + sym.iss) as u64)).unwrap();
        // let name = contents.read_be::<NullString>().unwrap().to_string();
        print!("{}: ", name);
        println!("{},", fd);
    }
    println!();
    for sym in local_syms_table {
        contents.seek(SeekFrom::Start((header.cbSsOffset + sym.iss) as u64)).unwrap();
        let name = contents.read_be::<NullString>().unwrap().to_string();
        print!("{}: ", name);
        println!("{},", sym);
    }
    println!();
    for sym in external_syms_table {
        contents.seek(SeekFrom::Start((header.cbSsExtOffset + sym.asym.iss) as u64)).unwrap();
        let name = contents.read_be::<NullString>().unwrap().to_string();
        print!("{}: ", name);
        println!("{},", sym);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).unwrap();

    let mut buf_reader = BufReader::new(file);

    buf_reader.rewind().unwrap();
    run(buf_reader)
}
