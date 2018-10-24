use super::{esdescr::ESDescriptor, DescrBaseTags};

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
struct ESIDInc {
    track_id: Option<u32>,
}