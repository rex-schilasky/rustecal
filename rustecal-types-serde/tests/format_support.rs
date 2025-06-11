use rustecal_types_serde::format_support::{self, FormatSupport};

#[test]
fn short_type_name_for_trait() {
    assert_eq!(format_support::short_type_name::<FormatSupport>(), "FormatSupport");
}

mod nested {
    pub struct Inner;
}

#[test]
fn short_type_name_for_nested_type() {
    assert_eq!(format_support::short_type_name::<nested::Inner>(), "Inner");
}
