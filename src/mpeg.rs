use atom_tree::*;

/*** Mpeg type definition ***/
#[derive(Debug, Default)]
pub struct Mpeg {
    atom_list: Option<u8>,
}

impl<'a> Mpeg {
    pub fn new(d: &'a [u8]) -> Self {
        let atom_list = build_tree(d);
        Mpeg {
            atom_list,
            ..Default::default()
        }
    }
}
/*** Mpeg type definition ***/
