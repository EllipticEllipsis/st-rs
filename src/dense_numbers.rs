#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use binrw::prelude::*;
use std::fmt;

/* dense numbers or sometimes called block numbers are stored in this type,
 *	a rfd of 0xffffffff is an index into the global table.
 */
// Size: 0x8
#[derive(BinRead, Debug)]
#[br(big)]
pub struct DNR {
    pub rfd: u32, /* index into the file table */
    pub index: u32, /* index int sym/aux/iss tables */
              // } DNR, *pDNR;
}

impl fmt::Display for DNR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DNR {{ rfd: {:08X}, index: {:5X} }}", self.rfd, self.index)
    }
}
