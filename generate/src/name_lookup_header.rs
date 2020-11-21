/// Total number of names available for lookup
pub fn num_names() -> usize {
    NAME_LOOKUP_MAP.len()
}

/// Is the name provided documented in this crate?
pub fn contains_name(name: &str) -> bool {
    NAME_LOOKUP_MAP.contains_key(name)
}

/// Get the emoji::Emoji associated with this name
pub fn lookup(name: &str) -> Option<&'static crate::Emoji> {
    NAME_LOOKUP_MAP.get(name)
}

/// An iterator over every name->emoji::Emoji pair
pub fn iter_name_emoji() -> phf::map::Entries<'static, &'static str, crate::Emoji> {
    NAME_LOOKUP_MAP.entries()   
}

/// An iterator over every name
pub fn iter_name() -> phf::map::Keys<'static, &'static str, crate::Emoji> {
    NAME_LOOKUP_MAP.keys()
}

/// An iterator over every emoji::Emoji
pub fn iter_emoji() -> phf::map::Values<'static, &'static str, crate::Emoji> {
    NAME_LOOKUP_MAP.values()
}
