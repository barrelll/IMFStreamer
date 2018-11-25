use base_descriptors::{DescrBuilder, ESDescriptor};
use {BuildNode, FullBox, Name};

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct Esds {
    pub fullbox: Option<FullBox>,
    pub od: Option<ESDescriptor>,
}

impl<'a> Name<'a> for Esds {
    fn name() -> &'a str {
        "esds"
    }
}

impl BuildNode for Esds {
    fn build(data: &[u8]) -> Option<Self> {
        let fullbox = FullBox::from(&data[8..12]).ok();
        let od = ESDescriptor::build(&data[12..]);
        Some(Esds { fullbox, od })
    }
}
