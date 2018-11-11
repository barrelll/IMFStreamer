use {BuildNode, Name};

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct Ftyp {
    pub major_brand: Option<String>,
    pub minor_version: Option<u32>,
    pub minor_brands: Option<Vec<String>>,
}

impl<'a> Name<'a> for Ftyp {
    fn name() -> &'a str {
        "ftyp"
    }
}

impl BuildNode for Ftyp {
    fn build(data: &[u8]) -> Option<Self> {
        use byteorder::{BigEndian, ReadBytesExt};
        use std::io::Cursor;

        let err = "Ftyp can't parse minor brands";
        let major_brand = String::from_utf8(data[8..12].to_vec()).ok();
        let minor_version = Cursor::new(&data[12..16]).read_u32::<BigEndian>().ok();
        let minor_brands: Option<Vec<String>> = Some(
            data[16..]
                .to_vec()
                .chunks(4)
                .map(|x| String::from_utf8(x.to_vec()).expect(err))
                .collect(),
        );
        Some(Ftyp {
            major_brand,
            minor_version,
            minor_brands,
        })
    }
}
