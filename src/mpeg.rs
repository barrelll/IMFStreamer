use atom_tree::*;

/*** Mpeg type definition ***/
#[derive(Debug, Default)]
pub struct Mpeg<'a> {
    cached: Option<u32>,
    atom_list: Option<Tree<'a, &'a [u8]>>,
}

impl<'a> Mpeg<'a> {
    pub fn major_brand(&self) -> Option<String> {
        let tree = &self.atom_list;
        let _f = atoms::Ftyp::search(
            tree.as_ref()
                .expect("Mpeg::major_brand (Tree doesn't exist yet)"),
        );
        None
    }

    pub fn minor_brands(&self) -> Option<Vec<String>> {
        None
    }
}

impl<'a> Mpeg<'a> {
    pub fn new(d: &'a [u8]) -> Self {
        let atom_list = build_tree(d);
        atom_list
            .to_owned()
            .unwrap()
            .search_path::<atoms::Tkhd>("moov.trak.tkhd");
        Mpeg {
            atom_list,
            ..Default::default()
        }
    }
}
/*** Mpeg type definition ***/
