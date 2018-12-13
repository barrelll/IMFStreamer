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

impl SlConfigDescr {
    pub fn build_specdecinfo(&self, _object_identifier: u8) -> SlConfigDescr {
        SlConfigDescr {
            ..Default::default()
        }
    }
}

impl DescrBuilder for SlConfigDescr {
    fn build(data: &[u8]) -> Option<Self> {
        use byteorder::ReadBytesExt;
        use std::io::Cursor;
        let tag = Some(match Cursor::new(&data[..1])
            .read_u8()
            .expect("SlConfigDescr error reading tag")
        {
            0x06 => DescrBaseTags::SLConfigDescrTag,
            _ => {
                panic!("SlConfigDescr descriptor tag doesn't match the object descriptor base tags");
            }
        });
        Some(SlConfigDescr {
            tag
        })
    }
}
