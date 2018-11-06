use {BuildNode, IsSlice, Name};

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct Stsd {
}

impl<'a> Name<'a> for Stsd {
    fn name() -> &'a str {
        "stsd"
    }
}

impl<'a> BuildNode for Stsd {
    fn build<T: IsSlice<Item = u8>>(data: T) -> Option<Self> {
        let data = data.as_slice();
        use byteorder::{BigEndian, ReadBytesExt};
        use std::io::Cursor;
        let num = Cursor::new(&data[12..16]).read_u32::<BigEndian>();
        println!("{:?}", num);
        Some(Stsd {
            ..Default::default()
        })
    }
}
