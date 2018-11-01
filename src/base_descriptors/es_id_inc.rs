use super::{DescrBase, DescrBuilder};
use IsSlice;

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct ESIDInc {
    track_id: Option<u32>,
}

impl DescrBase for ESIDInc {
    fn rdclone(&self) -> Box<DescrBase> {
        Box::new(self.clone())
    }
}

impl DescrBuilder for ESIDInc {
    fn build<T: IsSlice<Item = u8>>(_d: T) -> Option<Self> {
        Some(ESIDInc { track_id: Some(0) })
    }
}
