use super::{SampleEntryBase, VisualSampleEntry};

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
struct MP4VisualSampleEntry {
    visualsample: Option<VisualSampleEntry>,
}

impl SampleEntryBase for MP4VisualSampleEntry {
    fn seclone(&self) -> Box<SampleEntryBase> {
        Box::new(self.clone())
    }
}