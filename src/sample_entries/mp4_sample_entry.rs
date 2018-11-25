use super::{SampleEntry, SampleBuilder, SampleEntryBase};
use iso_p14::Esds;
use BuildNode;

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct MP4SampleEntry {
    pub sample_entry: Option<SampleEntry>,
    pub esds_box: Option<Esds>,
}

impl SampleEntryBase for MP4SampleEntry {
    fn seclone(&self) -> Box<SampleEntryBase> {
        Box::new(self.clone())
    }
    fn name(&self) -> String {
        String::from("MP4SampleEntry")
    }
}

impl SampleBuilder for MP4SampleEntry {
    fn build(data: &[u8]) -> Option<Self> {
        let sample_entry = SampleEntry::build(data);
        let esds_box = Esds::build(&data[16..]);
        Some(MP4SampleEntry {
            sample_entry,
            esds_box,
        })
    }
}
