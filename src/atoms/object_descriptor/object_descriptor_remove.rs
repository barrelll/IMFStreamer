struct ObjectDescriptorRemove {
    object_descriptor_id: Vec<[bool; 10]>,
}

impl super::BaseCommand for ObjectDescriptorRemove {
    fn tag(&self) -> super::DescriptorCommandTag {
        super::DescriptorCommandTag::ObjectDescrRemoveTag
    }
}