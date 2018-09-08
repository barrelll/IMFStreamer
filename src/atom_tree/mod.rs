mod atom_error;
mod atoms;

use std::fmt;
use std::str;

//use self::atoms::*;
pub use self::atom_error::AtomError;

pub fn build_tree(data: &[u8]) -> Tree<&[u8]> {
    let err_str = "Unreadable!";
    let eof = data.len();
    let mut tree = Tree::<&[u8]>::new();
    let mut idx = 0;
    let find_idx = |split: usize| -> usize {
        use byteorder::{BigEndian, ReadBytesExt};
        use std::io::Cursor;

        let d = &data[split..];
        let size_flag = Cursor::new(&d[..4]).read_u32::<BigEndian>().expect(err_str);
        let actual_size = match size_flag {
            0 => eof,
            1 => Cursor::new(&d[8..16])
                .read_u64::<BigEndian>()
                .expect(err_str) as usize,
            val => val as usize,
        };
        actual_size + split
    };

    loop {
        if idx >= eof {
            break;
        }
        let split = idx;
        // split
        idx = find_idx(split);
        let name = str::from_utf8(&data[split + 4..split + 8]).ok();
        if contains_children(name.expect(err_str)) {
            // let fn = get_child_function(name.expect(err_str));
            // fn (&data[split..idx])?
            tree.new_node(name, None, None, None, None, None, &data[split..idx]);
        } else {
            tree.new_node(name, None, None, None, None, None, &data[split..idx]);
        }
    }
    tree
}

fn contains_children(name: &str) -> bool {
    atoms::ATOM_TYPES_WCHILDREN.binary_search(&name).is_ok()
        || atoms::FULL_ATOM_TYPES_WCHILDREN
            .binary_search(&name)
            .is_ok()
}

type Root<'a, T> = Vec<Node<'a, T>>;

#[derive(Debug, Default)]
pub struct Tree<'a, T: 'a> {
    root: Root<'a, T>,
}

impl<'a, T> Tree<'a, T> {
    fn new_node(
        &mut self,
        name: Option<&'a str>,
        parent: Option<&'a Node<'a, T>>,
        parent_name: Option<&'a str>,
        parent_list: Option<&'a Vec<Node<'a, T>>>,
        children: Option<Vec<Node<'a, T>>>,
        next: Option<&'a Node<'a, T>>,
        data: T,
    ) -> usize {
        let next_index = self.root.len();

        self.root.push(Node {
            name,
            parent,
            parent_name,
            parent_list,
            children,
            next,
            data,
        });

        next_index
    }

    fn new() -> Tree<'a, T> {
        let root = Root::<'a, T>::new();
        Tree { root }
    }
}

#[derive(Default)]
struct Node<'a, T: 'a> {
    name: Option<&'a str>,
    parent: Option<&'a Node<'a, T>>,
    parent_name: Option<&'a str>,
    parent_list: Option<&'a Root<'a, T>>,
    children: Option<Root<'a, T>>,
    next: Option<&'a Node<'a, T>>,

    /// The actual data which will be stored within the tree
    pub data: T,
}

impl<'a, T> fmt::Debug for Node<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Node: [ name {:?}, parent_name {:?}, children {:?} ]",
            self.name, self.parent_name, self.children
        )
    }
}
