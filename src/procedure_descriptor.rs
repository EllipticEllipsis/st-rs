use crate::*;

/*
 * Procedure Descriptor
 *
 * There is one of these for EVERY TEXT LABEL.
 * If a procedure is in a file with full symbols, then isym
 * will point to the PROC symbols, else it will point to the
 * global symbol for the label.
 */
// size: 0x34
#[derive(BinRead, Debug)]
#[br(big)]
pub struct PDR {
    //  typedef struct pdr {
    pub adr: u32,         /* memory address of start of procedure */
    pub isym: i32,        /* start of local symbol entries */
    pub iline: i32,       /* start of line number entries*/
    pub regmask: i32,     /* save register mask */
    pub regoffset: i32,   /* save register offset */
    pub iopt: i32,        /* start of optimization symbol entries*/
    pub fregmask: i32,    /* save floating point register mask */
    pub fregoffset: i32,  /* save floating point register offset */
    pub frameoffset: i32, /* frame size */
    pub framereg: i16,    /* frame pointer register */
    pub pcreg: i16,       /* offset or reg of return pc */
    pub lnLow: i32,       /* lowest line in the procedure */
    pub lnHigh: i32,      /* highest line in the procedure */
    pub cbLineOffset: i32, /* byte offset for this procedure from the fd base */
                          // } PDR, *pPDR;
}

impl fmt::Display for PDR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PDR {{ adr: {}, \
            isym: {}, \
            iline: {}, \
            regmask: {:08X}, \
            regoffset: {:X}, \
            iopt: {}, \
            fregmask: {:08X}, \
            fregoffset: {:X}, \
            frameoffset: {:X}, \
            framereg: {}, \
            pcreg: {}, \
            lnLow: {}, \
            lnHigh: {}, \
            cbLineOffset: {} \
         }}",
            self.adr,
            self.isym,
            self.iline,
            self.regmask,
            self.regoffset,
            self.iopt,
            self.fregmask,
            self.fregoffset,
            self.frameoffset,
            self.framereg,
            self.pcreg,
            self.lnLow,
            self.lnHigh,
            self.cbLineOffset
        )
    }
}
