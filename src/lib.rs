#![allow(dead_code)]
#[macro_use]
extern crate downcast_rs;
extern crate byteorder;
pub mod base_descriptors;
pub mod iso_p12;
pub mod iso_p14;
mod tests;

use byteorder::{BigEndian, ReadBytesExt};
use std::{
    cell::RefCell,
    fs::File,
    io::{Cursor, Error, ErrorKind, Read, Result, Seek, SeekFrom},
    rc::Rc,
    rc::Weak,
    str,
};

pub trait Name<'a> {
    fn name() -> &'a str;
}

pub trait BuildNode {
    fn build<T: IsSlice<Item = u8>>(data: T) -> Option<Self>
    where
        Self: Sized;
}

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

pub trait MediaStreamTree {
    fn searchtree_stype<T: BuildNode>(&mut self, search: &str) -> Result<T>;
    fn searchtree(&mut self, search: &str) -> Result<Node>;
}

impl MediaStreamTree for File {
    fn searchtree_stype<T: BuildNode>(&mut self, search: &str) -> Result<T> {
        let node = self.searchtree(search)?;
        solid_ntype::<T>(self, &node)
    }

    fn searchtree(&mut self, search: &str) -> Result<Node> {
        let paths: Vec<&str> = search.split('.').collect();
        let mut slice = Slice(0, self.metadata()?.len(), 0);
        let mut node = Node {
            ..Default::default()
        };
        for path in paths {
            let idx: String = path.rmatches(char::is_numeric).collect();
            let idx = match idx.parse::<usize>() {
                Ok(val) => val,
                _ => 0,
            };
            node = search_slice(slice, self, &path[..4].to_ascii_lowercase(), idx)?;
            slice = node.slice;
        }
        Ok(node)
    }
}

fn solid_ntype<T: BuildNode>(fstream: &mut File, n: &Node) -> Result<T> {
    let buffer_size = n.slice.1 - n.slice.0;
    let mut buf = vec![0; buffer_size as usize];
    fstream.seek(SeekFrom::Start(n.slice.0))?;
    fstream.read_exact(&mut buf)?;
    T::build::<&[u8]>(&buf[..]).ok_or(Error::new(
        ErrorKind::InvalidData,
        format!("Data can't be read properly?"),
    ))
}

fn search_slice(s: Slice, handle: &mut File, atomname: &str, idx: usize) -> Result<Node> {
    let len = handle.metadata()?.len();
    let mut buf: [u8; 8] = [0; 8];
    let mut offset = 8;
    let mut cur_pos = handle.seek(SeekFrom::Start(s.0 + s.2))?;
    let mut enclosed_idx = 0;

    while handle.read_exact(&mut buf).is_ok() {
        let name = match str::from_utf8(&buf[4..8]) {
            Ok(val) => val,
            Err(e) => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Unable to read name, {}", e),
                ))
            }
        };

        let prevnodeendat = cur_pos;

        let size = match Cursor::new(&buf[..4]).read_u32::<BigEndian>() {
            Ok(val) => match val {
                0 => len,
                1 => {
                    offset = 16;
                    let mut buf: [u8; 8] = [0; 8];
                    let _ = handle.read_exact(&mut buf)?;
                    Cursor::new(&buf[..]).read_u64::<BigEndian>()?
                }
                _ => val as u64,
            },
            Err(e) => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Unable to read size, {}", e),
                ))
            }
        };

        cur_pos = match handle.seek(SeekFrom::Current((size - offset) as i64)) {
            Ok(val) => {
                if val > len {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!("Atom {} not found, name searched for = {} Cursor pos is greater than file size {} > {}", atomname, name, val, len),
                    ));
                } else {
                    val
                }
            }
            Err(e) => {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    format!("Atom {} not found, Error {:?}", atomname, e),
                ))
            }
        };

        if atomname.eq(name) {
            if enclosed_idx == idx {
                return Ok(Node::new(
                    Slice(prevnodeendat, cur_pos, offset),
                    Some(name.to_string()),
                    RefCell::new(Weak::new()),
                ));
            }
            enclosed_idx += 1;
        }
    }
    Err(Error::new(
        ErrorKind::NotFound,
        format!(
            "Atom {} not found, Cursor position {:?}, length of file {:?}",
            atomname, cur_pos, len
        ),
    ))
}

#[derive(Debug, Default, Clone, Copy)]
/// contains the start and end of each node relevant to the file size, with the last u64 being the amount of bytes to increment for size/name
pub struct Slice(u64, u64, u64);

#[derive(Debug, Default, Clone)]
pub struct Node {
    slice: Slice,
    name: Option<String>,
    parent: RefCell<Weak<Node>>,
    children: Vec<Rc<Node>>,
}

impl Node {
    fn new(slice: Slice, name: Option<String>, parent: RefCell<Weak<Node>>) -> Node {
        Node {
            slice,
            name,
            parent,
            ..Default::default()
        }
    }

    fn default() -> Node {
        Node {
            ..Default::default()
        }
    }

    pub fn num_children(&self) -> usize {
        self.children.len()
    }
}
