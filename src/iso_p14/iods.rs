use base_descriptors::{DescrBuilder, InitialObjectDescriptor};
use {BuildNode, Name};

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct Iods {
    od: Option<InitialObjectDescriptor>,
}

impl<'a> Name<'a> for Iods {
    fn name() -> &'a str {
        "iods"
    }
}

impl BuildNode for Iods {
    fn build(data: &[u8]) -> Option<Self> {
        let od = InitialObjectDescriptor::build(&data[12..]);
        Some(Iods { od })
    }
}
