use {BuildNode, FullBox, Name};

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct Tkhd {
    pub fullbox: Option<FullBox>,
    pub creation_time: Option<u64>,
    pub modification_time: Option<u64>,
    pub track_id: Option<u32>,
    pub reserved_1: Option<u32>,
    pub duration: Option<u64>,
    pub reserved_2: Option<[u32; 2]>,
    pub layer: Option<u16>,
    pub alternate_group: Option<u16>,
    pub volume: Option<u16>,
    pub reserved_3: Option<u16>,
    pub matrix: Option<[u32; 9]>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl<'a> Name<'a> for Tkhd {
    fn name() -> &'a str {
        "tkhd"
    }
}

impl BuildNode for Tkhd {
    fn build(data: &[u8]) -> Option<Self> {
        use byteorder::{BigEndian, ReadBytesExt};
        use std::io::Cursor;
        let mut in_csr = 8;
        let fullbox = FullBox::from(&data[in_csr..in_csr + 4]).ok();
        let creation_time: Option<u64>;
        let modification_time: Option<u64>;
        let track_id: Option<u32>;
        let reserved_1: Option<u32>;
        let duration: Option<u64>;
        if let Some(FullBox(1, ..)) = fullbox {
            creation_time = Cursor::new(&data[12..20]).read_u64::<BigEndian>().ok();
            modification_time = Cursor::new(&data[20..28]).read_u64::<BigEndian>().ok();
            track_id = Cursor::new(&data[28..32]).read_u32::<BigEndian>().ok();
            reserved_1 = Cursor::new(&data[32..36]).read_u32::<BigEndian>().ok();
            duration = Cursor::new(&data[36..44]).read_u64::<BigEndian>().ok();
            in_csr = 44;
        } else {
            creation_time =
                Some(Cursor::new(&data[12..16]).read_u32::<BigEndian>().unwrap() as u64);
            modification_time =
                Some(Cursor::new(&data[16..20]).read_u32::<BigEndian>().unwrap() as u64);
            track_id = Cursor::new(&data[20..24]).read_u32::<BigEndian>().ok();
            reserved_1 = Cursor::new(&data[24..28]).read_u32::<BigEndian>().ok();
            duration = Some(Cursor::new(&data[28..32]).read_u32::<BigEndian>().unwrap() as u64);
            in_csr = 32;
        }
        let reserved_2 = Some([
            Cursor::new(&data[in_csr..in_csr + 4])
                .read_u32::<BigEndian>()
                .unwrap(),
            Cursor::new(&data[in_csr + 4..in_csr + 8])
                .read_u32::<BigEndian>()
                .unwrap(),
        ]);
        let layer = Cursor::new(&data[in_csr + 8..in_csr + 10])
            .read_u16::<BigEndian>()
            .ok();
        let alternate_group = Cursor::new(&data[in_csr + 10..in_csr + 12])
            .read_u16::<BigEndian>()
            .ok();
        let volume = Cursor::new(&data[in_csr + 12..in_csr + 14])
            .read_u16::<BigEndian>()
            .ok();
        let reserved_3 = Cursor::new(&data[in_csr + 14..in_csr + 16])
            .read_u16::<BigEndian>()
            .ok();
        let matrix = Some([
            Cursor::new(&data[in_csr + 16..in_csr + 20])
                .read_u32::<BigEndian>()
                .unwrap(),
            Cursor::new(&data[in_csr + 20..in_csr + 24])
                .read_u32::<BigEndian>()
                .unwrap(),
            Cursor::new(&data[in_csr + 24..in_csr + 28])
                .read_u32::<BigEndian>()
                .unwrap(),
            Cursor::new(&data[in_csr + 28..in_csr + 32])
                .read_u32::<BigEndian>()
                .unwrap(),
            Cursor::new(&data[in_csr + 32..in_csr + 36])
                .read_u32::<BigEndian>()
                .unwrap(),
            Cursor::new(&data[in_csr + 36..in_csr + 40])
                .read_u32::<BigEndian>()
                .unwrap(),
            Cursor::new(&data[in_csr + 40..in_csr + 44])
                .read_u32::<BigEndian>()
                .unwrap(),
            Cursor::new(&data[in_csr + 44..in_csr + 48])
                .read_u32::<BigEndian>()
                .unwrap(),
            Cursor::new(&data[in_csr + 48..in_csr + 52])
                .read_u32::<BigEndian>()
                .unwrap(),
        ]);
        let width = Cursor::new(&data[in_csr + 52..in_csr + 56])
            .read_u32::<BigEndian>()
            .ok();
        let height = Cursor::new(&data[in_csr + 56..in_csr + 60])
            .read_u32::<BigEndian>()
            .ok();
        Some(Tkhd {
            fullbox,
            creation_time,
            modification_time,
            track_id,
            reserved_1,
            duration,
            reserved_2,
            layer,
            alternate_group,
            volume,
            reserved_3,
            matrix,
            width,
            height,
        })
    }
}
