use super::{descrfactory, size_of_instance, DescrBase, DescrBaseTags, DescrBuilder};
#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct ObjectDescriptor {
    tag: Option<DescrBaseTags>,
    size_of_instance: Option<u8>,
    od_id: Option<[bool; 10]>,
    reserved: Option<[bool; 5]>,
    url_flag: Option<bool>,
    url_length: Option<u8>,
    url_string: Option<String>,
    descriptors: Vec<Box<dyn DescrBase>>,
}

impl DescrBase for ObjectDescriptor {
    fn tag(&self) -> Option<DescrBaseTags> {
        self.tag.clone()
    }

    fn rdclone(&self) -> Box<DescrBase> {
        Box::new(self.clone())
    }
}

impl DescrBuilder for ObjectDescriptor {
    fn build(data: &[u8]) -> Option<Self> {
        use byteorder::{BigEndian, ReadBytesExt};
        use std::io::Cursor;
        let tag = Some(match Cursor::new(&data[..1])
            .read_u8()
            .expect("ObjectDescriptor error reading tag")
        {
            0x02 => DescrBaseTags::ObjectDescrTag,
            0x10 => DescrBaseTags::MP4ODTag,
            _ => {
                panic!("Object descriptor tag doesn't match the object descriptor base tags");
            }
        });

        let mut cursor: usize = 1;
        let size_of_instance = Some(size_of_instance(data, &mut cursor));
        let id = Cursor::new(&data[cursor..cursor + 2])
            .read_u16::<BigEndian>()
            .expect("ObjectDescriptor error reading id bytes");

        let od_id = Some({
            let mut arr = [false; 10];
            let mut arr_idx = 0;
            for i in (6..16).rev() {
                let val = id & (1 << i) > 0;
                arr[arr_idx] = val;
                arr_idx += 1;
            }
            arr
        });

        let url_flag = Some({ id & (1 << 5) > 0 });

        match url_flag {
            Some(v) => {
                let reserved = Some([true; 5]);
                let mut url_length = None;
                let mut url_string = None;
                if v {
                    url_length = match Cursor::new(&data[cursor + 2..cursor + 3]).read_u8() {
                        Ok(val) => {
                            url_string = match String::from_utf8(
                                data[cursor + 3..cursor + (val as usize)].to_vec(),
                            ) {
                                Ok(val) => Some(val),
                                Err(e) => {
                                    panic!(
                                        "ObjectDescriptor error, cannot parse url_string. {:?}",
                                        e
                                    );
                                }
                            };
                            Some(val)
                        }
                        Err(e) => {
                            panic!("ObjectDescriptor error, cannot parse url_length. {:?}", e);
                        }
                    };
                }

                let mut descriptors = descrfactory(&data[cursor + 7..]);
                Some(ObjectDescriptor {
                    tag,
                    size_of_instance,
                    od_id,
                    url_flag,
                    reserved,
                    url_length,
                    url_string,
                    descriptors,
                })
            }
            None => {
                panic!("Can't find ObjectDescriptor url flag!");
            }
        }
    }
}
