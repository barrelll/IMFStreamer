use super::{DescrBase, DescrBaseTags, DescrBuilder};

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct SlConfigDescr {
    pub tag: Option<DescrBaseTags>,
}

impl DescrBase for SlConfigDescr {
    fn tag(&self) -> Option<DescrBaseTags> {
        self.tag.clone()
    }

    fn rdclone(&self) -> Box<DescrBase> {
        Box::new(self.clone())
    }
}

impl DescrBuilder for SlConfigDescr {
    fn build(data: &[u8]) -> Option<Self> {
        use byteorder::ReadBytesExt;
        use std::io::Cursor;
        let mut cursor = Cursor::new(data);
        let tag = Some(match cursor
            .read_u8()
            .expect("SlConfigDescr error reading tag")
        {
            0x06 => DescrBaseTags::SLConfigDescrTag,
            _ => {
                panic!("SlConfigDescr descriptor tag doesn't match the object descriptor base tags");
            }
        });

        let predefined = cursor.read_u8().ok();
        match predefined {
            Some(val) => {
                let byte = cursor.read_u8();
                if val == 0 {} else {}
            }
            None => return None
        }
        Some(SlConfigDescr {
            tag
        })
    }
}
