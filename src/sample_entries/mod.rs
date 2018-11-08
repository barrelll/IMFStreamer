mod mp4_visual_sample_entry;

use self::mp4_visual_sample_entry::MP4VisualSampleEntry;
use byteorder::{BigEndian, ReadBytesExt};
use downcast_rs::Downcast;
use std::{
    fmt::{Debug, Formatter, Result},
    io::Cursor,
    str::from_utf8,
};
use IsSlice;

pub trait SampleEntryBase: Downcast {
    fn seclone(&self) -> Box<SampleEntryBase>;
}

impl_downcast!(SampleEntryBase);

impl Clone for Box<SampleEntryBase> {
    fn clone(&self) -> Box<dyn SampleEntryBase> {
        self.seclone()
    }
}

impl Debug for SampleEntryBase {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "SampleEntryBase")
    }
}

pub trait SampleBuilder {
    fn build<T: IsSlice<Item = u8>>(d: T) -> Option<Self>
    where
        Self: Sized;
}

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct SampleEntry {
    name: Option<String>,
    reserved: Option<[u8; 8]>,
    data_reference_index: Option<u16>,
}

impl SampleEntryBase for SampleEntry {
    fn seclone(&self) -> Box<SampleEntryBase> {
        Box::new(self.clone())
    }
}

impl SampleBuilder for SampleEntry {
    fn build<T: IsSlice<Item = u8>>(d: T) -> Option<Self> {
        let data = d.as_slice();
        let name = String::from_utf8(data[4..8].to_vec()).ok();
        let reserved: Option<[u8; 8]> = Some([0; 8]);
        let data_reference_index = Cursor::new(&data[40..42]).read_u16::<BigEndian>().ok();
        Some(SampleEntry {
            name,
            reserved,
            data_reference_index,
        })
    }
}

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct VisualSampleEntry {
    sample_entry: Option<SampleEntry>,
    pre_defined1: Option<u16>,
    reserved1: Option<u16>,
    pre_defined2: Option<[u32; 3]>,
    width: Option<u16>,
    height: Option<u16>,
    horiresolution: Option<u32>,
    vertresolution: Option<u32>,
    reserved2: Option<u32>,
    frame_count: Option<u16>,
    compressorname: Option<[u8; 32]>,
    depth: Option<u16>,
    pre_defined3: Option<i16>,
}

impl SampleEntryBase for VisualSampleEntry {
    fn seclone(&self) -> Box<SampleEntryBase> {
        Box::new(self.clone())
    }
}

impl SampleBuilder for VisualSampleEntry {
    fn build<T: IsSlice<Item = u8>>(d: T) -> Option<Self> {
        let data = d.as_slice();
        let sample_entry = SampleEntry::build(data);
        Some(VisualSampleEntry {
            sample_entry,
            ..Default::default()
        })
    }
}

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct AudioSampleEntry {
    sample_entry: Option<SampleEntry>,
}

impl SampleEntryBase for AudioSampleEntry {
    fn seclone(&self) -> Box<SampleEntryBase> {
        Box::new(self.clone())
    }
}

pub fn samplefactory(data: &[u8]) -> Vec<Box<SampleEntryBase>> {
    let len = data.len();
    let mut ret = Vec::<Box<SampleEntryBase>>::new();
    loop {
        let size = match Cursor::new(&data[0..4]).read_u32::<BigEndian>() {
            Ok(val) => match val {
                0 => len,
                1 => Cursor::new(&data[8..16])
                    .read_u64::<BigEndian>()
                    .expect("samplefactory, Error building samples") as usize,
                val => val as usize,
            },
            Err(e) => {
                panic!("samplefactory, Error building samples {:?}", e);
            }
        };
        match from_utf8(&data[4..8]) {
            Ok(val) => match val {
                "mp4v" => {
                    let vse = Box::new(
                        VisualSampleEntry::build(data)
                            .expect("samplefactory: mp4v: Error reading sample entry"),
                    ) as Box<SampleEntryBase>;
                    ret.push(vse);
                }
                _ => {}
            },
            Err(e) => {
                panic!("samplefactory, Error reading sample name {:?}", e);
            }
        };
        if size >= len {
            break;
        } else {

        }
    }
    ret
}
