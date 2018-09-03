use atom_tree::*;

/*** Mpeg type definition ***/
#[derive(Clone, Debug, Default)]
pub struct Mpeg<'a> {
    atom_list: Option<Vec<&'a str>>,
}

impl<'a> Mpeg<'a> {
    pub fn new(_d: &'a [u8]) -> Self {
        let atom_list = None;
        Mpeg {
            atom_list,
            ..Default::default()
        }
    }


    fn _build(d: &'a [u8]) -> Option<Vec<&'a str>> {
        let mut data = d;
        loop {
            if data.len() == 0 {
                break;
            }
            let size = match data.size() {
                Ok(val) => val,
                Err(e) => match e {
                    AtomError::EOFError => data.len(),
                    _ => {
                        panic!("Mpeg, build_layers: cannot read size of atom!");
                    }
                },
            };
            data = &data[size..];
        }
        None
    }
}
/*** Mpeg type definition ***/
