mod style_item_value;
pub use style_item_value::StyleItemValue;

mod style;
pub use style::{
    Entry,
    Style
};

pub trait Decorator {
    fn block_length(&self) -> usize;
    fn item(&self, level: u32, style: &Style) -> &str;
    fn previous_item_block(&self, level: u32, style: &Style) -> &str;
}
