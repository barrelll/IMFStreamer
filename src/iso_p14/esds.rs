use base_descriptors::{DescrBuilder, ESDescriptor};
use {BuildNode, IsSlice, Name};

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct Esds {
    od: Option<ESDescriptor>,
}

impl<'a> Name<'a> for Esds {
    fn name() -> &'a str {
        "esds"
    }
}

impl BuildNode for Esds {
    fn build<T: IsSlice<Item = u8>>(data: T) -> Option<Self> {
        let od = ESDescriptor::build(&data.as_slice()[12..]);
        Some(Esds { od })
    }
}
