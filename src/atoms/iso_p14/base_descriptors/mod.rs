#[derive(Debug, Clone)]
pub enum DescrBaseTags {
    Forbidden = 0x00,
    ObjectDescrTag = 0x01,
    InitialObjectDescrTag = 0x02,
    ESDescrTag = 0x03,
    DecoderConfigDescrTag = 0x04,
    DecSpecificInfoTag = 0x05,
    SLConfigDescrTag = 0x06,
    ContentIdentDescrTag = 0x07,
    SupplContentIdentDescrTag = 0x08,
    IPIDescrPointerTag = 0x09,
    IPMPDescrPointerTag = 0x0A,
    IPMPDescrTag = 0x0B,
    QoSDescrTag = 0x0C,
    RegistrationDescrTag = 0x0D,
    ESIDIncTag = 0x0E,
    ESIDRefTag = 0x0F,
    MP4IODTag = 0x10,
    MP4ODTag = 0x11,
    IPLDescrPointerRefTag = 0x12,
    ExtensionProfileLevelDescrTag = 0x13,
    ProfileLevelIndicationIndexDescrTag = 0x14,
    Reserved1 = 0x15 - 0x3F,
    ContentClassificationDescrTag = 0x40,
    KeyWordDescrTag = 0x41,
    RatingDescrTag = 0x42,
    LanguageDescrTag = 0x43,
    ShortTextualDescrTag = 0x44,
    ExpandedTextualDescrTag = 0x45,
    ContentCreatorNameDescrTag = 0x46,
    ContentCreationDateDescrTag = 0x47,
    OCICreatorNameDescrTag = 0x48,
    OCICreationDateDescrTag = 0x49,
    SmpteCameraPositionDescrTag = 0x4A,
    SegmentDescrTag = 0x4B,
    MediaTimeDescrTag = 0x4C,
    Reserved2 = 0x4D - 0x5F,
    IPMPToolsListDescrTag = 0x60,
    IPMPToolTag = 0x61,
    M4MuxTimingDescrTag = 0x62,
    M4MuxCodeTableDescrTag = 0x63,
    ExtSLConfigDescrTag = 0x64,
    M4MuxBufferSizeDescrTag = 0x65,
    M4MuxIdentDescrTag = 0x66,
    DependencyPointerTag = 0x67,
    DependencyMarkerTag = 0x68,
    M4MuxChannelDescrTag = 0x69,
    Reserved3 = 0x6A - 0xBF,
    Userrivate = 0xC0 - 0xFE,
    Forbidden2 = 0xFF,
}

enum CommandBaseTags {
    Forbidden1 = 0x00,
    ObjectDescrUpdateTag = 0x01,
    ObjectDescrRemoveTag = 0x02,
    ESDescrUpdateTag = 0x03,
    ESDescrRemoveTag = 0x04,
    IPMPDescrUpdateTag = 0x05,
    IPMPDescrRemoveTag = 0x06,
    ESDescrRemoveRefTag = 0x07,
    ObjectDescrExecuteTag = 0x08,
    Reserved1 = 0x09 - 0xBF,
    UserPrivate = 0xC0 - 0xFE,
    Forbidden2 = 0xFF,
}

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
                let include_inline_profile_level_flag = Some(Cursor::new(&data[2..3]).read_u8().unwrap() & (1 << 5) != 0);
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
            },
        }
    }
}

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
struct ESDescriptor {}
