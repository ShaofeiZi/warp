use super::Locale;

#[test]
fn parses_supported_locale_identifiers() {
    assert_eq!(Locale::parse("en-US"), Some(Locale::EnUs));
    assert_eq!(Locale::parse("en-us"), Some(Locale::EnUs));
    assert_eq!(Locale::parse("zh-CN"), Some(Locale::ZhCn));
    assert_eq!(Locale::parse("zh_cn"), Some(Locale::ZhCn));
}

#[test]
fn rejects_unknown_locale_identifiers() {
    assert_eq!(Locale::parse("zh-TW"), None);
    assert_eq!(Locale::parse("fr-FR"), None);
    assert_eq!(Locale::parse(""), None);
}

#[test]
fn exposes_stable_ids_and_display_names() {
    assert_eq!(Locale::EnUs.as_str(), "en-US");
    assert_eq!(Locale::ZhCn.as_str(), "zh-CN");
    assert_eq!(Locale::EnUs.display_name(), "English");
    assert_eq!(Locale::ZhCn.display_name(), "简体中文");
}

#[test]
fn supported_locales_are_ordered_for_settings() {
    assert_eq!(Locale::supported(), &[Locale::EnUs, Locale::ZhCn]);
}
