use {
    sample_entries::{samplefactory, SampleEntry},
    BuildNode, FullBox, IsSlice, Name,
};

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct Stsd {
    fullbox: Option<FullBox>,
    entry_count: Option<u32>,
    sample_entries: Vec<Box<dyn SampleEntry>>,
}

impl<'a> Name<'a> for Stsd {
    fn name() -> &'a str {
        "stsd"
    }
}

impl<'a> BuildNode for Stsd {
    fn build<T: IsSlice<Item = u8>>(data: T) -> Option<Self> {
        let data = data.as_slice();
        use byteorder::{BigEndian, ReadBytesExt};
        use std::io::Cursor;
        // version and flags just before entry count
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
