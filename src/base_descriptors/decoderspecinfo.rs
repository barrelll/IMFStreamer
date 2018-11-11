use super::{size_of_instance, DescrBase, DescrBaseTags, DescrBuilder};

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct DecoderSpecificInfo {
    tag: Option<DescrBaseTags>,
    size_of_instance: Option<u8>,
    datav: Vec<u8>,
}

impl DescrBase for DecoderSpecificInfo {
    fn tag(&self) -> Option<DescrBaseTags> {
        self.tag.clone()
    }

    fn rdclone(&self) -> Box<DescrBase> {
        Box::new(self.clone())
    }
}

impl DecoderSpecificInfo {
    pub fn build_specdecinfo(_object_identifier: u8) -> DecoderSpecificInfo {
        DecoderSpecificInfo {
            ..Default::default()
        }
    }
}

impl DescrBuilder for DecoderSpecificInfo {
    fn build(data: &[u8]) -> Option<Self> {
        use byteorder::BigEndian;
        use byteorder::ReadBytesExt;
        use std::io::Cursor;
        let tag = Some(match Cursor::new(&data[..1])
            .read_u8()
            .expect("ESDescriptor error reading tag")
        {
            0x05 => DescrBaseTags::DecSpecificInfoTag,
            _ => {
                panic!("DecoderSpecificInfo descriptor tag doesn't match the object descriptor base tags");
            }
        });
        let mut cursor = 1;
        let size_of_instance = Some(size_of_instance(data, &mut cursor));
        for n in &data[..size_of_instance.unwrap() as usize] {
            let mut s = format!("{:b}", n);
            for _u in 0..8 - s.len() {
                s.insert(0, '0');
            }
            print!("{} ", s);
        }
        let some_value = Cursor::new(&data[cursor..cursor + 4]).read_u32::<BigEndian>();
        println!("\ns {:?}", some_value);
        let some_value = Cursor::new(&data[cursor + 4..cursor + 5]).read_u8();
        println!("s {:?}", some_value);
        let datav = data[cursor..cursor + size_of_instance.unwrap() as usize].to_vec();
        Some(DecoderSpecificInfo {
            tag,
            size_of_instance,
            datav,
        })
    }
}
