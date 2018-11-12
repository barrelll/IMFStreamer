use super::{size_of_instance, DescrBase, DescrBaseTags, DescrBuilder};
use std::fmt::{Debug, Display, Formatter, Result};
use downcast_rs::Downcast;

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct DecoderSpecificInfo {
    tag: Option<DescrBaseTags>,
    size_of_instance: Option<u8>,
    extension: Option<Box<dyn DecoderSpecInfoExtension>>,
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
    pub fn build_specdecinfo(&self, _object_identifier: u8) -> DecoderSpecificInfo {
        //        let extension =
        println!("ddsadasdas");
        DecoderSpecificInfo {
            ..Default::default()
        }
    }
}

impl DescrBuilder for DecoderSpecificInfo {
    fn build(data: &[u8]) -> Option<Self> {
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
        let extension = Some(Box::new(data[cursor..cursor + size_of_instance.unwrap() as usize].to_vec()) as Box<DecoderSpecInfoExtension>);
        Some(DecoderSpecificInfo {
            tag,
            size_of_instance,
            extension,
        })
    }
}

trait DecoderSpecInfoExtension: Downcast {
    fn extension_clone(&self) -> Box<DecoderSpecInfoExtension>;
}

impl_downcast!(DecoderSpecInfoExtension);

impl DecoderSpecInfoExtension for Vec<u8> {
    fn extension_clone(&self) -> Box<DecoderSpecInfoExtension> {
        Box::new(self.to_vec())
    }
}

impl Display for DecoderSpecInfoExtension {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "DecoderSpecInfoExtension")
    }
}

impl Debug for DecoderSpecInfoExtension {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "DecoderSpecInfoExtension")
    }
}

impl Clone for Box<DecoderSpecInfoExtension> {
    fn clone(&self) -> Box<dyn DecoderSpecInfoExtension> {
        self.extension_clone()
    }
}

#[derive(Debug, Default, Clone)]
struct VisualObjectSequence;

impl DecoderSpecInfoExtension for VisualObjectSequence {
    fn extension_clone(&self) -> Box<DecoderSpecInfoExtension> {
        Box::new(self.clone())
    }
}
