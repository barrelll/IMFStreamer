use downcast_rs::Downcast;
use std::fmt::{Debug, Formatter, Result};

pub trait SampleEntry: Downcast {
    fn seclone(&self) -> Box<SampleEntry>;
}

impl_downcast!(SampleEntry);

impl Clone for Box<SampleEntry> {
    fn clone(&self) -> Box<dyn SampleEntry> {
        self.seclone()
    }
}

impl Debug for SampleEntry {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "SampleEntry")
    }
}

pub fn samplefactory(data: &[u8]) -> Vec<Box<SampleEntry>> {
    use byteorder::{BigEndian, ReadBytesExt};
    use std::{io::Cursor, str::from_utf8};
    let ret = Vec::<Box<SampleEntry>>::new();
    let size = Cursor::new(&data[0..4]).read_u32::<BigEndian>().unwrap();
    let name = from_utf8(&data[4..8]);
    println!("{:?} {:?}", size, name);
    ret
}
