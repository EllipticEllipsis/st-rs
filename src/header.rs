#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use binrw::prelude::*;
use std::fmt;

const magicSym: i16 = 0x07009;

#[derive(BinRead, Debug)]
#[br(big/* , magic=0x07009i16 */ /* magicSym */)]
pub struct HDRR {
    // typedef struct __sgi_hdrr_s  {
    magic: i16,         /* to verify validity of the table */
    vstamp: i16,        /* version stamp */
    ilineMax: i32,      /* number of line number entries */
    cbLine: i32,        /* number of bytes for line number entries */
    cbLineOffset: i32,  /* offset to start of line number entries*/
    idnMax: i32,        /* max index into dense number table */
    cbDnOffset: i32,    /* offset to start dense number table */
    ipdMax: i32,        /* number of procedures */
    cbPdOffset: i32,    /* offset to procedure descriptor table */
    isymMax: i32,       /* number of local symbols */
    cbSymOffset: i32,   /* offset to start of local symbols*/
    ioptMax: i32,       /* max index into optimization symbol entries */
    cbOptOffset: i32,   /* offset to optimization symbol entries */
    iauxMax: i32,       /* number of auxillary symbol entries */
    cbAuxOffset: i32,   /* offset to start of auxillary symbol entries*/
    issMax: i32,        /* max index into local strings */
    cbSsOffset: i32,    /* offset to start of local strings */
    issExtMax: i32,     /* max index into external strings */
    cbSsExtOffset: i32, /* offset to start of external strings */
    ifdMax: i32,        /* number of file descriptor entries */
    cbFdOffset: i32,    /* offset to file descriptor table */
    crfd: i32,          /* number of relative file descriptor entries */
    cbRfdOffset: i32,   /* offset to relative file descriptor table */
    iextMax: i32,       /* max index into external symbols */
    cbExtOffset: i32,   /* offset to start of external symbol entries*/

                        // /* If you add machine dependent fields, add them here */
                        // } HDRR, *pHDRR;
}

impl fmt::Display for HDRR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "magic: {:X}, vstamp: {:X}", self.magic, self.vstamp)
    }
}
