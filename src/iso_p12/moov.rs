use {BuildNode, Name};

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
    fn build(data: &[u8]) -> Option<Self> {
        Some(Moov {
            ..Default::default()
        })
    }
}
