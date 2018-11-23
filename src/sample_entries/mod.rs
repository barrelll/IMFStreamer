mod mp4_visual_sample_entry;

pub use self::mp4_visual_sample_entry::MP4VisualSampleEntry;
use byteorder::{BigEndian, ReadBytesExt};
use downcast_rs::Downcast;
use std::{
    fmt::{Debug, Formatter, Result},
    io::Cursor,
    str::from_utf8,
};

pub trait SampleEntryBase: Downcast {
    fn seclone(&self) -> Box<SampleEntryBase>;
    fn name(&self) -> String;
}

impl_downcast!(SampleEntryBase);

impl Clone for Box<SampleEntryBase> {
    fn clone(&self) -> Box<dyn SampleEntryBase> {
        self.seclone()
    }
}

impl Debug for SampleEntryBase {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "SampleEntryBase: Type {}", self.name())
    }
}

pub trait SampleBuilder {
    fn build(data: &[u8]) -> Option<Self>
    where
        Self: Sized;
}

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct SampleEntry {
    name: Option<String>,
    reserved: Option<[u8; 6]>,
    data_reference_index: Option<u16>,
}

impl SampleEntryBase for SampleEntry {
    fn seclone(&self) -> Box<SampleEntryBase> {
        Box::new(self.clone())
    }
    fn name(&self) -> String {
        String::from("SampleEntry")
    }
}

impl SampleBuilder for SampleEntry {
    fn build(data: &[u8]) -> Option<Self> {
        let name = String::from_utf8(data[4..8].to_vec()).ok();
        let reserved: Option<[u8; 6]> = Some([0; 6]);
        let data_reference_index = Cursor::new(&data[14..16]).read_u16::<BigEndian>().ok();
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
    fn name(&self) -> String {
        String::from("VisualSampleEntry")
    }
}

impl SampleBuilder for VisualSampleEntry {
    fn build(data: &[u8]) -> Option<Self> {
        let sample_entry = SampleEntry::build(data);
        let pre_defined1 = Cursor::new(&data[16..18]).read_u16::<BigEndian>().ok();
        let reserved1 = Cursor::new(&data[18..20]).read_u16::<BigEndian>().ok();
        let pre_defined2 = Some({
            [
                Cursor::new(&data[20..24])
                    .read_u32::<BigEndian>()
                    .expect("SampleBuilder, Error reading VisualSampleEntry"),
                Cursor::new(&data[24..28])
                    .read_u32::<BigEndian>()
                    .expect("SampleBuilder, Error reading VisualSampleEntry"),
                Cursor::new(&data[28..32])
                    .read_u32::<BigEndian>()
                    .expect("SampleBuilder, Error reading VisualSampleEntry"),
            ]
        });
        let width = Cursor::new(&data[32..34]).read_u16::<BigEndian>().ok();
        let height = Cursor::new(&data[34..36]).read_u16::<BigEndian>().ok();
        let horiresolution = Cursor::new(&data[36..40]).read_u32::<BigEndian>().ok();
        let vertresolution = Cursor::new(&data[40..44]).read_u32::<BigEndian>().ok();
        let reserved2 = Cursor::new(&data[44..48]).read_u32::<BigEndian>().ok();
        let frame_count = Cursor::new(&data[48..50]).read_u16::<BigEndian>().ok();
        let compressorname = Some({
            let mut array = [0; 32];
            array.copy_from_slice(&data[50..82]);
            array
        });
        let depth = Cursor::new(&data[82..84]).read_u16::<BigEndian>().ok();
        let pre_defined3 = Cursor::new(&data[84..86]).read_i16::<BigEndian>().ok();
        Some(VisualSampleEntry {
            sample_entry,
            pre_defined1,
            reserved1,
            pre_defined2,
            width,
            height,
            horiresolution,
            vertresolution,
            reserved2,
            frame_count,
            compressorname,
            depth,
            pre_defined3,
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
    fn name(&self) -> String {
        String::from("AudioSampleEntry")
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
                        MP4VisualSampleEntry::build(data)
                            .expect("samplefactory: mp4v: Error reading sample entry"),
                    ) as Box<SampleEntryBase>;
                    ret.push(vse);
                }
                any => {
                    let se = Box::new(
                        SampleEntry::build(data)
                            .expect(format!("samplefactory: {} : Error reading sample entry", any).as_ref()),
                    ) as Box<SampleEntryBase>;
                    ret.push(se);
                }
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
