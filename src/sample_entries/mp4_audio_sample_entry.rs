use super::{AudioSampleEntry, SampleBuilder, SampleEntryBase};
use iso_p14::Esds;
use BuildNode;

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct MP4AudioSampleEntry {
    pub audiosample: Option<AudioSampleEntry>,
    pub esds_box: Option<Esds>,
}

impl SampleEntryBase for MP4AudioSampleEntry {
    fn seclone(&self) -> Box<SampleEntryBase> {
        Box::new(self.clone())
    }
    fn name(&self) -> String {
        String::from("MP4AudioSampleEntry")
    }
}

impl SampleBuilder for MP4AudioSampleEntry {
    fn build(data: &[u8]) -> Option<Self> {
        let audiosample = AudioSampleEntry::build(data);
        let esds_box = Esds::build(&data[36..]);
        Some(MP4AudioSampleEntry {
            audiosample,
            esds_box,
        })
    }
}
