use super::{descrfactory, size_of_instance, DescrBase, DescrBaseTags, DescrBuilder};
#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct InitialObjectDescriptor {
    tag: Option<DescrBaseTags>,
    size_of_instance: Option<u8>,
    od_id: Option<[bool; 10]>,
    reserved: Option<[bool; 4]>,
    url_flag: Option<bool>,
    include_inline_profile_level_flag: Option<bool>,
    url_length: Option<u8>,
    url_string: Option<String>,
    od_profile_level_indication: Option<u8>,
    scene_profile_level_indication: Option<u8>,
    audio_profile_level_indication: Option<u8>,
    visual_profile_level_indication: Option<u8>,
    graphics_profile_level_indication: Option<u8>,
    descriptors: Vec<Box<dyn DescrBase>>,
}

impl DescrBase for InitialObjectDescriptor {
    fn tag(&self) -> Option<DescrBaseTags> {
        self.tag.clone()
    }

    fn rdclone(&self) -> Box<DescrBase> {
        Box::new(self.clone())
    }
}

impl DescrBuilder for InitialObjectDescriptor {
    fn build(data: &[u8]) -> Option<Self> {
        use byteorder::{BigEndian, ReadBytesExt};
        use std::io::Cursor;
        let tag = Some(match Cursor::new(&data[..1])
            .read_u8()
            .expect("InitialObjectDescriptor error reading tag")
        {
            0x02 => DescrBaseTags::InitialObjectDescrTag,
            0x10 => DescrBaseTags::MP4IODTag,
            _ => {
                panic!("Object descriptor tag doesn't match the object descriptor base tags");
            }
        });

        let mut cursor: usize = 1;
        let size_of_instance = Some(size_of_instance(data, &mut cursor));
        let id = Cursor::new(&data[cursor..cursor + 2])
            .read_u16::<BigEndian>()
            .expect("InitialObjectDescriptor error reading id bytes");

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

        let include_inline_profile_level_flag = Some({ id & (1 << 4) > 0 });

        let url_flag = Some({ id & (1 << 5) > 0 });

        match url_flag {
            Some(v) => {
                let reserved = Some([true; 4]);
                let mut url_length = None;
                let mut url_string = None;
                let mut od_profile_level_indication = None;
                let mut scene_profile_level_indication = None;
                let mut audio_profile_level_indication = None;
                let mut visual_profile_level_indication = None;
                let mut graphics_profile_level_indication = None;
                if v {
                    url_length = match Cursor::new(&data[cursor + 2..cursor + 3]).read_u8() {
                        Ok(val) => {
                            url_string = match String::from_utf8(
                                data[cursor + 3..cursor + (val as usize)].to_vec(),
                            ) {
                                Ok(val) => Some(val),
                                Err(e) => {
                                    panic!("InitialObjectDescriptor error, cannot parse url_string. {:?}", e);
                                }
                            };
                            Some(val)
                        }
                        Err(e) => {
                            panic!(
                                "InitialObjectDescriptor error, cannot parse url_length. {:?}",
                                e
                            );
                        }
                    };
                } else {
                    od_profile_level_indication =
                        Cursor::new(&data[cursor + 2..cursor + 3]).read_u8().ok();
                    scene_profile_level_indication =
                        Cursor::new(&data[cursor + 3..cursor + 4]).read_u8().ok();
                    audio_profile_level_indication =
                        Cursor::new(&data[cursor + 4..cursor + 5]).read_u8().ok();
                    visual_profile_level_indication =
                        Cursor::new(&data[cursor + 5..cursor + 6]).read_u8().ok();
                    graphics_profile_level_indication =
                        Cursor::new(&data[cursor + 6..cursor + 7]).read_u8().ok();
                }

                let mut descriptors = descrfactory(&data[cursor + 7..]);
                Some(InitialObjectDescriptor {
                    tag,
                    size_of_instance,
                    od_id,
                    url_flag,
                    reserved,
                    include_inline_profile_level_flag,
                    url_length,
                    url_string,
                    od_profile_level_indication,
                    scene_profile_level_indication,
                    audio_profile_level_indication,
                    visual_profile_level_indication,
                    graphics_profile_level_indication,
                    descriptors,
                })
            }
            None => {
                panic!("Can't find InitialObjectDescriptor url flag!");
            }
        }
    }
}
