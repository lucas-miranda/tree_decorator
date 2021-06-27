mod style_item_value;
pub use style_item_value::StyleItemValue;

mod style;
pub use style::{
    Entry,
    Style
};

/// Trait to implement a decorator.
pub trait Decorator {
    /// Single block length.
    ///
    /// It should account for any possible char, including spaces.
    fn block_length(&self) -> usize;

    /// Item matching provided [`Style`].
    ///
    /// Right side spaces can be omitted, as [`crate::ElementHandler`] 
    /// will them until match [`Decorator::block_length`] size to keep 
    /// blocks at fixed size.
    ///
    /// Current block level is also provided for custom level behavior. 
    /// (to not display any item at level 0, for example)
    fn item(&self, level: u32, style: &Style) -> &str;

    /// Previous block matching provided [`Style`].
    ///
    /// Each block before current one will use it to choose the right representation.
    fn previous_item_block(&self, level: u32, style: &Style) -> &str;
}
