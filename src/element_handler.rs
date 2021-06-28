use crate::decorator::Style;

/// Handle tree elements by applying styles.
pub struct ElementHandler {
    previous_styles: Vec<Style>,
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
            previous_styles: Vec::new(),
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
                crate::LEVEL += 1;
                self.previous_styles.push(s.clone());
            } else if s.last {
                crate::LEVEL -= 1;
                self.previous_styles.pop();
            }
        }

        (self.fallback)(m.clone());

        m
    }

    /// Clear all previous tracked styles
    pub(crate) fn clear_previous_styles(&mut self) {
        self.previous_styles.clear();
    }

    /// Prepares representation of every previous block levels.
    ///
    /// By calling current [`Decorator::previous_item_block`] to prepare each block 
    /// with it's [`Style`].
    ///
    /// [`Decorator::previous_item_block`]: crate::decorator::Decorator::previous_item_block
    /// [`Style`]: crate::decorator::Style
    unsafe fn render_previous_levels(&self, m: &mut String) {
        // TODO  DECORATOR.previous_item_block result should be cached directly
        //       instead of calling it at every single item

        for (level, style) in self.previous_styles.iter().enumerate().skip(1).rev() {
            let block_len = crate::DECORATOR.block_length();
            let item = crate::DECORATOR.previous_item_block(level as u32, &style);
            let item_chars_count = item.chars().count();

            if item_chars_count < block_len {
                m.insert_str(0, &" ".repeat(block_len - item_chars_count));
            }

            m.insert_str(0, item);
        }
    }
}
