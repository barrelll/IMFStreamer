#![allow(dead_code)]
pub mod iso_p12;
pub mod iso_p14;

pub trait Name<'a> {
    fn name() -> &'a str;
}

pub trait BuildNode {
    fn build<T: IsSlice<Item = u8>>(data: T) -> Option<Self>
    where
        Self: Sized;
}

use std::{cell::RefCell, fmt, rc::Rc, rc::Weak, str};

pub trait IsSlice {
    type Item;
    fn as_slice(&self) -> &[Self::Item];
}

impl<'a> IsSlice for &'a [u8] {
    type Item = u8;
    fn as_slice(&self) -> &[Self::Item] {
        self
    }
}

impl<'a> IsSlice for Vec<u8> {
    type Item = u8;
    fn as_slice(&self) -> &[Self::Item] {
        &self
    }
}

#[derive(Debug, Default, Clone)]
pub struct Tree<'a, T: 'a>
where
    T: Copy + Clone + IsSlice<Item = u8> + Default,
{
    root: Vec<Rc<Node<'a, T>>>,
}

impl<'a, T: 'a> Tree<'a, T>
where
    T: Copy + Clone + IsSlice<Item = u8> + Default,
{
    fn new() -> Tree<'a, T> {
        Tree {
            root: Vec::<Rc<Node<'a, T>>>::new(),
        }
    }

    fn from_root(root: Vec<Rc<Node<'a, T>>>) -> Tree<'a, T> {
        Tree { root }
    }

    fn push(&mut self, n: Rc<Node<'a, T>>) {
        self.root.push(n);
    }

    pub fn solid_type_search_path<'p, N: BuildNode + Name<'p>>(&self, path: &str) -> Option<N> {
        let paths: Vec<&str> = path.split('.').collect();
        let idx: String = paths[0].rmatches(char::is_numeric).collect();
        let idx = match idx.parse::<i32>() {
            Ok(val) => val,
            _ => 0,
        };
        let path_to_match: String = paths[0].matches(char::is_alphabetic).collect();
        let iter = self.root.iter();
        let mut i = 0;
        for node in iter {
            match node.name {
                Some(val) => {
                    if val == path_to_match {
                        let path = {
                            let mut ret = String::new();
                            let len = paths.len();
                            if len == 1 {
                                if idx == i {
                                    return N::build(node.data.unwrap());
                                } else {
                                    i += 1;
                                    continue;
                                }
                            }
                            let slice = &paths[1..len - 1];
                            for &p in slice {
                                ret += &(p.to_string() + ".");
                            }
                            ret + paths[len - 1]
                        };
                        if idx == i {
                            return node.solid_type_search_path(path.as_str());
                        } else {
                            i += 1;
                            continue;
                        }
                    }
                }
                None => return None,
            }
        }
        None
    }

    pub fn node_search_path(&self, path: &str) -> Option<Rc<Node<'a, T>>> {
        let paths: Vec<&str> = path.split('.').collect();
        let idx: String = paths[0].rmatches(char::is_numeric).collect();
        let idx = match idx.parse::<i32>() {
            Ok(val) => val,
            _ => 0,
        };
        let path_to_match: String = paths[0].matches(char::is_alphabetic).collect();
        let iter = self.root.iter();
        let mut i = 0;
        for node in iter {
            match node.name {
                Some(val) => {
                    if val == path_to_match {
                        let path = {
                            let mut ret = String::new();
                            let len = paths.len();
                            if len == 1 {
                                if idx == i {
                                    return Some(Rc::clone(node));
                                } else {
                                    i += 1;
                                    continue;
                                }
                            }
                            let slice = &paths[1..len - 1];
                            for &p in slice {
                                ret += &(p.to_string() + ".");
                            }
                            ret + paths[len - 1]
                        };
                        if idx == i {
                            return node.node_search_path(path.as_str());
                        } else {
                            i += 1;
                            continue;
                        }
                    }
                }
                None => return None,
            }
        }
        None
    }
}

#[derive(Default, Clone)]
pub struct Node<'a, T>
where
    T: Copy + Clone + IsSlice<Item = u8> + Default,
{
    data: Option<T>,
    name: Option<&'a str>,
    parent: RefCell<Weak<Node<'a, T>>>,
    children: Vec<Rc<Node<'a, T>>>,
}

impl<'a, T: 'a> fmt::Debug for Node<'a, T>
where
    T: Copy + Clone + IsSlice<Item = u8> + Default,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Node {{ name: {:?}, parent: {:?}, children {:?} }}",
            self.name, self.parent, self.children
        )
    }
}

impl<'a, T: 'a> Node<'a, T>
where
    T: Copy + Clone + IsSlice<Item = u8> + Default,
{
    fn new(
        data: Option<T>,
        name: Option<&'a str>,
        parent: RefCell<Weak<Node<'a, T>>>,
    ) -> Node<'a, T> {
        Node {
            data,
            name,
            parent,
            ..Default::default()
        }
    }

    fn default() -> Node<'a, T> {
        Node {
            ..Default::default()
        }
    }

    fn push_children(
        data: Option<&'a [u8]>,
        name: Option<&'a str>,
        parent: RefCell<Weak<Node<'a, &'a [u8]>>>,
        start: usize,
    ) -> Rc<Node<'a, &'a [u8]>> {
        let mut n = Node::<&[u8]>::new(data, name, parent);
        if contains_children(n.name.unwrap()) {
            if unique(name.unwrap()) {
                return Rc::new(n);
            }
            let children = build(&data.unwrap()[start..]);
            n.children = children;
            let n = Rc::new(n);
            for node in n.children.iter() {
                *node.parent.borrow_mut() = Rc::downgrade(&n);
            }
            n
        } else {
            let n = Rc::new(n);
            n
        }
    }

    pub fn solid_type_children_of_type<'p, N: BuildNode + Name<'p>>(&self) -> Vec<Option<N>> {
        let v: Vec<Option<N>> = self
            .children
            .iter()
            .filter(|x| {
                N::name().eq(x
                    .name
                    .expect("Node: node_children_of_type node.name doesn't exist"))
            }).map(|x| {
                N::build(
                    x.data
                        .expect("Node: node_children_of_type node.data doesn't exist"),
                )
            }).collect();
        v
    }

    pub fn node_children_of_type<'p, N: Name<'p>>(&self) -> Vec<&Rc<Node<'a, T>>> {
        let v: Vec<&Rc<Node<T>>> = self
            .children
            .iter()
            .filter(|x| {
                N::name().eq(x
                    .name
                    .expect("Node: node_children_of_type node.name doesn't exist"))
            }).collect();
        v
    }

    pub fn num_children(&self) -> usize {
        self.children.len()
    }

    pub fn solid_type_search_path<'p, N: BuildNode + Name<'p>>(&self, path: &str) -> Option<N> {
        let paths: Vec<&str> = path.split('.').collect();
        let idx: String = paths[0].rmatches(char::is_numeric).collect();
        let idx = match idx.parse::<i32>() {
            Ok(val) => val,
            _ => 0,
        };
        let path_to_match: String = paths[0].matches(char::is_alphabetic).collect();
        let iter = self.children.iter();
        let mut i = 0;
        for node in iter {
            match node.name {
                Some(val) => {
                    if val == path_to_match {
                        let path = {
                            let mut ret = String::new();
                            let len = paths.len();
                            if len == 1 {
                                if idx == i {
                                    return N::build(node.data.unwrap());
                                } else {
                                    i += 1;
                                    continue;
                                }
                            }
                            let slice = &paths[1..len - 1];
                            for &p in slice {
                                ret += &(p.to_string() + ".");
                            }
                            ret + paths[len - 1]
                        };
                        if idx == i {
                            return node.solid_type_search_path(path.as_str());
                        } else {
                            i += 1;
                            continue;
                        }
                    }
                }
                None => return None,
            }
        }
        None
    }

    pub fn node_search_path(&self, path: &str) -> Option<Rc<Node<'a, T>>> {
        let paths: Vec<&str> = path.split('.').collect();
        let idx: String = paths[0].rmatches(char::is_numeric).collect();
        let idx = match idx.parse::<i32>() {
            Ok(val) => val,
            _ => 0,
        };
        let path_to_match: String = paths[0].matches(char::is_alphabetic).collect();
        let iter = self.children.iter();
        let mut i = 0;
        for node in iter {
            match node.name {
                Some(val) => {
                    if val == path_to_match {
                        let path = {
                            let mut ret = String::new();
                            let len = paths.len();
                            if len == 1 {
                                if idx == i {
                                    return Some(Rc::clone(node));
                                } else {
                                    i += 1;
                                    continue;
                                }
                            }
                            let slice = &paths[1..len - 1];
                            for &p in slice {
                                ret += &(p.to_string() + ".");
                            }
                            ret + paths[len - 1]
                        };
                        if idx == i {
                            return node.node_search_path(path.as_str());
                        } else {
                            i += 1;
                            continue;
                        }
                    }
                }
                None => return None,
            }
        }
        None
    }
}

fn unique(name: &str) -> bool {
    name == "dref" || name == "stsd" || name == "udta"
}

fn contains_children(name: &str) -> bool {
    iso_p12::ATOM_TYPES_WCHILDREN.binary_search(&name).is_ok() || iso_p12::FULL_ATOM_TYPES_WCHILDREN
        .binary_search(&name)
        .is_ok()
}

fn build<'a>(data: &'a [u8]) -> Vec<Rc<Node<'a, &[u8]>>> {
    let err_str = "Unreadable!";
    let eof = data.len();
    let mut root = Vec::<Rc<Node<&[u8]>>>::new();
    let mut idx = 0;

    let find_idx = |split: usize| -> (usize, usize) {
        use byteorder::{BigEndian, ReadBytesExt};
        use std::io::Cursor;

        let d = &data[split..];
        let size_flag = Cursor::new(&d[..4]).read_u32::<BigEndian>().expect(err_str);
        let mut start_pos: usize = 8;
        let actual_size = match size_flag {
            0 => eof,
            1 => {
                start_pos = 16;
                Cursor::new(&d[8..16])
                    .read_u64::<BigEndian>()
                    .expect(err_str) as usize
            }
            val => val as usize,
        };
        (actual_size + split, start_pos)
    };

    loop {
        if idx >= eof {
            break;
        }
        let split = idx;
        // split
        let (x, y) = find_idx(split);
        idx = x;
        let name = str::from_utf8(&data[split + 4..split + 8]).ok();
        let parent = RefCell::new(Weak::new());
        if idx <= data.len() {
            let node = Node::<&[u8]>::push_children(Some(&data[split..idx]), name, parent, y);
            root.push(node);
        }
    }
    root
}

pub fn build_tree(data: &[u8]) -> Option<Tree<&[u8]>> {
    let tree = Tree::from_root(build(data));
    Some(tree)
}
