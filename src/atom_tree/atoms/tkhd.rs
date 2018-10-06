use atom_tree::{BuildNode, IsSlice, Name};

#[derive(Debug, Default, Clone, Copy)]
pub struct Tkhd {
    track_id: Option<u32>,
}

impl<'a> Name<'a> for Tkhd {
    fn name() -> &'a str {
        "tkhd"
    }
}

impl BuildNode for Tkhd {
    fn build<T: IsSlice<Item = u8>>(data: T) -> Option<Self> {
        let _d = data.as_slice();
        Some(Tkhd {
            ..Default::default()
        })
    }
}
