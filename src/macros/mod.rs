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

#[macro_export]
macro_rules! end_all_tree_items {
    () => {
        $crate::reset_level();
    };
}

#[cfg(not(feature = "no_log"))]
#[macro_use]
mod log;
