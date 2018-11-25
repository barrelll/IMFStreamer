use {BuildNode, FullBox, Name};

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct Stsz {
    pub fullbox: Option<FullBox>,
    pub sample_size: Option<u32>,
    pub sample_count: Option<u32>,
    pub entry_sizes: Vec<u32>,
}

impl<'a> Name<'a> for Stsz {
    fn name() -> &'a str {
        "stsz"
    }
}

impl<'a> BuildNode for Stsz {
    fn build(data: &[u8]) -> Option<Self> {
        use byteorder::{BigEndian, ReadBytesExt};
        use std::io::Cursor;
        let fullbox = FullBox::from(&data[8..12]).ok();
        let sample_size = Cursor::new(&data[12..16]).read_u32::<BigEndian>().ok();
        let sample_count = Cursor::new(&data[16..20]).read_u32::<BigEndian>().ok();
        let entry_sizes: Vec<u32> = data[20..]
            .chunks(4)
            .map(|val| {
                Cursor::new(val).read_u32::<BigEndian>().unwrap()
            }).collect();
        Some(Stsz {
            fullbox,
            sample_size,
            sample_count,
            entry_sizes,
        })
    }
}
