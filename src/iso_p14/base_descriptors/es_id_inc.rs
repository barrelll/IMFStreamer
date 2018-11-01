use super::{RawDescr, DescrBaseTags};

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct ESIDInc {
    track_id: Option<u32>,
}

impl RawDescr for ESIDInc {
    fn rdclone(&self) -> Box<RawDescr> {
        Box::new(self.clone())
    }
}