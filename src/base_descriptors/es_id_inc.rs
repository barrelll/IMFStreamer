use super::{size_of_instance, DescrBase, DescrBaseTags, DescrBuilder};
use IsSlice;

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct ESIDInc {
    tag: Option<DescrBaseTags>,
    size_of_instance: Option<u8>,
    track_id: Option<u32>,
}

impl DescrBase for ESIDInc {
    fn tag(&self) -> Option<DescrBaseTags> {
        self.tag.clone()
    }

    fn rdclone(&self) -> Box<DescrBase> {
        Box::new(self.clone())
    }
}

impl DescrBuilder for ESIDInc {
    fn build<T: IsSlice<Item = u8>>(d: T) -> Option<Self> {
        let data = d.as_slice();
        use byteorder::{BigEndian, ReadBytesExt};
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
        let mut cursor = 1;
        let size_of_instance = Some(size_of_instance(&data, &mut cursor));
        let track_id = Cursor::new(&data[cursor..cursor + 4])
            .read_u32::<BigEndian>()
            .ok();
        Some(ESIDInc {
            tag,
            size_of_instance,
            track_id,
        })
    }
}
