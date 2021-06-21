use super::Entry;

pub trait StyleItemValue {
    fn default_enable_value() -> Self;
}

impl StyleItemValue for bool {
    fn default_enable_value() -> bool {
        true
    }
}

impl StyleItemValue for Entry {
    fn default_enable_value() -> Entry {
        Entry::Simple
    }
}
