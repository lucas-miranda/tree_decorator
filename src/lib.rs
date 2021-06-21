pub mod decorator;
pub use decorator::{
    Decorator,
    Style
};

mod standard_decorator;
pub use standard_decorator::StandardDecorator;

mod decorator_builder;
pub use decorator_builder::DecoratorBuilder;

mod element_handler;
pub use element_handler::ElementHandler;

#[macro_use]
mod macros;

// state
static mut LEVEL: u32 = 0;

static mut DECORATOR: &dyn Decorator = &NoDecorator;
static mut ELEMENT_HANDLER: Option<ElementHandler> = None;

//

pub struct NoDecorator;

impl Decorator for NoDecorator {
    fn block_length(&self) -> usize {
        0
    }

    fn item(&self, _level: u32, _style: &Style) -> &str {
        ""
    }

    fn previous_item_block(&self, _level: u32, _style: &Style) -> &str {
        ""
    }
}

//

pub fn decorator() -> &'static dyn Decorator {
    unsafe {
        DECORATOR
    }
}

pub fn element_handler() -> &'static mut Option<ElementHandler> {
    unsafe {
        &mut ELEMENT_HANDLER
    }
}

pub fn level() -> u32 {
    unsafe {
        LEVEL
    }
}

pub fn reset_level() {
    unsafe {
        if let Some(handler) = element_handler() {
            handler.clear_previous_styles();
        }

        LEVEL = 0;
    }
}

pub fn shutdown() {
    unsafe {
        reset_level();
        DECORATOR = &crate::NoDecorator;
        ELEMENT_HANDLER = None;
    }
}
