use super::{size_of_instance, DescrBase, DescrBaseTags, DescrBuilder, descrfactory};
use IsSlice;

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct ESDescriptor {
    tag: Option<DescrBaseTags>,
    size_of_instance: Option<u8>,
    es_id: Option<u16>,
    stream_dependence_flag: Option<bool>,
    url_flag: Option<bool>,
    ocr_stream_flag: Option<bool>,
    strean_priority: Option<[bool; 5]>,
    depends_on_es_id: Option<u16>,
    url_length: Option<u8>,
    url_string: Option<String>,
    ocr_es_id: Option<u16>,
    descriptors: Vec<Box<dyn DescrBase>>,
}

impl DescrBase for ESDescriptor {
    fn tag(&self) -> Option<DescrBaseTags> {
        self.tag.clone()
    }

    fn rdclone(&self) -> Box<DescrBase> {
        Box::new(self.clone())
    }
}

impl DescrBuilder for ESDescriptor {
    fn build<T: IsSlice<Item = u8>>(d: T) -> Option<Self> {
        let data = d.as_slice();
        use byteorder::{BigEndian, ReadBytesExt};
        use std::io::Cursor;
        let tag = Some(match Cursor::new(&data[..1])
            .read_u8()
            .expect("ESDescriptor error reading tag")
        {
            0x03 => DescrBaseTags::ESDescrTag,
            _ => {
                panic!("ESDescriptor descriptor tag doesn't match the object descriptor base tags");
            }
        });
        let mut cursor = 1;
        let size_of_instance = Some(size_of_instance(data, &mut cursor));
        let es_id = Cursor::new(&data[cursor..cursor + 2])
            .read_u16::<BigEndian>()
            .ok();
        let byte = Cursor::new(&data[cursor + 2..cursor + 3])
            .read_u8()
            .expect("ESDescriptor error reading byte");
        let stream_dependence_flag = Some(byte & (1 << 7) > 0);
        let url_flag = Some(byte & (1 << 6) > 0);
        let ocr_stream_flag = Some(byte & (1 << 5) > 0);
        let strean_priority: Option<[bool; 5]> = Some({
            let mut arr_idx = 0;
            let mut ret = [false; 5];
            for idx in (0..5).rev() {
                ret[arr_idx] = byte & (1 << idx) > 0;
                arr_idx += 1;
            }
            ret
        });
        cursor += 3;
        let mut depends_on_es_id = None;
        if let Some(true) = stream_dependence_flag {
            depends_on_es_id = Cursor::new(&data[cursor..cursor+2]).read_u16::<BigEndian>().ok();
            cursor += 2;
        }
        let mut url_length = None;
        let mut url_string = None;
        if let Some(true) = url_flag {
            url_length = Cursor::new(&data[cursor..cursor+1]).read_u8().ok();
            url_string = String::from_utf8(data[cursor+1..cursor+1+url_length.unwrap() as usize].to_vec()).ok();
            cursor += 1 + url_length.unwrap() as usize;
        }
        let mut ocr_es_id = None;
        if let Some(true) = ocr_stream_flag {
            ocr_es_id = Cursor::new(&data[cursor..cursor+2]).read_u16::<BigEndian>().ok();
            cursor += 2;
        }
        let descriptors = descrfactory(&data[cursor..]);
        Some(ESDescriptor {
            tag,
            size_of_instance,
            es_id,
            stream_dependence_flag,
            url_flag,
            ocr_stream_flag,
            strean_priority,
            depends_on_es_id,
            url_length,
            url_string,
            ocr_es_id,
            descriptors,
        })
    }
}
