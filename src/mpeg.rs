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
            Some(t) => match t.solid_type_search_path::<iso_p12::Ftyp>("ftyp", None) {
                Some(val) => val.major_brand,
                None => None,
            },
            None => None,
        }
    }

    pub fn minor_version(&self) -> Option<u32> {
        let tree = &self.atom_list;
        match tree {
            Some(t) => match t.solid_type_search_path::<iso_p12::Ftyp>("ftyp", None) {
                Some(val) => val.minor_version,
                None => None,
            },
            None => None,
        }
    }

    pub fn minor_brands(&self) -> Option<Vec<String>> {
        let tree = &self.atom_list;
        match tree {
            Some(t) => match t.solid_type_search_path::<iso_p12::Ftyp>("ftyp", None) {
                Some(val) => val.minor_brands,
                None => None,
            },
            None => None,
        }
    }

    pub fn t_grab_traks(&self) -> Option<Vec<Option<iso_p12::Trak>>> {
        let tree = &self.atom_list;
        match tree {
            Some(t) => {
                let parent = t.node_search_path("moov", None).expect("can't find moov");
                let traks = parent.solid_type_children_of_type::<iso_p12::Trak>();
                let trak_nodes = parent.node_children_of_type::<iso_p12::Trak>();
                for node in trak_nodes {
                    let _ =
                        node.solid_type_search_path::<iso_p12::Stsd>("mdia.minf.stbl.stsd", None);
                }
                Some(traks)
            }
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
