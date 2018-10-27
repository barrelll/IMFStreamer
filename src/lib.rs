#![allow(dead_code)]
extern crate byteorder;
pub mod iso_p12;
pub mod iso_p14;
mod tests;

use std::fs::File;

pub trait Name<'a> {
    fn name() -> &'a str;
}

pub trait BuildNode {
    fn build<T: IsSlice<Item = u8>>(data: T) -> Option<Self>
    where
        Self: Sized;
}

use std::{cell::RefCell, rc::Rc, rc::Weak, str};

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
pub struct AdamTree<'a> {
    fhandle: Option<&'a File>,
    root: Option<Vec<Rc<Node<'a>>>>,
}

impl<'a> AdamTree<'a> {
    fn new(fhandle: &'a File) -> AdamTree<'a> {
        let fhandle = Some(fhandle);
        AdamTree {
            fhandle,
            ..Default::default()
        }
    }

    fn search_tree(&mut self, search_name: &'a str) -> Result<Node<'a>, String> {
        use byteorder::{BigEndian, ReadBytesExt};
        use std::io::{Cursor, Read, Seek, SeekFrom};

        let paths: Vec<&str> = search_name.split('.').collect();
        let idx: String = paths[0].rmatches(char::is_numeric).collect();
        let idx = match idx.parse::<i32>() {
            Ok(val) => val,
            _ => 0,
        };

        println!("{:?}, {:?}", paths, idx);

        let mut buf: [u8; 4] = [0; 4];
        let root = self.root.as_ref();
        match root {
            Some(val) => {}
            None => {
                let mut handle = self
                    .fhandle
                    .expect("File handle doesn't exist for AdamTree");
                while handle.read_exact(&mut buf).is_ok() {
                    let cursor_s = handle.seek(SeekFrom::Current(0)).unwrap() - 4;
                    let size = match Cursor::new(&buf).read_u32::<BigEndian>() {
                        Ok(val) => match val {
                            0 => {
                                let len = handle.metadata().unwrap().len();
                                len
                            }
                            1 => {
                                let mut buf: [u8; 8] = [0; 8];
                                handle.seek(SeekFrom::Current(4));
                                handle.read_exact(&mut buf);
                                let ret = Cursor::new(&buf).read_u64::<BigEndian>().unwrap();
                                handle.seek(SeekFrom::Current(-12));
                                ret
                            }
                            _ => val as u64,
                        },
                        _ => return Err("".to_string()),
                    };
                    let cursor_e = cursor_s + size;
                    if handle.read_exact(&mut buf).is_ok() {
                        let name = str::from_utf8(&buf).ok();
                        let slice = Slice(cursor_s, cursor_e);
                        let parent = RefCell::new(Weak::new());
                        let node = Node::new(slice, name, parent);
                        println!(
                            "cursor_s {:?}, size {:?}, cursor_e {:?}, name {:?}",
                            cursor_s, size, cursor_e, name
                        );
                    } else {
                        break;
                    }
                    handle.seek(SeekFrom::Current(size as i64 - 8));
                }
            }
        }
        Err("Not impl'd yet".to_string())
    }
}

#[derive(Debug, Default, Clone)]
struct Slice(u64, u64);

#[derive(Debug, Default, Clone)]
pub struct Node<'a> {
    slice: Slice,
    name: Option<&'a str>,
    parent: RefCell<Weak<Node<'a>>>,
    children: Vec<Rc<Node<'a>>>,
}

impl<'a> Node<'a> {
    fn new(slice: Slice, name: Option<&'a str>, parent: RefCell<Weak<Node<'a>>>) -> Node<'a> {
        Node {
            slice,
            name,
            parent,
            ..Default::default()
        }
    }

    fn default() -> Node<'a> {
        Node {
            ..Default::default()
        }
    }

    pub fn num_children(&self) -> usize {
        self.children.len()
    }
}
