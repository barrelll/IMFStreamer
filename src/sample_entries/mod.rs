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

pub fn samplefactory(_data: &[u8]) -> Vec<Box<SampleEntry>> {
    let ret = Vec::<Box<SampleEntry>>::new();
    ret
}
