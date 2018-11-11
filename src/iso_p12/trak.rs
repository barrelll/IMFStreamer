use {BuildNode, Name};

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct Trak;

impl<'a> Name<'a> for Trak {
    fn name() -> &'a str {
        "trak"
    }
}

impl BuildNode for Trak {
    fn build(_data: &[u8]) -> Option<Self> {
        Some(Trak)
    }
}
