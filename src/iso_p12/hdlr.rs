use {BuildNode, FullBox, Name};

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct Hdlr {
    pub fullbox: Option<FullBox>,
    pub pre_defined: Option<u32>,
    pub handler_type: Option<String>,
    pub reserved: Option<[u32; 3]>,
    pub name: Option<String>,
}

impl<'a> Name<'a> for Hdlr {
    fn name() -> &'a str {
        "hdlr"
    }
}

impl<'a> BuildNode for Hdlr {
    fn build(data: &[u8]) -> Option<Self> {
        use byteorder::{BigEndian, ReadBytesExt};
        use std::io::Cursor;
        let fullbox = FullBox::from(&data[8..12]).ok();
        let pre_defined = Cursor::new(&data[12..16]).read_u32::<BigEndian>().ok();
        let handler_type = String::from_utf8(data[16..20].to_vec()).ok();
        let reserved = Some([
            Cursor::new(&data[20..24]).read_u32::<BigEndian>().unwrap(),
            Cursor::new(&data[24..28]).read_u32::<BigEndian>().unwrap(),
            Cursor::new(&data[28..32]).read_u32::<BigEndian>().unwrap(),
        ]);
        let name = String::from_utf8(data[32..].to_vec()).ok();
        Some(Hdlr {
            fullbox,
            pre_defined,
            handler_type,
            reserved,
            name,
        })
    }
}
