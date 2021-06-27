use super::Entry;

/// Style item value behavior to implement at each possible type available at [`Style`].
///
/// [`Style`]: crate::decorator::Style
pub trait StyleItemValue {
    fn default_enable_value() -> Self;
}

/// Default enable value is `true`.
impl StyleItemValue for bool {
    fn default_enable_value() -> bool {
        true
    }
}

/// Default enable value is [`Entry::Simple`].
impl StyleItemValue for Entry {
    fn default_enable_value() -> Entry {
        Entry::Simple
    }
}
