/// Modifies a [`Style`] with provided style item values.
///
/// [`Style`]: crate::decorator::Decorator
#[doc(hidden)]
#[macro_export]
macro_rules! apply_style {
    ($style:expr, $name:ident : $value:expr) => {
        $style.$name = $value;
    };

    ($style:expr, $name:ident) => {
        $style.$name = $crate::decorator::StyleItemValue::default_enable_value();
    };

    ($style:expr, $name:ident : $value:expr $( ; $other_name:ident $( : $other_value:expr )? )*) => {
        $style.$name = $value;

        $( 
            $crate::apply_style!($style, $other_name $( : $other_value )? );
        )*
    };

    ($style:expr, $name:ident $( ; $other_name:ident $( : $other_value:expr )? )*) => {
        $style.$name = true;

        $( 
            $crate::apply_style!($style, $other_name $( : $other_value )? );
        )*
    };
}

/// Handles a style list of args and prepares a [`Style`] containing it all.
///
/// At current crate version, it don't need to be used directly.
///
/// [`Style`]: crate::decorator::Decorator
#[doc(hidden)]
#[macro_export]
macro_rules! handle_styles {
    ($name:ident : $value:expr $( ; $other_name:ident $( : $other_value:expr )? )*) => {{
        let mut style = $crate::handle_styles!();
        $crate::apply_style!(
            style, 
            $name : $value 
            $( ; $other_name $(: $other_value)? )*
        );

        style
    }};

    ($name:ident $( ; $other_name:ident $( : $other_value:expr )? )*) => {{
        let mut style = $crate::handle_styles!();
        $crate::apply_style!(
            style, 
            $name
            $( ; $other_name $(: $other_value)? )*
        );

        style
    }};

    () => {{
        $crate::decorator::Style::default()
    }};
}

/// Prepares tree item with provided styles.
///
/// Simplified usage overview:
///
/// ```ignore
/// tree_item!(
///     style_a; style_b: custom_value_b; style_c,
///     "A literal string which supports {} interpolation {}",
///     interpolation_arg_a, interpolation_arg_b
/// );
/// ```
///
/// Each section, `styles`, `literal string` and `interpolation args`, is optional,
/// but the order should be preserved. An `interpolation args` couldn't appears before,
/// or without, a `literal string` section.
///
/// # Styles
/// 
/// Every opt-in style must match the name of a defined [`Style`] struct field
/// and multiple ones can be declared using `;` as it's separator. A `,` marks 
/// the end of styles section.
/// 
/// Using only it's name will apply an enable value (see at [`StyleItemValue`]). 
///
/// **Note**: Order doesn't matter at styles definition.
///
/// ## Custom Value
///
/// Every style item supports a custom value, instead of just using it's name.
///
/// To the [`block`] it may not be useful, as it only has two states, on or off (since it's
/// a [`bool`]), but to the [`entry`] it's essential to define which value it should have
/// or it'll use the [default one](crate::decorator::StyleItemValue#impl-StyleItemValue).
///
/// # Literal String and Interpolation Args
///
/// It's just a literal string which may contains interpolations (using `{}`).
/// 
/// Following args should match `{}` amount.
///
/// All the rules and usage are the same as [`std::format`].
///
/// # Return
///
/// It always returns the full representation, including blocks and message, as a `&[str](std::str)`.
/// 
/// # Examples
/// 
/// ```
/// use tree_decorator::{
///     decorator::Entry,
///     tree_item
/// };
///
/// tree_item!(block, "Items");
/// tree_item!(entry: Entry::Double; dashed, "Hello {}", "World!");
/// tree_item!(last, "x = {}, y = {y}", 10, y = 30);
/// ```
///
/// [`Style`]: crate::decorator::Decorator
/// [`block`]: crate::decorator::Style::block
/// [`entry`]: crate::decorator::Style::block
/// [`StyleItemValue`]: crate::decorator::StyleItemValue
#[macro_export]
macro_rules! tree_item {
    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )* , $str:literal $($arg:tt)*) => {
        {
            if let Some(handler) = $crate::element_handler() {
                let d = $crate::handle_styles!(
                    $first_style_name $(: $first_style_value)?
                    $( ; $other_style_name $(: $other_style_value)? )*
                );

                handler.handle_item(format!($str $($arg)*).as_str(), Some(&d))
            } else {
                format!($str $($arg)*)
            }
        }.as_str()
    };

    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )*) => {
        {
            if let Some(handler) = $crate::element_handler() {
                let d = $crate::handle_styles!(
                    $first_style_name $(: $first_style_value)?
                    $( ; $other_style_name $(: $other_style_value)? )*
                );

                handler.handle_item("", Some(&d))
            } else {
                String::from("")
            }
        }.as_str()
    };

    ($str:literal $($arg:tt)*) => {
        {
            if let Some(handler) = $crate::element_handler() {
                handler.handle_item(format!($str $($arg)*).as_str(), None)
            } else {
                format!($str $($arg)*)
            }
        }.as_str()
    };

    () => {
        {
            if let Some(handler) = $crate::element_handler() {
                handler.handle_item("", None)
            } else {
                String::from("")
            }
        }.as_str()
    };
}

/// Close all opened blocks
///
/// As the usage of `tree_decorator` is immediate, we can't know when
/// a block should ends without it been explicit stated.
///
/// To the cases where a [`tree_item!`] doesn't applies anymore and
/// it should close blocks, this should be used.
#[macro_export]
macro_rules! end_all_tree_items {
    () => {
        $crate::reset_level();
    };
}

/// Close one or more tree items.
///
/// An optional arg can be supplied to close multiple tree items.
///
/// ```
/// use tree_decorator::close_tree_item;
///
/// close_tree_item!(3)
/// ```
///
/// Or omit to just close a single one.
///
/// ```
/// use tree_decorator::close_tree_item;
///
/// close_tree_item!()
/// ```
#[macro_export]
macro_rules! close_tree_item {
    ($blocks:expr) => {
        if let Some(handler) = $crate::element_handler() {
            handler.close_item($blocks);
        }
    };

    () => {
        if let Some(handler) = $crate::element_handler() {
            handler.close_item(1);
        }
    };
}

#[cfg(not(feature = "no_log"))]
#[macro_use]
mod log;
