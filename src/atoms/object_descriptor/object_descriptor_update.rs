struct ObjectDescriptorUpdate {
    od: [Box<dyn super::BaseCommand>; 255],
}

impl super::BaseCommand for ObjectDescriptorUpdate {
    fn tag(&self) -> super::DescriptorCommandTag {
        super::DescriptorCommandTag::ObjectDescrUpdateTag
    }
}