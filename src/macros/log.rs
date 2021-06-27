/// Tree item call with a [`log::debug!`].
///
/// It don't do anything extra, just calls [`tree_item!`] and
/// gives the returned value to a [`log::debug!`].
///
/// Works the same as:
///
/// ```
/// use tree_decorator::tree_item;
///
/// log::debug!("{}", tree_item!(dashed, "My tree item"));
/// ```
///
/// But looks more concise and simple:
///
/// ```
/// use tree_decorator::tree_item_debug;
///
/// tree_item_debug!(dashed, "My tree item");
/// ```
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

/// Tree item call with a [`log::error!`].
///
/// It don't do anything extra, just calls [`tree_item!`] and
/// gives the returned value to a [`log::error!`].
///
/// Works the same as:
///
/// ```
/// use tree_decorator::tree_item;
///
/// log::error!("{}", tree_item!(dashed, "My tree item"));
/// ```
///
/// But looks more concise and simple:
///
/// ```
/// use tree_decorator::tree_item_error;
///
/// tree_item_error!(dashed, "My tree item");
/// ```
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

/// Tree item call with a [`log::info!`].
///
/// It don't do anything extra, just calls [`tree_item!`] and
/// gives the returned value to a [`log::info!`].
///
/// Works the same as:
///
/// ```
/// use tree_decorator::tree_item;
///
/// log::info!("{}", tree_item!(dashed, "My tree item"));
/// ```
///
/// But looks more concise and simple:
///
/// ```
/// use tree_decorator::tree_item_info;
///
/// tree_item_info!(dashed, "My tree item");
/// ```
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

/// Tree item call with a [`log::trace!`].
///
/// It don't do anything extra, just calls [`tree_item!`] and
/// gives the returned value to a [`log::trace!`].
///
/// Works the same as:
///
/// ```
/// use tree_decorator::tree_item;
///
/// log::trace!("{}", tree_item!(dashed, "My tree item"));
/// ```
///
/// But looks more concise and simple:
///
/// ```
/// use tree_decorator::tree_item_trace;
///
/// tree_item_trace!(dashed, "My tree item");
/// ```
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


/// Tree item call with a [`log::warn!`].
///
/// It don't do anything extra, just calls [`tree_item!`] and
/// gives the returned value to a [`log::warn!`].
///
/// Works the same as:
///
/// ```
/// use tree_decorator::tree_item;
///
/// log::warn!("{}", tree_item!(dashed, "My tree item"));
/// ```
///
/// But looks more concise and simple:
///
/// ```
/// use tree_decorator::tree_item_warn;
///
/// tree_item_warn!(dashed, "My tree item");
/// ```
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
