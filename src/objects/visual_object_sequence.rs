use super::ObjectBuilder;

pub struct VisualObjectSequence {
    visual_object_sequence_start_code: u32,
    profile_and_level_indication: u8,
    // studio visual objects
    // visual objects
}

impl ObjectBuilder for VisualObjectSequence {
    fn build(_data: &[u8]) -> Option<Self> {
//        use byteorder::{BigEndian, ReadBytesExt};
//        use std::io::Cursor;
//        let mut s = String::new();
//        for item in data {
//            let bin = format!("{:#010b} ", item);
//            let slen = s.len();
//            s = format!("{}{}", s, bin);
//        }
//        // left to right, Big endian
//        let mut bit_position = 0;
        None
    }
}