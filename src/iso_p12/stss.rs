use {BuildNode, Name, FullBox};

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct Stss {
    pub fullbox: Option<FullBox>,
    pub entry_count: Option<u32>,
    pub sample_numbers: Vec<u32>,
}

impl<'a> Name<'a> for Stss {
    fn name() -> &'a str {
        "stss"
    }
}

impl BuildNode for Stss {
    fn build(data: &[u8]) -> Option<Self> {
        use byteorder::{BigEndian, ReadBytesExt};
        use std::io::Cursor;

        let fullbox = FullBox::from(&data[8..12]).ok();
        let entry_count = Cursor::new(&data[12..16]).read_u32::<BigEndian>().ok();
        let sample_numbers: Vec<u32> = data[16..].chunks(4).map(|val| {
            Cursor::new(val).read_u32::<BigEndian>().unwrap()
        }).collect();

        Some(Stss {
            fullbox,
            entry_count,
            sample_numbers,
        })
    }
}
