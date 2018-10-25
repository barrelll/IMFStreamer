use {BuildNode, IsSlice, Name};

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct Stsd {
    sample_entries: Vec<i32>,
}

impl<'a> Name<'a> for Stsd {
    fn name() -> &'a str {
        "stsd"
    }
}

impl<'a> BuildNode for Stsd {
    fn build<T: IsSlice<Item = u8>>(data: T) -> Option<Self> {
        let _d = data.as_slice();
        Some(Stsd {
            ..Default::default()
        })
    }
}
