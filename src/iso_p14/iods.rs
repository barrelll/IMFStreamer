use {BuildNode, IsSlice, Name};
use ::base_descriptors::InitialObjectDescriptor;

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
        for n in data.as_slice() {
            let mut s = format!("{:b}", n);
            if s.len() < 8 {
                for _ in 0..8 - s.len() {
                    s.insert(0, '0');
                }
            }
            print!("{} ", s);
        }
        let od = InitialObjectDescriptor::from_u8_slice(&data.as_slice()[12..]);
        Some(Iods { od })
    }
}
