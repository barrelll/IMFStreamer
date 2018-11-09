use super::{descrfactory, size_of_instance, DescrBase, DescrBaseTags, DescrBuilder};
use IsSlice;

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct DecoderSpecificInfo {
    tag: Option<DescrBaseTags>,
    size_of_instance: Option<u8>,
}

impl DescrBase for DecoderSpecificInfo {
    fn tag(&self) -> Option<DescrBaseTags> {
        self.tag.clone()
    }

    fn rdclone(&self) -> Box<DescrBase> {
        Box::new(self.clone())
    }
}

impl DescrBuilder for DecoderSpecificInfo {
    fn build<T: IsSlice<Item = u8>>(d: T) -> Option<Self> {
        None
    }
}
