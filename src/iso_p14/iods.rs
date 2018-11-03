use base_descriptors::{DescrBuilder, InitialObjectDescriptor};
use {BuildNode, IsSlice, Name};

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct Iods {
    od: Option<InitialObjectDescriptor>,
}

impl<'a> Name<'a> for Iods {
    fn name() -> &'a str {
        "iods"
    }
}

impl BuildNode for Iods {
    fn build<T: IsSlice<Item = u8>>(data: T) -> Option<Self> {
        //                for n in data.as_slice() {
        //                    let mut s = format!("{:b}", n);
        //                    if s.len() < 8 {
        //                        for _ in 0..8 - s.len() {
        //                            s.insert(0, '0');
        //                        }
        //                    }
        //                    print!("{} ", s);
        //                }
        //                println!("");
        let od = InitialObjectDescriptor::build(&data.as_slice()[12..]);
        Some(Iods { od })
    }
}
