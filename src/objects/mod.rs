mod visual_object_sequence;
pub use self::visual_object_sequence::VisualObjectSequence;

pub trait ObjectBuilder {
    fn build(data: &[u8]) -> Option<Self>
    where
        Self: Sized;
}