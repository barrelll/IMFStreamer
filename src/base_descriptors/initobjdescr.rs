use super::{es_id_inc::ESIDInc, DescrBase, DescrBaseTags, DescrBuilder};
use IsSlice;
#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct InitialObjectDescriptor {
    tag: Option<DescrBaseTags>,
    od_id: Option<[bool; 10]>,
    url_flag: Option<bool>,
    reserved: Option<[bool; 5]>,
    url_length: Option<u8>,
    url_string: Option<String>,
    include_inline_profile_level_flag: Option<bool>,
    od_profile_level_indication: Option<u8>,
    scene_profile_level_indication: Option<u8>,
    audio_profile_level_indication: Option<u8>,
    visual_profile_level_indication: Option<u8>,
    graphics_profile_level_indication: Option<u8>,
    descriptors: Vec<Box<dyn DescrBase>>,
}

impl DescrBuilder for InitialObjectDescriptor {
    fn build<T: IsSlice<Item = u8>>(d: T) -> Option<Self> {
        let data = d.as_slice();
        use byteorder::ReadBytesExt;
        use std::io::Cursor;
        let _tag = Some(match Cursor::new(&data[..1]).read_u8().unwrap() {
            0x02 => DescrBaseTags::InitialObjectDescrTag,
            0x10 => DescrBaseTags::MP4IODTag,
            _ => {
                panic!("Object descriptor tag doesn't match the object descriptor base tags");
            }
        });
        let descr = Box::new(ESIDInc::build(data).unwrap()) as Box<DescrBase>;
        let _ = vec![descr];
        let length = Cursor::new(&data[1..2]).read_u8().unwrap();
        println!("\nlength? {:?} {:?}", length, data.len());
        None
    }
}
