use crate::*;

#[allow(dead_code)]
const magicSym: i16 = 0x07009;

/*
 * Symbolic Header (HDR) structure.
 * As long as all the pointers are set correctly,
 * we don't care WHAT order the various sections come out in!
 *
 * A file produced solely for the use of CDB will probably NOT have
 * any instructions or data areas in it, as these are available
 * in the original.
 */

// size: 0x60
#[derive(BinRead, Debug)]
#[br(big/* , magic=0x07009i16 */ /* magicSym */)]
pub struct HDRR {
    // typedef struct __sgi_hdrr_s  {
    pub magic: i16,  /* to verify validity of the table */
    pub vstamp: i16, /* version stamp */

    pub ilineMax: i32,     /* number of line number entries */
    pub cbLine: i32,       /* number of bytes for line number entries */
    pub cbLineOffset: i32, /* offset to start of line number entries*/

    pub idnMax: i32,     /* max index into dense number table */
    pub cbDnOffset: i32, /* offset to start dense number table */

    pub ipdMax: i32,     /* number of procedures */
    pub cbPdOffset: i32, /* offset to procedure descriptor table */

    pub isymMax: i32,     /* number of local symbols */
    pub cbSymOffset: i32, /* offset to start of local symbols*/

    pub ioptMax: i32,     /* max index into optimization symbol entries */
    pub cbOptOffset: i32, /* offset to optimization symbol entries */

    pub iauxMax: i32,     /* number of auxillary symbol entries */
    pub cbAuxOffset: i32, /* offset to start of auxillary symbol entries*/

    pub issMax: i32,     /* max index into local strings */
    pub cbSsOffset: i32, /* offset to start of local strings */

    pub issExtMax: i32,     /* max index into external strings */
    pub cbSsExtOffset: i32, /* offset to start of external strings */

    pub ifdMax: i32,     /* number of file descriptor entries */
    pub cbFdOffset: i32, /* offset to file descriptor table */

    pub crfd: i32,        /* number of relative file descriptor entries */
    pub cbRfdOffset: i32, /* offset to relative file descriptor table */

    pub iextMax: i32, /* max index into external symbols */
    pub cbExtOffset: i32, /* offset to start of external symbol entries*/

                      // /* If you add machine dependent fields, add them here */
                      // } HDRR, *pHDRR;
}

impl fmt::Display for HDRR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vstamp_hi = self.vstamp >> 8;
        let vstamp_lo = self.vstamp & 0xFF;

        write!(
            f,
            "HDDR {{
    magic: {:#X}, vstamp: {}.{},
    ilineMax: {}, cbLine: {}, cbLineOffset: {:#X},
    idnMax:    {:5}, cbDnOffset:      {:#6X},
    ipdMax:    {:5}, cbPdOffset:      {:#6X},
    isymMax:   {:5}, cbSymOffset:     {:#6X},
    ioptMax:   {:5}, cbOptOffset:     {:#6X},
    iauxMax:   {:5}, cbAuxOffset:     {:#6X},
    issMax:    {:#5X}, cbSsOffset:      {:#6X},
    issExtMax: {:#5X}, cbSsExtOffset:   {:#6X},
    ifdMax:    {:5}, cbFdOffset:      {:#6X},
    crfd:      {:5}, cbRfdOffset:     {:#6X},
    iextMax:   {:5}, cbExtOffset:     {:#6X}
}}",
            self.magic,
            vstamp_hi,
            vstamp_lo,
            self.ilineMax,
            self.cbLine,
            self.cbLineOffset,
            self.idnMax,
            self.cbDnOffset,
            self.ipdMax,
            self.cbPdOffset,
            self.isymMax,
            self.cbSymOffset,
            self.ioptMax,
            self.cbOptOffset,
            self.iauxMax,
            self.cbAuxOffset,
            self.issMax,
            self.cbSsOffset,
            self.issExtMax,
            self.cbSsExtOffset,
            self.ifdMax,
            self.cbFdOffset,
            self.crfd,
            self.cbRfdOffset,
            self.iextMax,
            self.cbExtOffset,
        )
    }
}
