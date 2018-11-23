use {BuildNode, FullBox, Name};

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct Stsc {
    pub fullbox: Option<FullBox>,
    pub entry_count: Option<u32>,
    pub chunk_description: Vec<(u32, u32, u32)>,
}

impl<'a> Name<'a> for Stsc {
    fn name() -> &'a str {
        "stsc"
    }
}

impl BuildNode for Stsc {
    fn build(data: &[u8]) -> Option<Self> {
        use byteorder::{BigEndian, ReadBytesExt};
        use std::io::Cursor;

        let fullbox = FullBox::from(&data[8..12]).ok();
        let entry_count = Cursor::new(&data[12..16]).read_u32::<BigEndian>().ok();
        let sample_numbers: Vec<(u32, u32, u32)> = data[16..]
            .chunks(12)
            .map(|val| {
                let first_chunk = Cursor::new(&val[..4]).read_u32::<BigEndian>().unwrap();
                let samples_per_chunk = Cursor::new(&val[4..8]).read_u32::<BigEndian>().unwrap();
                let sample_description_index =
                    Cursor::new(&val[8..]).read_u32::<BigEndian>().unwrap();
                (first_chunk, samples_per_chunk, sample_description_index)
            }).collect();

        Some(Stsc {
            fullbox,
            entry_count,
            sample_numbers,
        })
    }
}
