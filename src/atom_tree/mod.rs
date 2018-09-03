mod atoms;
mod atom_error;

use std::str;

//use self::atoms::*;
pub use self::atom_error::AtomError;

/*** Atom Data type implementation ***/
pub trait AtomData {
    fn name(&self) -> Result<&str, AtomError>;
    fn size(&self) -> Result<usize, AtomError>;
}

impl<'a> AtomData for &'a [u8] {
    fn name(&self) -> Result<&str, AtomError> {
        Ok(str::from_utf8(&self[4..8])?)
    }

    fn size(&self) -> Result<usize, AtomError> {
        use byteorder::{BigEndian, ReadBytesExt};
        use std::io::Cursor;

        let size_flag = Cursor::new(&self[..4]).read_u32::<BigEndian>()?;
        let actual_size = match size_flag {
            0 => return Err(AtomError::EOFError),
            1 => Cursor::new(&self[8..16]).read_u64::<BigEndian>()? as usize,
            val => val as usize,
        };
        Ok(actual_size)
    }
}
/*** Atom Data type implementation ***/
