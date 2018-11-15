use {
    sample_entries::{samplefactory, SampleEntryBase},
    BuildNode, FullBox, Name,
};

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct Stsd {
    pub fullbox: Option<FullBox>,
    pub entry_count: Option<u32>,
    pub sample_entries: Vec<Box<dyn SampleEntryBase>>,
}

impl<'a> Name<'a> for Stsd {
    fn name() -> &'a str {
        "stsd"
    }
}

impl<'a> BuildNode for Stsd {
    fn build(data: &[u8]) -> Option<Self> {
        use byteorder::{BigEndian, ReadBytesExt};
        use std::io::Cursor;
        let fullbox = FullBox::from(&data[8..12]).ok();
        let entry_count = Cursor::new(&data[12..16]).read_u32::<BigEndian>().ok();
        let sample_entries = samplefactory(&data[16..]);
        Some(Stsd {
            fullbox,
            entry_count,
            sample_entries,
        })
    }
}
