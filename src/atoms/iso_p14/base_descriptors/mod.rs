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
pub struct ObjectDescriptor {
    tag: Option<DescrBaseTags>,
    od_id: Option<[bool; 10]>,
    url_flag: Option<bool>,
    reserved: Option<[bool; 5]>,
    url_length: Option<[bool; 8]>,
    url_string: Option<String>,
    ext_descr: Option<u32>,
}

impl ObjectDescriptor {
    pub fn from_u8_slice(data: &[u8]) -> Option<ObjectDescriptor> {
        use byteorder::{BigEndian, ReadBytesExt};
        use std::io::Cursor;
        let tag = Some(match Cursor::new(&data[..1]).read_u8().unwrap() {
            0x01 => DescrBaseTags::ObjectDescrTag,
            0x02 => DescrBaseTags::InitialObjectDescrTag,
            0x10 => DescrBaseTags::MP4IODTag,
            0x11 => DescrBaseTags::MP4ODTag,
            _ => {
                panic!("Object descriptor tag doesn't match the object descriptor base tags");
            }
        });
        let orig = Cursor::new(&data[1..3]).read_u16::<BigEndian>().unwrap() >> 6;
        let od_id = Some({
            let mut ret_val = [false; 10];
            let val = Cursor::new(&data[1..3]).read_u16::<BigEndian>().unwrap() >> 6;
            for i in 0..10 {
                ret_val[i] = val & (1 << i + 1) != 0
            }
            ret_val
        });
        let url_flag =
            Some(Cursor::new(&data[1..3]).read_u16::<BigEndian>().unwrap() & (1 << 11) != 0);

        //        if url_flag == 1 {
        //            let url_length = Cursor::new(&data[4..5]).read_u8().unwrap();
        //            let url_string = String::from_utf8(data[5..url_length as usize].to_vec());
        //            println!("url_length {:?} {:b} {:x} {:X}", url_length, url_length, url_length, url_length);
        //            println!("url_string {:?}", url_string);
        //        } else {
        //            // build more descriptors
        //        }
        Some(ObjectDescriptor {
            tag,
            od_id,
            url_flag,
            ..Default::default()
        })
    }
}
