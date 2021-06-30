//! An utility rust lib to render pleasing tree structures at terminal programs.
//!
//! The `tree_decorator` crate goal is to simplify tree structure display while ensuring 
//! a good looking to it. So it just handle strings, returning the expected result (with 
//! current level and supplied styles) and nothing more.
//!
//! # Usage
//!
//! Before everything, a [`Decorator`] must be built using [`DecoratorBuilder`], 
//! a custom one can be used or just use the default one [`StandardDecorator`].
//!
//! ```
//! use tree_decorator::DecoratorBuilder;
//!
//! DecoratorBuilder::default()
//!                  .build();
//! ```
//!
//! After that, everything works around [`tree_item!`] macro (please, see it to detailed 
//! explanations).
//!
//! ## Styles
//!
//! Some styles can be applied to a tree item by using a defined [`Style`] struct field's name,
//! multiple ones can be used separating them with `;` and ending with `,` (see [`tree_item!`] 
//! to more examples).
//!
//! Order doesn't matter, at style list, it can be defined at any arbitrary order.
//! 
//! ### Custom Value
//!
//! By default, only using style's name, will apply an enable value (see at [`StyleItemValue`]).
//!
//! Most of it, such as [`block`], [`dashed`] or [`last`], don't need a specialized value
//! (even if it could be explicit defined), but [`entry`] should be specified using 
//! a custom [`Entry`] value.
//! 
//! ```
//! use tree_decorator::{
//!     decorator::Entry,
//!     tree_item
//! };
//!
//! tree_item!(entry: Entry::Double, "Item with style custom value");
//! ```
//!
//! ## Examples
//!
//! ```
//! use tree_decorator::tree_item;
//!
//! tree_item!(block, "Items List");
//! tree_item!("Sub Item");
//! tree_item!("Another Sub Item");
//! tree_item!(last, "Last Sub Item");
//! ```
//!
//! At some occasions, because how `tree_decorator` works, some blocks may still be opened,
//! as there is no way to know when it should be closed by context, and another [`tree_item!`] 
//! doesn't applies (or simply because it's too much blocks to close).
//!
//! There is some options:
//!
//! * Specific close one or more blocks using [`close_tree_item!`]:
//!
//! ```
//! use tree_decorator::{
//!     close_tree_item,
//!     tree_item
//! };
//!
//! tree_item!(block, "Items List");
//! tree_item!(block, "Sub Item");
//! tree_item!(block, "Another Sub Item");
//! tree_item!(last, "Last Sub Item");
//! close_tree_item!(2); // close two remaining opened blocks
//! ```
//!
//! * Use [`end_all_tree_items!`] to close it all:
//!
//! ```
//! use tree_decorator::{
//!     end_all_tree_items,
//!     tree_item
//! };
//!
//! tree_item!(block, "Items List");
//! tree_item!(block, "Sub Item");
//! tree_item!(block, "Another Sub Item");
//! tree_item!(last, "Last Sub Item");
//! end_all_tree_items!(); // close all opened blocks
//! ```
//!
//! # Features
//!
//! - `no_log`: Opt-out log dependency and it's related macros ([`tree_item_debug!`], [`tree_item_error!`], [`tree_item_info!`], [`tree_item_trace!`] and [`tree_item_warn!`]).
//!
//! [`Decorator`]: decorator::Decorator
//! [`Entry`]: decorator::Entry
//! [`Style`]: decorator::Style
//! [`StyleItemValue`]: decorator::StyleItemValue
//! [`Style::entry`]: decorator::Style::entry
//! [`block`]: decorator::Style::block
//! [`dashed`]: decorator::Style::dashed
//! [`last`]: decorator::Style::last
//! [`entry`]: decorator::Style::entry

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

/// Placeholder [`Decorator`], it does nothing.
///
/// [`Decorator`]: decorator::Decorator
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

/// Reference to current [`Decorator`].
///
/// [`Decorator`]: decorator::Decorator
pub fn decorator() -> &'static dyn Decorator {
    unsafe {
        DECORATOR
    }
}

/// Mutable reference to current [`ElementHandler`].
pub fn element_handler() -> &'static mut Option<ElementHandler> {
    unsafe {
        &mut ELEMENT_HANDLER
    }
}

/// Current block level.
pub fn level() -> u32 {
    unsafe {
        LEVEL
    }
}

/// Reset block level.
pub fn reset_level() {
    unsafe {
        if let Some(handler) = element_handler() {
            handler.clear_previous_styles();
        }

        LEVEL = 0;
    }
}

/// Reset everything to it's initial state.
pub fn shutdown() {
    unsafe {
        reset_level();
        DECORATOR = &crate::NoDecorator;
        ELEMENT_HANDLER = None;
    }
}
