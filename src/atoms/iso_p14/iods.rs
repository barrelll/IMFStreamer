use atoms::{BuildNode, IsSlice, Name};

#[derive(Debug, Default, Clone)]
pub struct Iods {
    //    od: ObjectDescriptor;
}

impl<'a> Name<'a> for Iods {
    fn name() -> &'a str {
        "iods"
    }
}

impl BuildNode for Iods {
    fn build<T: IsSlice<Item = u8>>(_data: T) -> Option<Self> {
        Some(Iods {})
    }
}
