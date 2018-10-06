use atoms::{build, BuildNode, IsSlice, Name};

#[derive(Debug, Default, Clone, Copy)]
pub struct Stsd;

impl<'a> Name<'a> for Stsd {
    fn name() -> &'a str {
        "stsd"
    }
}

impl BuildNode for Stsd {
    fn build<T: IsSlice<Item = u8>>(data: T) -> Option<Self> {
        let _d = data.as_slice();
        let children = build(&_d[16..]);
        println!("stsd children {:?}", children);
        Some(Stsd)
    }
}
