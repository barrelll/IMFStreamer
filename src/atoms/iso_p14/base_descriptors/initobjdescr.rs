use super::{esdescr::ESDescriptor, DescrBaseTags};
#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct InitialObjectDescriptor {
    tag: Option<DescrBaseTags>,
    od_id: Option<[bool; 10]>,
    url_flag: Option<bool>,
    reserved: Option<[bool; 5]>,
    url_length: Option<[bool; 8]>,
    url_string: Option<String>,
    include_inline_profile_level_flag: Option<bool>,
    od_profile_level_indication: Option<u8>,
    scene_profile_level_indication: Option<u8>,
    audio_profile_level_indication: Option<u8>,
    visual_profile_level_indication: Option<u8>,
    graphics_profile_level_indication: Option<u8>,
    ext_descr: Vec<ESDescriptor>,
}

impl InitialObjectDescriptor {
    pub fn from_u8_slice(data: &[u8]) -> Option<InitialObjectDescriptor> {
        use byteorder::{BigEndian, ReadBytesExt};
        use std::io::Cursor;
        let tag = Some(match Cursor::new(&data[..1]).read_u8().unwrap() {
            0x02 => DescrBaseTags::InitialObjectDescrTag,
            0x10 => DescrBaseTags::MP4IODTag,
            _ => {
                panic!("Object descriptor tag doesn't match the object descriptor base tags");
            }
        });
        let od_id = Some({
            let mut ret_val = [false; 10];
            let val = Cursor::new(&data[1..3]).read_u16::<BigEndian>().unwrap() >> 6;
            for i in 0..10 {
                ret_val[i] = val & (1 << i + 1) != 0
            }
            ret_val
        });
        let url_flag = Some(Cursor::new(&data[2..3]).read_u8().unwrap() & (1 << 6) != 0);
        match url_flag {
            Some(true) => Some(InitialObjectDescriptor {
                tag,
                od_id,
                url_flag,
                ..Default::default()
            }),
            _ => {
                let include_inline_profile_level_flag =
                    Some(Cursor::new(&data[2..3]).read_u8().unwrap() & (1 << 5) != 0);
                let od_profile_level_indication = Cursor::new(&data[3..4]).read_u8().ok();
                let scene_profile_level_indication = Cursor::new(&data[4..5]).read_u8().ok();
                let audio_profile_level_indication = Cursor::new(&data[5..6]).read_u8().ok();
                let visual_profile_level_indication = Cursor::new(&data[6..7]).read_u8().ok();
                let graphics_profile_level_indication = Cursor::new(&data[7..8]).read_u8().ok();
                Some(InitialObjectDescriptor {
                    tag,
                    od_id,
                    url_flag,
                    include_inline_profile_level_flag,
                    od_profile_level_indication,
                    scene_profile_level_indication,
                    audio_profile_level_indication,
                    visual_profile_level_indication,
                    graphics_profile_level_indication,
                    ..Default::default()
                })
            }
        }
    }
}
