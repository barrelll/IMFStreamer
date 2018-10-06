use atoms::{BuildNode, IsSlice, Name};

#[derive(Debug, Default, Clone)]
pub struct Trak;

impl<'a> Name<'a> for Trak {
    fn name() -> &'a str {
        "trak"
    }
}

impl BuildNode for Trak {
    fn build<T: IsSlice<Item = u8>>(data: T) -> Option<Self> {
        let _d = data.as_slice();
        Some(Trak)
    }
}
