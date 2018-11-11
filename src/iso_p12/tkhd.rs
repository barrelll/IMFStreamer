use {BuildNode, Name};

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct Tkhd {
    track_id: Option<u32>,
}

impl<'a> Name<'a> for Tkhd {
    fn name() -> &'a str {
        "tkhd"
    }
}

impl BuildNode for Tkhd {
    fn build(data: &[u8]) -> Option<Self> {
        Some(Tkhd {
            ..Default::default()
        })
    }
}
