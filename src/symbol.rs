use crate::*;

/*
 * The Symbol Structure        (GFW, to those who Know!)
 */
// size: 0xC
#[derive(BinRead, Debug)]
#[br(big)]
pub struct SYMR {
    // typedef struct __sgi_symr_s {
    pub iss: i32,   /* index into String Space of name */
    pub value: i32, /* value of symbol */
    pub BITFIELD: u32,
    //    unsigned st : 6;    /* symbol type */
    //    unsigned sc  : 5;    /* storage class - text, data, etc */
    //    unsigned reserved : 1;    /* reserved */
    //    unsigned index : 20;    /* index into sym/aux table */
    //    } SYMR, *pSYMR;
}

/*
 * Storage Classes
 */
#[derive(Debug, Display, PartialEq, FromRepr)]
pub enum sc {
    Nil,       // 0
    Text,      // 1    /* text symbol */
    Data,      // 2    /* initialized data symbol */
    Bss,       // 3    /* un-initialized data symbol */
    Register,  // 4    /* value of symbol is register number */
    Abs,       // 5    /* value of symbol is absolute */
    Undefined, // 6    /* who knows? */
    CdbLocal,  // 7    /* variable's value is IN se->va.?? */
    Bits,      // 8    /* this is a bit field */
    CdbSystem, // 9    /* variable's value is IN CDB's address space */
    // Dbx,         // 9    /* overlap dbx internal use */
    RegImage,    // 10    /* register value saved on stack */
    Info,        // 11    /* symbol contains debugger information */
    UserStruct,  // 12    /* address in struct user for current process */
    SData,       // 13    /* load time only small data */
    SBss,        // 14    /* load time only small common */
    RData,       // 15    /* load time only read only data */
    Var,         // 16    /* Var parameter (fortran,pascal) */
    Common,      // 17    /* common variable */
    SCommon,     // 18    /* small common */
    VarRegister, // 19    /* Var parameter in a register */
    Variant,     // 20    /* Variant record */
    SUndefined,  // 21    /* small undefined(external) data */
    Init,        // 22    /* .init section symbol */
    BasedVar,    // 23    /* Fortran or PL/1 ptr based var */
    XData,       // 24    /* exception handling data */
    PData,       // 25    /* Procedure section */
    Fini,        // 26    /* .fini section */
    // sgi
    NonGP, // 27      /* don't put this symbol in GP region */
           // C++ extras omitted
}

/*
 *   Symbol Types
 */
#[derive(Debug, Display, PartialEq, FromRepr)]
pub enum st {
    Nil,        // 0       /* Nuthin' special */
    Global,     // 1       /* external symbol */
    Static,     // 2       /* static */
    Param,      // 3       /* procedure argument */
    Local,      // 4       /* local variable */
    Label,      // 5       /* label */
    Proc,       // 6       /*     "      "         Procedure */
    Block,      // 7       /* beginnning of block */
    End,        // 8       /* end (of anything) */
    Member,     // 9       /* member (of anything        - struct/union/enum */
    Typedef,    // 10      /* type definition */
    File,       // 11      /* file name */
    RegReloc,   // 12      /* register relocation */
    Forward,    // 13      /* forwarding address */
    StaticProc, // 14      /* load time only static procs */
    Constant,   // 15      /* const */
    StaParam,   // 16      /* Fortran static parameters */
    Base,       // 17      /* DEC */
    Tag,        // 18      /*  DEC */
    AdjMember,  // 19      /*  DEC INTERLUDE */
    Public,     // 20      /* public access divider */
    Protected,  // 21      /* protected access divider */
    Private,    // 22      /* private access divider */
    Temp,       // 23      /* template */
    TempProc,   // 24      /* template function */
    DefArg,     // 25      /* default argument */

    // sgi
    Struct, // 26      /* begin Struct kind of stBlock */
    Union,  // 27      /* begin Union kind of stBlock */
    Enum,   // 28      /* begin Enum  kind of stBlock */
    // end sgi
    Vtbl,        // 29        /* virtual table */
    QMember,     // 30        /* qualified member */
    DeltaReloc,  // 31        /* delta relocation symbol */
    CDeltaReloc, // 32        /* delta relocation symbol */
    MemberProc,  // 33        /* member function */

    Indirect, // 34        /* Indirect type specification */
    /* Next two entries are used by the Workshop Interpreter */
    StaticIdx, // 35      /* indirection table index for static var */
    StaticProcIdx, // 36      /* indirection table index for static func */

               /* Psuedo-symbols - internal to debugger */
               // Need a way to skip to these in the enum
               // Str,    // 60        /* string */
               // Number, // 61        /* pure number (ie. 4 NOR 2+2) */
               // Expr,   // 62        /* 2+2 vs. 4 */
               // Type,   // 63        /* post-coersion SER */
}

impl SYMR {
    pub fn st(&self) -> Option<st> {
        st::from_repr(bits_get(self.BITFIELD, 6, 0) as usize)
    }
    pub fn sc(&self) -> Option<sc> {
        sc::from_repr(bits_get(self.BITFIELD, 5, 6) as usize)
    }
    pub fn reserved(&self) -> u32 {
        bits_get(self.BITFIELD, 1, 11)
    }
    pub fn index(&self) -> u32 {
        bits_get(self.BITFIELD, 20, 12)
    }
}

impl fmt::Display for SYMR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let index_str = if self.index() == indexNil {
            "indexNil".to_string()
        } else {
            format!("{}", self.index())
        };
        write!(
            f,
            "SYM {{ iss: {}, value: {}, BITFIELD: {:#034b} }}
    BITFIELD: {{ st: {}, sc: {}, reserved: {}, index: {} }}",
            self.iss,
            self.value,
            self.BITFIELD,
            self.st().unwrap(),
            self.sc().unwrap(),
            self.reserved(),
            index_str
        )
    }
}

/* E X T E R N A L   S Y M B O L  R E C O R D
 *
 *	Same as the SYMR except it contains file context to determine where
 *	the index is.
 */
// size: 0x10
#[derive(BinRead, Debug)]
#[br(big)]
pub struct EXTR {
    // typedef struct __sgi_extr__ {
    pub BITFIELD: u16,
    // unsigned jmptbl:1;	/* symbol is a jump table entry for shlibs */
    // unsigned cobol_main:1;	/* symbol is a cobol main procedure */
    // unsigned weakext:1;	/* symbol is weak external */
    // unsigned deltacplus:1;	/* symbol is delta C++ symbol */
    // unsigned multiext:1;	/* symbol may be defined multiple times */
    // unsigned reserved:11;	/* reserved for future use */
    pub ifd: i16, /* where the iss and index fields point into */
    pub asym: SYMR, /* symbol for the external */
              // } EXTR, *pEXTR;
}

fn widen(bitfield: u16) -> u32 {
    (bitfield as u32) << 0x10
}

impl EXTR {
    fn jmptbl(&self) -> bool {
        bits_get(widen(self.BITFIELD), 1, 0) != 0
    }
    fn cobol_main(&self) -> bool {
        bits_get(widen(self.BITFIELD), 1, 1) != 0
    }
    fn weakext(&self) -> bool {
        bits_get(widen(self.BITFIELD), 1, 2) != 0
    }
    fn deltacplus(&self) -> bool {
        bits_get(widen(self.BITFIELD), 1, 3) != 0
    }
    fn multiext(&self) -> bool {
        bits_get(widen(self.BITFIELD), 1, 4) != 0
    }
    fn reserved(&self) -> u32 {
        bits_get(widen(self.BITFIELD), 11, 5)
    }
}

impl fmt::Display for EXTR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "EXTR {{
    BITFIELD: {{ jmptbl: {}, cobol_main: {}, weakext: {}, deltacplus: {}, multiext: {}, reserved: {} }},
    ifd: {},
    {}, 
}}",
            self.jmptbl(),
            self.cobol_main(),
            self.weakext(),
            self.deltacplus(),
            self.multiext(),
            self.reserved(),
            self.ifd,
            self.asym
        )
    }
}
