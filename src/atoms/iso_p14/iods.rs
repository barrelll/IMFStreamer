use atoms::{BuildNode, IsSlice, Name};

#[derive(Debug, Default, Clone)]
pub struct Iods {
}

impl<'a> Name<'a> for Iods {
    fn name() -> &'a str {
        "iods"
    }
}

impl BuildNode for Iods {
    fn build<T: IsSlice<Item = u8>>(data: T) -> Option<Self> {
        Some(Iods {})
    }
}
