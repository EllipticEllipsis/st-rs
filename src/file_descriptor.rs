use crate::*;

/*
* File Descriptor
*
* There is one of these for EVERY FILE, whether compiled with
* full debugging symbols or not.  The name of a file should be
* the path name given to the compiler.	 This allows the user
 * to simply specify the names of the directories where the COMPILES
 * were done, and we will be able to find their files.
 * A field whose comment starts with "R - " indicates that it will be
 * setup at runtime.
 */
// size: 0x48
#[derive(BinRead, Debug)]
#[br(big)]
pub struct FDR {
    // typedef struct fdr {
    pub adr: u32, /* memory address of beginning of file */
    pub rss: i32, /* file name (of source, if known) */

    pub issBase: i32, /* file's string space */
    pub cbSs: i32,    /* number of bytes in the ss */

    pub isymBase: i32, /* beginning of symbols */
    pub csym: i32,     /* count file's of symbols */

    pub ilineBase: i32, /* file's line symbols */
    pub cline: i32,     /* count of file's line symbols */

    pub ioptBase: i32, /* file's optimization entries */
    pub copt: i32,     /* count of file's optimization entries */

    /* the following MUST be unsigned: still
    	** restricts to 64K procedures. sgi */
    pub ipdFirst: u16, /* start of procedures for this file */
    pub cpd: u16,      /* count of procedures for this file */

    pub iauxBase: i32, /* file's auxiliary entries */
    pub caux: i32,     /* count of file's auxiliary entries */

    pub rfdBase: i32, /* index into the file indirect table */
    pub crfd: i32,    /* count file indirect entries */

    pub BITFIELD: u32, // What am I supposed to do with this?
    // unsigned lang: 5;	/* language for this file */
    // unsigned fMerge : 1;	/* whether this file can be merged */
    // unsigned fReadin : 1;	/* true if it was read in (not just created) */
    // unsigned fBigendian : 1;/* if set, was compiled on big endian machine */
    // 			/*	aux's will be in compile host's sex */
    // unsigned glevel : 2;	/* level this file was compiled with */
    // unsigned signedchar : 1; /* whether files was compiled with char being signed */
    // unsigned ipdFirstMSBits: 4; /* upper bits to allow  ipdFirst to
    // 		 exceed 64K entries
    //                      (These are the most significant bits of what is,
    // 		 after concatenating the bits, a 20 bit number) */
    // unsigned cpdMSBits: 4;  /* upper bits to allow cpd to exceed 64K
    // 		 entries
    //                      (These are the most significant bits of what is,
    // 		 after concatenating the bits, a 20 bit number) */
    // unsigned reserved : 13;  /* reserved for future use */
    pub cbLineOffset: i32, /* byte offset from header for this file ln's */
    pub cbLine: i32,       /* size of lines for this file */
                           // } FDR, *pFDR;
}

fn bits_get(bitfield: u32, width: u32, position: u32) -> u32 {
    (bitfield << position) >> (32 - width)
}

#[derive(Debug, Display, PartialEq, FromRepr)]
pub enum lang {
    C,
    Pascal,
    Fortran,
    Assembler,
    Machine,
    Nil,
    Ada,
    Pl1,
    Cobol,
}

// Yes, it really is ordered in this extraordinarily silly way
#[derive(Debug, Display, PartialEq, FromRepr)]
pub enum GLEVEL {
    g2,
    g1,
    g0,
    g3,
}

impl FDR {
    pub fn lang(&self) -> Option<lang> {
        lang::from_repr(bits_get(self.BITFIELD, 5, 0) as usize)
    }
    pub fn fMerge(&self) -> bool {
        bits_get(self.BITFIELD, 1, 5) != 0
    }
    pub fn fReadin(&self) -> bool {
        bits_get(self.BITFIELD, 1, 6) != 0
    }
    pub fn fBigendian(&self) -> bool {
        bits_get(self.BITFIELD, 1, 7) != 0
    }
    pub fn glevel(&self) -> GLEVEL {
        GLEVEL::from_repr(bits_get(self.BITFIELD, 2, 8) as usize).unwrap()
    }
    pub fn signedchar(&self) -> bool {
        bits_get(self.BITFIELD, 1, 10) != 0
    }
    pub fn ipdFirstMSBits(&self) -> u32 {
        bits_get(self.BITFIELD, 4, 11)
    }
    pub fn cpdMSBits(&self) -> u32 {
        bits_get(self.BITFIELD, 4, 15)
    }
    pub fn reserved(&self) -> u32 {
        bits_get(self.BITFIELD, 13, 19)
    }
}

impl fmt::Display for FDR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FDR {{ \
                adr: {:#X}, \
                rss: {}, \
                issBase: {:#X}, \
                cbSs: {:#X}, \
                isymBase: {:#X}, \
                csym: {}, \
                ilineBase: {:#X}, \
                cline: {}, \
                ioptBase: {:#X}, \
                copt: {}, \
                ipdFirst: {:#X}, \
                cpd: {}, \
                iauxBase: {:#X}, \
                caux: {}, \
                rfdBase: {:#X}, \
                crfd: {}, \
                BITFIELD: {:#034b}, \
                cbLineOffset: {:#X}, \
                cbLine: {:#X} \
            }}\n\
            BITFIELD: {{ \
                lang: {}, fMerge: {}, fReadin: {}, fBigendian: {}, glevel: {}, signedchar: {}, ipdFirstMSBits: {:#X}, cpdMSBits: {:#X}, reserved: {:#015b} \
            }}",
            self.adr,
            self.rss,
            self.issBase,
            self.cbSs,
            self.isymBase,
            self.csym,
            self.ilineBase,
            self.cline,
            self.ioptBase,
            self.copt,
            self.ipdFirst,
            self.cpd,
            self.iauxBase,
            self.caux,
            self.rfdBase,
            self.crfd,
            self.BITFIELD,
            self.cbLineOffset,
            self.cbLine,
            self.lang().unwrap(),
            self.fMerge(),
            self.fReadin(),
            self.fBigendian(),
            self.glevel(),
            self.signedchar(),
            self.ipdFirstMSBits(),
            self.cpdMSBits(),
            self.reserved()
        )
    }
}
