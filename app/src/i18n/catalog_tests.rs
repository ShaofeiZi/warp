use super::{Catalog, I18nKey};

#[test]
fn parses_key_value_resources() {
    let catalog = Catalog::parse("common.save = Save\n# comment\ncommon.cancel = Cancel");
    assert_eq!(catalog.get("common.save"), Some("Save"));
    assert_eq!(catalog.get("common.cancel"), Some("Cancel"));
    assert_eq!(catalog.get("missing"), None);
}

#[test]
fn ignores_empty_lines_and_malformed_lines() {
    let catalog = Catalog::parse("\nmalformed\ncommon.open = Open\n");
    assert_eq!(catalog.get("common.open"), Some("Open"));
    assert_eq!(catalog.get("malformed"), None);
}

#[test]
fn interpolates_named_arguments() {
    let catalog = Catalog::parse("message = Hello {name}");
    assert_eq!(
        catalog.format("message", &[("name", "Warp")]),
        Some("Hello Warp".to_owned())
    );
}

#[test]
fn preserves_missing_interpolation_arguments() {
    let catalog = Catalog::parse("message = Hello {name}");
    assert_eq!(
        catalog.format("message", &[]),
        Some("Hello {name}".to_owned())
    );
}

#[test]
fn english_resource_contains_required_keys() {
    let catalog = Catalog::parse(include_str!("../../assets/bundled/i18n/en-US.ftl"));
    for key in I18nKey::required() {
        assert!(
            catalog.contains(key.as_str()),
            "missing English key {}",
            key.as_str()
        );
    }
}

#[test]
fn chinese_resource_contains_required_keys() {
    let catalog = Catalog::parse(include_str!("../../assets/bundled/i18n/zh-CN.ftl"));
    for key in I18nKey::required() {
        assert!(
            catalog.contains(key.as_str()),
            "missing Chinese key {}",
            key.as_str()
        );
    }
}
