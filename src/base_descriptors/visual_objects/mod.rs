pub struct BitCursor<'a> {
    cursor: u64,
    data: &'a [u8],
}

pub trait VisualObjectReader {
    type Cursor;
    // functions yay
}
