use super::{es_id_inc::ESIDInc, size_of_instance, DescrBase, DescrBaseTags, DescrBuilder};
use IsSlice;
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
    fn rdclone(&self) -> Box<DescrBase> {
        Box::new(self.clone())
    }
}

impl DescrBuilder for InitialObjectDescriptor {
    fn build<T: IsSlice<Item = u8>>(d: T) -> Option<Self> {
        let data = d.as_slice();
        use byteorder::{BigEndian, ReadBytesExt};
        use std::io::Cursor;
        let tag = Some(match Cursor::new(&data[..1]).read_u8().unwrap() {
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
            .unwrap();

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
                let descr = Box::new(ESIDInc::build(data).unwrap()) as Box<DescrBase>;
                let descriptors = vec![descr];
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
