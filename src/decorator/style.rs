
#[derive(Clone)]
pub enum Entry {
    Simple,
    Double,
    None
}

/// A style to be applied to tree item.
///
/// Each field, called style item, 
/// can be opt-in by using it's name at [`tree_item!`].
#[derive(Clone)]
pub struct Style {
    /// Start a new block level.
    pub block: bool,

    /// Indicate last item on a block level.
    ///
    /// Combining with [`Style::block`] will not decrease block level.
    ///
    /// ## Last Item With New Block
    ///
    /// By using [`Style::block`] with [`Style::last`] we can achieve a last item block 
    /// while rendering previous absence of block correctly.
    ///
    /// ```
    /// use tree_decorator::tree_item;
    ///
    /// tree_item!(block, "Root Item");
    /// tree_item!("Item A");
    /// tree_item!(block; last, "Item B");
    /// tree_item!("Item B-a");
    /// tree_item!(last, "Item B-b");
    /// ```
    ///
    /// Using [`crate::StandardDecorator`], will result into:
    ///
    /// ```plain
    /// Root Item
    /// ├  Item A
    /// └  Item B
    ///    ├  Item B-a
    ///    └  Item B-b
    /// ```
    ///
    /// Without [`Style::last`] from `block; last` style combination, will result into:
    ///
    /// ```plain
    /// Root Item
    /// ├  Item A
    /// ├  Item B
    /// │  ├  Item B-a
    /// │  └  Item B-b
    /// ```
    ///
    pub last: bool,

    /// Dashed block style.
    pub dashed: bool,

    /// Entry style.
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
