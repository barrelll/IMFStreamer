use atom_tree::{BuildNode, IsSlice, Name, SearchFor, Tree};

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
    fn build<T: IsSlice<Item = u8>>(data: T) -> Option<Self> {
        use byteorder::{BigEndian, ReadBytesExt};
        use std::io::Cursor;

        let d = data.as_slice();
        let major_brand = String::from_utf8(d[8..12].to_vec()).ok();
        let minor_version = Cursor::new(&d[12..16]).read_u32::<BigEndian>().ok();
        let minor_brands: Option<Vec<String>> = Some(
            d[16..]
                .to_vec()
                .chunks(4)
                .map(|x| String::from_utf8(x.to_vec()).unwrap())
                .collect(),
        );
        Some(Ftyp {
            major_brand,
            minor_version,
            minor_brands,
        })
    }
}

impl SearchFor for Ftyp {
    fn search(tree: &Tree<&[u8]>) -> Option<Self> {
        let mut ret: Option<Self> = None;
        tree.root.iter().for_each(|x| match x.name {
            Some(val) => {
                if val == Self::name() {
                    ret = Self::build(x.data.expect("Data doesn't exist yet?"));
                }
            }
            None => {
                ret = None;
            }
        });
        ret
    }
}
