use {BuildNode, Name, FullBox};

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct Co64 {
    pub fullbox: Option<FullBox>,
    pub entry_count: Option<u32>,
    pub chunk_offsets: Vec<u64>,
}

impl<'a> Name<'a> for Co64 {
    fn name() -> &'a str {
        "co64"
    }
}

impl BuildNode for Co64 {
    fn build(data: &[u8]) -> Option<Self> {
        use byteorder::{BigEndian, ReadBytesExt};
        use std::io::Cursor;

        let fullbox = FullBox::from(&data[8..12]).ok();
        let entry_count = Cursor::new(&data[12..16]).read_u32::<BigEndian>().ok();
        let chunk_offsets: Vec<u64> = data[16..].chunks(8).map(|val| {
            Cursor::new(val).read_u64::<BigEndian>().unwrap()
        }).collect();

        Some(Co64 {
            fullbox,
            entry_count,
            chunk_offsets,
        })
    }
}
