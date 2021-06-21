
#[derive(Clone)]
pub enum Entry {
    Simple,
    Double,
    None
}

#[derive(Clone)]
pub struct Style {
    /// Start a new block
    pub block: bool,

    /// End current block
    pub last: bool,

    /// Dashed block style
    pub dashed: bool,

    pub entry: Entry
}

impl Default for Style {
    fn default() -> Self {
        Self {
            block: false,
            last: false,
            dashed: false,
            entry: Entry::Simple
        }
    }
}
