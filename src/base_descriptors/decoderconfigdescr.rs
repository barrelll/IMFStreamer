use super::{descrfactory, size_of_instance, DescrBase, DescrBaseTags, DescrBuilder};
use IsSlice;
#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct DecoderConfigDescriptor {
    tag: Option<DescrBaseTags>,
    size_of_instance: Option<u8>,
    objecttypeindication: Option<u8>,
    upstream: Option<bool>,
    reserved: Option<bool>,
    buffersize_db: Option<[bool; 24]>,
    max_bit_rate: Option<u32>,
    avg_bit_rate: Option<u32>,
    descriptors: Vec<Box<dyn DescrBase>>,
}

impl DescrBase for DecoderConfigDescriptor {
    fn tag(&self) -> Option<DescrBaseTags> {
        self.tag.clone()
    }

    fn rdclone(&self) -> Box<DescrBase> {
        Box::new(self.clone())
    }
}

impl DescrBuilder for DecoderConfigDescriptor {
    fn build<T: IsSlice<Item = u8>>(d: T) -> Option<Self> {
        let data = d.as_slice();
        use byteorder::{BigEndian, ReadBytesExt};
        use std::io::Cursor;
        let tag = Some(match Cursor::new(&data[..1])
            .read_u8()
            .expect("DecoderConfigDescriptor error reading tag")
        {
            0x04 => DescrBaseTags::DecoderConfigDescrTag,
            val => {
                panic!(format!("DecoderConfigDescriptor tag doesn't match the object descriptor base tags, {:?}", val));
            }
        });

        let mut cursor: usize = 1;
        let size_of_instance = Some(size_of_instance(data, &mut cursor));
        Some(DecoderConfigDescriptor {
            tag,
            size_of_instance,
            ..Default::default()
        })
    }
}
