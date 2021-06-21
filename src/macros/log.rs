#[macro_export]
macro_rules! tree_item_debug {
    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )* , $str:literal $($arg:tt)*) => {
        log::debug!(
            "{}", 
            $crate::tree_item!($first_style_name $( : $ first_style_value )? $( ; $other_style_name $( : $other_style_value )? )* , $str $( $arg )*)
        );
    };

    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )*) => {
        log::debug!(
            "{}", 
            $crate::tree_item!($first_style_name $( : $first_style_value )? $( ; $other_style_name $( : $other_style_value )? )*)
        );
    };

    ($str:literal $($arg:tt)*) => {
        log::debug!(
            "{}", 
            $crate::tree_item!($str $( $arg )*)
        );
    };

    () => {
        log::debug!(
            "{}", 
            $crate::tree_item!()
        );
    };
}

#[macro_export]
macro_rules! tree_item_error {
    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )* , $str:literal $($arg:tt)*) => {
        log::error!(
            "{}", 
            $crate::tree_item!($first_style_name $( : $ first_style_value )? $( ; $other_style_name $( : $other_style_value )? )* , $str $( $arg )*)
        );
    };

    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )*) => {
        log::error!(
            "{}", 
            $crate::tree_item!($first_style_name $( : $first_style_value )? $( ; $other_style_name $( : $other_style_value )? )*)
        );
    };

    ($str:literal $($arg:tt)*) => {
        log::error!(
            "{}", 
            $crate::tree_item!($str $( $arg )*)
        );
    };

    () => {
        log::error!(
            "{}", 
            $crate::tree_item!()
        );
    };
}

#[macro_export]
macro_rules! tree_item_info {
    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )* , $str:literal $($arg:tt)*) => {
        log::info!(
            "{}", 
            $crate::tree_item!($first_style_name $( : $ first_style_value )? $( ; $other_style_name $( : $other_style_value )? )* , $str $( $arg )*)
        );
    };

    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )*) => {
        log::info!(
            "{}", 
            $crate::tree_item!($first_style_name $( : $first_style_value )? $( ; $other_style_name $( : $other_style_value )? )*)
        );
    };

    ($str:literal $($arg:tt)*) => {
        log::info!(
            "{}", 
            $crate::tree_item!($str $( $arg )*)
        );
    };

    () => {
        log::info!(
            "{}", 
            $crate::tree_item!()
        );
    };
}

#[macro_export]
macro_rules! tree_item_trace {
    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )* , $str:literal $($arg:tt)*) => {
        log::trace!(
            "{}", 
            $crate::tree_item!($first_style_name $( : $ first_style_value )? $( ; $other_style_name $( : $other_style_value )? )* , $str $( $arg )*)
        );
    };

    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )*) => {
        log::trace!(
            "{}", 
            $crate::tree_item!($first_style_name $( : $first_style_value )? $( ; $other_style_name $( : $other_style_value )? )*)
        );
    };

    ($str:literal $($arg:tt)*) => {
        log::trace!(
            "{}", 
            $crate::tree_item!($str $( $arg )*)
        );
    };

    () => {
        log::info!(
            "{}", 
            $crate::tree_item!()
        );
    };
}

#[macro_export]
macro_rules! tree_item_warn {
    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )* , $str:literal $($arg:tt)*) => {
        log::warn!(
            "{}", 
            $crate::tree_item!($first_style_name $( : $ first_style_value )? $( ; $other_style_name $( : $other_style_value )? )* , $str $( $arg )*)
        );
    };

    ($first_style_name:ident $( : $first_style_value:expr )? $( ; $other_style_name:ident $( : $other_style_value:expr )? )*) => {
        log::warn!(
            "{}", 
            $crate::tree_item!($first_style_name $( : $first_style_value )? $( ; $other_style_name $( : $other_style_value )? )*)
        );
    };

    ($str:literal $($arg:tt)*) => {
        log::warn!(
            "{}", 
            $crate::tree_item!($str $( $arg )*)
        );
    };

    () => {
        log::warn!(
            "{}", 
            $crate::tree_item!()
        );
    };
}
