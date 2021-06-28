use crate::{
    decorator::{
        Entry,
        Style
    },
    Decorator
};

/// Default decorator implementation.
///
/// # Style
///
/// ```plain
/// Root
/// ├  Item A
/// │  Description
/// ┊- Small Description
/// └  Item B
///    ├  Item B.a
///    ├  Item B.b
///    │  └  Item B.b.a
///    └  Item B.c
/// ```
///
/// Uses a block length of 3 by default, but can be [modified](StandardDecorator::new).
/// 
/// ## Block Level 0
///
/// At root item, nothing is rendered.
///
/// ## Block Level 1 or Greater
///
/// Every following section describes a style item available to use at styles section (at
/// [`tree_item!`]).
///
/// ### Precedence
///
/// Some of them will takes precedence of others, such as [`Last`] and [`Dashed`], 
/// meaning that, when both are opt-in together, it'll use one which has the 
/// most precedence as guide to choose suitable item.
///
/// ### No Effect Combination
///
/// Also, there are combinations which has no effect, such as [`Last`] and [`Dashed`], 
/// as well, they will have no effect by lack of proper character representation or 
/// the combination simply does't makes sense.
/// 
/// When it occurs, it'll effectively ignore the least preceding style.
///
/// ### Items
///
/// #### Last
///
/// No effect: [`Style::dashed`]
///
/// Takes precedence over: [`Style::dashed`]
///
/// |     Entry Kind      | [`Style::last`] | [`Style::dashed`] |
/// | :-----------------: | :-------------: | :---------------: |
/// |  [`Entry::Simple`]  |       `└`       |        `└`        |
/// |  [`Entry::Double`]  |       `╘`       |        `╘`        |
/// |   [`Entry::None`]   |       `╵`       |        `╵`        |
///
/// #### Dashed
///
/// No effect: [`Style::last`]
///
/// |     Entry Kind      | [`Style::dashed`] | [`Style::last`][^dashed-last-note] |
/// | :-----------------: | :---------------: | :--------------: |
/// |  [`Entry::Simple`]  |       `┊-`        |        `└`       |
/// |  [`Entry::Double`]  |       `┊=`        |        `╘`       |
/// |   [`Entry::None`]   |       `┊ `        |        `╵`       |
///
///
/// [^dashed-last-note]: [`Style::last`] takes precedence here, as [explained](#precedence), 
/// so it'll display as [`Style::dashed`] was ignored, as [expected](#no-effect-combination), 
/// since there is no available combination to both of them.
///
/// [`tree_item!`]: crate::tree_item!
/// [`Style::dashed`]: crate::decorator::Style::dashed
/// [`Style::last`]: crate::decorator::Style::last
/// [`Entry::Simple`]: crate::decorator::Entry::Simple
/// [`Entry::Double`]: crate::decorator::Entry::Double
/// [`Entry::None`]: crate::decorator::Entry::None
/// [`Last`]: #last
/// [`Dashed`]: #dashed
pub struct StandardDecorator {
    block_len: usize
}

impl StandardDecorator {
    pub fn new(block_len: usize) -> Self {
        Self {
            block_len
        }
    }
}

impl Default for StandardDecorator {
    fn default() -> Self {
        StandardDecorator::new(3)
    }
}

impl Decorator for StandardDecorator {
    fn block_length(&self) -> usize {
        self.block_len
    }

    fn item(&self, level: u32, style: &Style) -> &str {
        if level == 0 {
            return "";
        }

        if style.last {
            match style.entry {
                Entry::Simple => "└",
                Entry::Double => "╘",
                Entry::None   => "╵",
            }
        } else if style.dashed {
            match style.entry {
                Entry::Simple => "┊-",
                Entry::Double => "┊=",
                Entry::None   => "┊",
            }
        } else {
            match style.entry {
                Entry::Simple => "├",
                Entry::Double => "╞",
                Entry::None   => "│",
            }
        }
    }

    fn previous_item_block(&self, level: u32, style: &Style) -> &str {
        if level == 0 {
            return "";
        }

        if style.last {
            " "
        } else if style.dashed {
            "┆"
        } else {
            "│"
        }
    }
}
