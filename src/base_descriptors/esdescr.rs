use super::{DescrBase, DescrBaseTags, DescrBuilder};
use IsSlice;

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct ESDescriptor {
    tag: Option<DescrBaseTags>,
    es_id: Option<u16>,
    stream_dependence_flag: Option<bool>,
    url_flag: Option<bool>,
    ocr_stream_flag: Option<bool>,
    strean_priority: Option<[bool; 5]>,
    depends_on_es_id: Option<u16>,
    url_length: Option<u8>,
    url_string: Option<String>,
}

impl DescrBase for ESDescriptor {
    fn tag(&self) -> Option<DescrBaseTags> {
        self.tag.clone()
    }

    fn rdclone(&self) -> Box<DescrBase> {
        Box::new(self.clone())
    }
}

impl DescrBuilder for ESDescriptor {
    fn build<T: IsSlice<Item = u8>>(_d: T) -> Option<Self> {
        None
    }
}
