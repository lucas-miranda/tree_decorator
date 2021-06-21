use crate::{
    decorator::{
        Entry,
        Style
    },
    Decorator
};

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
            "   "
        } else if style.dashed {
            "┆  "
        } else {
            "│  "
        }
    }
}
