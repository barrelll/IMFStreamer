use super::{DescrBase, DescrBaseTags, DescrBuilder};
use IsSlice;

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct ESDescriptor {
    tag: Option<DescrBaseTags>,
    size_of_instance: Option<u8>,
    es_id: Option<u16>,
    stream_dependence_flag: Option<bool>,
    url_flag: Option<bool>,
    ocr_stream_flag: Option<bool>,
    strean_priority: Option<[bool; 5]>,
    depends_on_es_id: Option<u16>,
    url_length: Option<u8>,
    url_string: Option<String>,
    ocr_es_id: Option<u16>,
    descriptors: Vec<Box<dyn DescrBase>>,
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
    fn build<T: IsSlice<Item = u8>>(d: T) -> Option<Self> {
        let data = d.as_slice();
        use byteorder::ReadBytesExt;
        use std::io::Cursor;
        let tag = Some(match Cursor::new(&data[..1])
            .read_u8()
            .expect("ESIDInc error reading tag")
        {
            0x0E => DescrBaseTags::ESIDIncTag,
            _ => {
                panic!("ESIDInc descriptor tag doesn't match the object descriptor base tags");
            }
        });
        Some(ESDescriptor{
            tag,
            ..Default::default()
        })
    }
}
