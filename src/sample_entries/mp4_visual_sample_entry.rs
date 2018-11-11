use super::{SampleBuilder, SampleEntryBase, VisualSampleEntry};
use iso_p14::Esds;
use BuildNode;

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct MP4VisualSampleEntry {
    visualsample: Option<VisualSampleEntry>,
    esds_box: Option<Esds>,
}

impl SampleEntryBase for MP4VisualSampleEntry {
    fn seclone(&self) -> Box<SampleEntryBase> {
        Box::new(self.clone())
    }
}

impl SampleBuilder for MP4VisualSampleEntry {
    fn build(data: &[u8]) -> Option<Self> {
        let visualsample = VisualSampleEntry::build(data);
        let esds_box = Esds::build(&data[86..]);
        Some(MP4VisualSampleEntry {
            visualsample,
            esds_box,
        })
    }
}
