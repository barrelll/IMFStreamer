use super::{SampleEntryBase, VisualSampleEntry};
use ::iso_p14::Esds;

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
struct MP4VisualSampleEntry {
    visualsample: Option<VisualSampleEntry>,
    esds_box: Option<Esds>,
}

impl SampleEntryBase for MP4VisualSampleEntry {
    fn seclone(&self) -> Box<SampleEntryBase> {
        Box::new(self.clone())
    }
}