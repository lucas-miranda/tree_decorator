use crate::decorator::Style;

/// Handle tree elements by applying styles.
pub struct ElementHandler {
    previous_blocks: Vec<Block>,
    default_style: Style,
    fallback: Box<dyn FnMut(String)>
}

impl ElementHandler {
    /// [`DecoratorBuilder`] will always creates it at [`DecoratorBuilder::build`] call.
    ///
    /// [`DecoratorBuilder`]: crate::DecoratorBuilder
    /// [`DecoratorBuilder::build`]: crate::DecoratorBuilder
    pub(crate) fn new(fallback: Box<dyn FnMut(String)>) -> Self {
        Self {
            previous_blocks: Vec::new(),
            default_style: Style::default(),
            fallback
        }
    }

    /// Apply provided [`Style`] and returns it's representation.
    /// 
    /// It'll call current defined [`Decorator::item`] (defined with [`DecoratorBuilder::build`])
    /// to prepare current style and block level representation combining it with `message` contents.
    ///
    /// Every previous block levels, if applicable, with it's styles, will be combined 
    /// also to the final result.
    ///
    /// `message` contents will not change and it doesn't matter to style apply.
    ///
    /// [`DecoratorBuilder::build`]: crate::DecoratorBuilder::build
    /// [`Decorator::item`]: crate::decorator::Decorator::item
    pub fn handle_item(&mut self, message: &str, style: Option<&Style>) -> String {
        let mut m = message.to_owned();

        unsafe {
            let s = style.unwrap_or(&self.default_style);
            let item = crate::DECORATOR.item(crate::LEVEL, s);

            if !item.is_empty() {
                let block_len = crate::DECORATOR.block_length();
                let item_chars_count = item.chars().count();

                if item_chars_count < block_len {
                    m.insert_str(0, &" ".repeat(block_len - item_chars_count));
                }

                m.insert_str(0, item);
            }

            self.render_previous_levels(&mut m);

            if s.block {
                self.previous_blocks.push(Block::with_style(&s));
                crate::LEVEL += 1;
            } else if s.last {
                crate::LEVEL -= 1;
                self.previous_blocks.pop();
            }
        }

        (self.fallback)(m.clone());

        m
    }

    /// Close item blocks.
    pub fn close_item(&mut self, amount: u32) {
        if amount == 0 {
            return;
        }

        unsafe {
            crate::LEVEL = crate::LEVEL.saturating_sub(amount);
            self.previous_blocks.truncate(crate::LEVEL as usize);
        }
    }

    /// Clear all previous tracked styles
    pub(crate) fn clear_previous_styles(&mut self) {
        self.previous_blocks.clear();
    }

    /// Prepares representation of every previous block levels.
    ///
    /// By calling current [`Decorator::previous_item_block`] to prepare each block 
    /// with it's [`Style`].
    ///
    /// [`Decorator::previous_item_block`]: crate::decorator::Decorator::previous_item_block
    /// [`Style`]: crate::decorator::Style
    unsafe fn render_previous_levels(&self, m: &mut String) {
        let block_len = crate::DECORATOR.block_length();

        for previous_block in self.previous_blocks.iter().skip(1).rev() {
            let char_count = previous_block.representation
                                           .chars()
                                           .count();

            if char_count < block_len {
                m.insert_str(0, &" ".repeat(block_len - char_count));
            }

            m.insert_str(0, &previous_block.representation);
        }
    }
}

struct Block {
    pub representation: String,
    pub style: Style
}

impl Block {
    pub fn with_style(style: &Style) -> Self {
        Self {
            representation: unsafe {
                crate::DECORATOR.previous_item_block(crate::LEVEL, style)
                                .to_owned()
            },
            style: style.clone()
        }
    }
}
