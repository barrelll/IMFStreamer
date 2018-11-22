use {BuildNode, Name, FullBox};

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct Stco {
    pub fullbox: Option<FullBox>,
    pub entry_count: Option<u32>,
    pub chunk_offsets: Vec<u32>,
}

impl<'a> Name<'a> for Stco {
    fn name() -> &'a str {
        "stco"
    }
}

impl BuildNode for Stco {
    fn build(data: &[u8]) -> Option<Self> {
        use byteorder::{BigEndian, ReadBytesExt};
        use std::io::Cursor;

        let fullbox = FullBox::from(&data[8..12]).ok();
        let entry_count = Cursor::new(&data[12..16]).read_u32::<BigEndian>().ok();
        let chunk_offsets: Vec<u32> = data[16..].chunks(4).map(|val| {
            Cursor::new(val).read_u32::<BigEndian>().unwrap()
        }).collect();

        Some(Stco {
            fullbox,
            entry_count,
            chunk_offsets,
        })
    }
}
