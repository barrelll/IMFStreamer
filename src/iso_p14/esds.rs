use base_descriptors::{DescrBuilder, ESDescriptor};
use {BuildNode, Name};

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct Esds {
    pub od: Option<ESDescriptor>,
}

impl<'a> Name<'a> for Esds {
    fn name() -> &'a str {
        "esds"
    }
}

impl BuildNode for Esds {
    fn build(data: &[u8]) -> Option<Self> {
        let od = ESDescriptor::build(&data[12..]);
        Some(Esds { od })
    }
}
