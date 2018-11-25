use super::{SampleBuilder, SampleEntryBase, AudioSampleEntry};
use iso_p14::Esds;
use BuildNode;

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct MP4AudioSampleEntry {
    pub visualsample: Option<AudioSampleEntry>,
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
        let visualsample = AudioSampleEntry::build(data);
        let esds_box: Option<Esds> = Esds::build(&data[36..]);
        Some(MP4AudioSampleEntry {
            visualsample,
            esds_box,
        })
    }
}
