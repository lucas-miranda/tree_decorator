use crate::decorator::Style;

pub struct ElementHandler {
    previous_styles: Vec<Style>,
    default_style: Style,
    fallback: Box<dyn FnMut(String)>
}

impl ElementHandler {
    pub(crate) fn new(fallback: Box<dyn FnMut(String)>) -> Self {
        Self {
            previous_styles: Vec::new(),
            default_style: Style::default(),
            fallback
        }
    }

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

    pub(crate) fn clear_previous_styles(&mut self) {
        self.previous_styles.clear();
    }

    unsafe fn render_previous_levels(&self, m: &mut String) {
        for (level, style) in self.previous_styles.iter().enumerate().skip(1).rev() {
            m.insert_str(
                0, 
                crate::DECORATOR.previous_item_block(level as u32, &style)
            );
        }
    }
}
