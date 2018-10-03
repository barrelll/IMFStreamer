#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
mod atoms;

use std::{cell::RefCell, fmt, rc::Rc, rc::Weak, str};

mod private {
    pub trait IsSlice {
        type Item;
        fn as_slice(&self) -> &[Self::Item];
    }

    impl<'a, T> IsSlice for &'a [T] {
        type Item = T;
        fn as_slice(&self) -> &[Self::Item] {
            self
        }
    }

    impl<'a, T> IsSlice for Vec<T> {
        type Item = T;
        fn as_slice(&self) -> &[Self::Item] {
            &self
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Tree<'a, T: 'a>
where
    T: Copy + Clone + private::IsSlice + Default,
{
    root: Vec<Rc<Node<'a, T>>>,
}

impl<'a, T: 'a> Tree<'a, T>
where
    T: Copy + Clone + private::IsSlice + Default,
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
}

#[derive(Default, Clone)]
struct Node<'a, T>
where
    T: Copy + Clone + private::IsSlice + Default,
{
    data: Option<T>,
    name: Option<&'a str>,
    parent: RefCell<Weak<Node<'a, T>>>,
    children: Vec<Rc<Node<'a, T>>>,
}

impl<'a, T: 'a> fmt::Debug for Node<'a, T>
where
    T: Copy + Clone + private::IsSlice + Default,
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
    T: Copy + Clone + private::IsSlice + Default,
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

    fn children(
        data: Option<&'a [u8]>,
        name: Option<&'a str>,
        parent: RefCell<Weak<Node<'a, &'a [u8]>>>,
    ) -> Rc<Node<'a, &'a [u8]>> {
        let mut n = Node::<&[u8]>::new(data, name, parent);
        if contains_children(n.name.unwrap()) {
            let children = build(&data.unwrap()[8..]);
            n.children = children;
            let n= Rc::new(n);
            for node in n.children.iter() {
                *node.parent.borrow_mut() = Rc::downgrade(&n);
            }
            n
        } else {
            let n= Rc::new(n);
            n
        }
    }
}

fn contains_children(name: &str) -> bool {
    atoms::ATOM_TYPES_WCHILDREN.binary_search(&name).is_ok() || atoms::FULL_ATOM_TYPES_WCHILDREN
        .binary_search(&name)
        .is_ok()
}

fn build<'a>(data: &'a [u8]) -> Vec<Rc<Node<'a, &[u8]>>> {
    let err_str = "Unreadable!";
    let eof = data.len();
    let mut root = Vec::<Rc<Node<&[u8]>>>::new();
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
        let parent = RefCell::new(Weak::new());
        let node = Node::<&[u8]>::children(Some(&data[split..idx]), name, parent);
        root.push(node);
    }
    println!("{:?}", root);
    root
}

pub fn build_tree(data: &[u8]) -> Option<u8> {
    let _ = Tree::from_root(build(data));
    None
}
