use atom_tree::{BuildNode, Name, SearchFor, Tree, IsSlice};

#[derive(Debug, Clone, Copy)]
pub struct Ftyp;

impl<'a> Name<'a> for Ftyp {
    fn name() -> &'a str {
        "ftyp"
    }
}

impl BuildNode for Ftyp {
    fn build<T: IsSlice<Item=u8>>(_data: T) -> Option<Self> {
        let d = _data.as_slice();
        println!("{:?}", &d[0..4]);
        Some(Ftyp)
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
