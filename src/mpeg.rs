use atom_tree::*;

/*** Mpeg type definition ***/
#[derive(Debug, Default)]
pub struct Mpeg<'a> {
    atom_list: Option<Tree<'a, &'a [u8]>>,
}

impl<'a> Mpeg<'a> {
    pub fn new(d: &'a [u8]) -> Self {
        let atom_list = build_tree(d);
        Mpeg {
            atom_list,
            ..Default::default()
        }
    }
}
/*** Mpeg type definition ***/
