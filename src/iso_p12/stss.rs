use {BuildNode, Name};

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct Stss {
    entry_count: Option<u32>,
    sample_numbers: Vec<u32>,
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

        let entry_count = Cursor::new(&data[8..12]).read_u32::<BigEndian>().ok();
        let sample_numbers: Vec<u32> = data[12..].chunks(4).map(|val| {
            Cursor::new(val).read_u32::<BigEndian>().unwrap()
        }).collect();

        Some(Stss {
            entry_count,
            sample_numbers,
        })
    }
}
