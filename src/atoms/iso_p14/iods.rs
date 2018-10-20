use atoms::{iso_p14::base_descriptors::InitialObjectDescriptor, BuildNode, IsSlice, Name};

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
    fn build<T: IsSlice<Item = u8>>(data: T) -> Option<Self> {
        let od = InitialObjectDescriptor::from_u8_slice(&data.as_slice()[12..]);
        Some(Iods { od })
    }
}
