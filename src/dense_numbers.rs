use crate::*;

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
        let rfd_str = if self.rfd == ST_EXTIFD {
            "ST_EXTIFD".to_string()
        } else {
            format!("{:9}", self.rfd)
        };
        let index_str = if self.index == ST_ANONINDEX {
            "ST_ANONINDEX".to_string()
        } else {
            format!("{:12}", self.index)
        };
        write!(f, "DNR {{ rfd: {}, index: {} }}", rfd_str, index_str)
    }
}
