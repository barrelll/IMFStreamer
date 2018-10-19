use atoms::{build, BuildNode, IsSlice, Name};

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct Stsd {
    sample_entries: Vec<Box<i32>>,
}

impl<'a> Name<'a> for Stsd {
    fn name() -> &'a str {
        "stsd"
    }
}

impl<'a> BuildNode for Stsd {
    fn build<T: IsSlice<Item = u8>>(data: T) -> Option<Self> {
        let _d = data.as_slice();
        let children = build(&_d[16..]);
        println!("stsd children {:?}", children);
        Some(Stsd {
            ..Default::default()
        })
    }
}
