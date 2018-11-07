mod es_id_inc;
mod esdescr;
mod initobjdescr;
mod objdescr;
pub use self::es_id_inc::ESIDInc;
pub use self::esdescr::ESDescriptor;
pub use self::initobjdescr::InitialObjectDescriptor;
pub use self::objdescr::ObjectDescriptor;

use downcast_rs::Downcast;
use std::fmt::{Debug, Display, Formatter, Result};
use IsSlice;

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

impl Display for DescrBaseTags {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Clone)]
pub enum CommandBaseTags {
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

impl Display for CommandBaseTags {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self)
    }
}

pub trait DescrBase: Downcast {
    fn tag(&self) -> Option<DescrBaseTags>;
    fn rdclone(&self) -> Box<DescrBase>;
}

impl_downcast!(DescrBase);

impl Clone for Box<DescrBase> {
    fn clone(&self) -> Box<dyn DescrBase> {
        self.rdclone()
    }
}

impl Debug for DescrBase {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "DescrBase {{ {:?} }}", self.tag())
    }
}

pub trait DescrBuilder {
    fn build<T: IsSlice<Item = u8>>(d: T) -> Option<Self>
    where
        Self: Sized;
}

fn size_of_instance(data: &[u8], cursor: &mut usize) -> u8 {
    let mut c = *cursor;
    use byteorder::ReadBytesExt;
    use std::io::Cursor;
    let mut size_of_instance = 0;
    let mut next_byte = true;
    while next_byte {
        let size = Cursor::new(&data[c..c + 1])
            .read_u8()
            .expect("size_of_instance error reading size byte");
        c += 1;
        next_byte = size & (1 << 7) > 0;
        size_of_instance = size_of_instance << 7 | size;
    }
    *cursor = c;
    size_of_instance
}

fn descrfactory(data: &[u8]) -> Vec<Box<DescrBase>> {
    use byteorder::ReadBytesExt;
    use std::io::Cursor;
    let len = data.len();
    let mut ret = Vec::<Box<DescrBase>>::new();
    if len == 0 {
        return ret;
    }
    let mut cursor_s = 0;
    let mut cursor_e = {
        let mut cursor = 1;
        let end = size_of_instance(&data, &mut cursor);
        (end as usize) + cursor
    };
    loop {
        match Cursor::new(&data[..1])
            .read_u8()
            .expect("descrfactory: Error reading tag")
        {
            0x03 => {
                let val = Box::new(
                    ESDescriptor::build(&data[cursor_s..cursor_e])
                        .expect("ESDescriptor not found?"),
                ) as Box<DescrBase>;
                ret.push(val);
            }
            0x0E => {
                // DescrBaseTags::ESIDInc
                let val = Box::new(
                    ESIDInc::build(&data[cursor_s..cursor_e]).expect("ESIDInc not found?"),
                ) as Box<DescrBase>;
                ret.push(val);
            }
            val => {
                println!("{:?}", val);
            }
        }
        cursor_s = cursor_e;
        if cursor_e >= len {
            break;
        } else {
            cursor_e = {
                let mut cursor = 1;
                let end = size_of_instance(&data[cursor_s..], &mut cursor);
                (end as usize) + cursor + cursor_s
            };
        }
    }
    ret
}
