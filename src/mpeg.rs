use atoms::*;

/*** Mpeg type definition ***/
#[derive(Debug, Default)]
pub struct Mpeg<'a> {
    cached: Option<u32>,
    atom_list: Option<Tree<'a, &'a [u8]>>,
}

impl<'a> Mpeg<'a> {
    pub fn major_brand(&self) -> Option<String> {
        let tree = &self.atom_list;
        match tree {
            Some(t) => match t.solid_type_search_path::<iso_p12::Ftyp>("ftyp") {
                Some(val) => val.major_brand,
                None => None,
            },
            None => None,
        }
    }

    pub fn minor_version(&self) -> Option<u32> {
        let tree = &self.atom_list;
        match tree {
            Some(t) => match t.solid_type_search_path::<iso_p12::Ftyp>("ftyp") {
                Some(val) => val.minor_version,
                None => None,
            },
            None => None,
        }
    }

    pub fn minor_brands(&self) -> Option<Vec<String>> {
        let tree = &self.atom_list;
        match tree {
            Some(t) => match t.solid_type_search_path::<iso_p12::Ftyp>("ftyp") {
                Some(val) => val.minor_brands,
                None => None,
            },
            None => None,
        }
    }

    pub fn find_iods(&self) -> Option<iso_p14::Iods> {
        let tree = &self.atom_list;
        match tree {
            Some(t) => t.solid_type_search_path::<iso_p14::Iods>("ftyp"),
            None => None,
        }
    }
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
