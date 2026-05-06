use super::{Catalog, I18n, I18nKey, Locale};

#[test]
fn translates_from_active_locale() {
    let i18n = I18n::from_catalogs_for_test(
        Locale::ZhCn,
        Catalog::parse("common.save = Save"),
        Some(Catalog::parse("common.save = 保存")),
    );

    assert_eq!(i18n.tr(I18nKey::CommonSave), "保存");
    assert_eq!(i18n.locale(), Locale::ZhCn);
}

#[test]
fn falls_back_to_english_when_active_locale_key_is_missing() {
    let i18n = I18n::from_catalogs_for_test(
        Locale::ZhCn,
        Catalog::parse("common.save = Save"),
        Some(Catalog::parse("common.cancel = 取消")),
    );

    assert_eq!(i18n.tr(I18nKey::CommonSave), "Save");
}

#[test]
fn falls_back_to_key_when_english_key_is_missing() {
    let i18n = I18n::from_catalogs_for_test(Locale::EnUs, Catalog::default(), None);

    assert_eq!(i18n.tr(I18nKey::CommonSave), "common.save");
}

#[test]
fn formats_with_named_arguments() {
    let i18n = I18n::from_catalogs_for_test(
        Locale::ZhCn,
        Catalog::parse("settings.appearance.language.changed = Warp will use {language}."),
        Some(Catalog::parse(
            "settings.appearance.language.changed = Warp 将使用 {language}。",
        )),
    );

    assert_eq!(
        i18n.tr_args(
            I18nKey::SettingsAppearanceLanguageChanged,
            &[("language", "简体中文")]
        ),
        "Warp 将使用 简体中文。"
    );
}
