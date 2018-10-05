use atom_tree::{BuildNode, IsSlice, Name, SearchFor, Tree};

#[derive(Debug, Clone, Copy)]
pub struct Tkhd;

impl<'a> Name<'a> for Tkhd {
    fn name() -> &'a str {
        "tkhd"
    }
}

impl BuildNode for Tkhd {
    fn build<T: IsSlice<Item = u8>>(_data: T) -> Option<Self> {
        use std::str::from_utf8;
        let d = _data.as_slice();
        println!("{:?}", from_utf8(&d[4..8]));
        Some(Tkhd)
    }
}

impl SearchFor for Tkhd {
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
