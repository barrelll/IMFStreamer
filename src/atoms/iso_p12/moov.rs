use atoms::{BuildNode, IsSlice, Name};

#[derive(Debug, Default, Clone)]
pub struct Moov {
    track_id: Option<u32>,
}

impl<'a> Name<'a> for Moov {
    fn name() -> &'a str {
        "moov"
    }
}

impl BuildNode for Moov {
    fn build<T: IsSlice<Item = u8>>(data: T) -> Option<Self> {
        let _d = data.as_slice();
        Some(Moov {
            ..Default::default()
        })
    }
}
