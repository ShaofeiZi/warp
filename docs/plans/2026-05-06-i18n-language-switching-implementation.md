# I18n Language Switching Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add a maintainable application localization layer to Warp, expose a Settings -> Appearance language selector, and ship initial English plus Simplified Chinese resources for primary UI surfaces.

**Architecture:** Add `app/src/i18n/` with `Locale`, typed translation keys, resource loading, fallback lookup, and a singleton `I18n` model that observes `I18nSettings`. Store locale as a synced public setting, load bundled resources from `app/assets/bundled/i18n/`, and migrate high-frequency UI strings by calling `I18n::tr` / `I18n::tr_args` at render time.

**Tech Stack:** Rust 2021, WarpUI entity/model/settings system, `settings::define_settings_group!`, `rust-embed` app assets, simple Fluent-like key-value parser with `{name}` interpolation, `cargo test` / `cargo fmt`.

---

## Preconditions

- Work in `/Users/bytedance/warp`.
- Preserve unrelated dirty files. At plan time these exist and must not be reverted or staged unless the user explicitly asks: `/Users/bytedance/warp/app/assets/windows/arm64/OpenConsole.pdb`, `/Users/bytedance/warp/app/assets/windows/x64/OpenConsole.pdb`, `/Users/bytedance/warp/.codex/`.
- Use `git -c filter.lfs.process= -c filter.lfs.required=false -c filter.lfs.smudge= -c filter.lfs.clean= ...` for git commands if regular git fails because `git-lfs` is missing.
- Design doc: `/Users/bytedance/warp/docs/plans/2026-05-06-i18n-language-switching-design.md`.

## Task 1: Add locale type and tests

**Files:**
- Create: `/Users/bytedance/warp/app/src/i18n/mod.rs`
- Create: `/Users/bytedance/warp/app/src/i18n/locale.rs`
- Create: `/Users/bytedance/warp/app/src/i18n/locale_tests.rs`
- Modify: `/Users/bytedance/warp/app/src/lib.rs`

**Step 1: Expose the module**

Add a private module in `/Users/bytedance/warp/app/src/lib.rs` near the other `mod` declarations:

```rust
mod i18n;
```

**Step 2: Write the failing locale tests**

Create `/Users/bytedance/warp/app/src/i18n/locale_tests.rs`:

```rust
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
```

**Step 3: Run the failing test**

Run:

```bash
cargo test -p warp i18n::locale_tests --lib
```

Expected: FAIL because `app/src/i18n/locale.rs` and `Locale` do not exist yet.

**Step 4: Implement `Locale`**

Create `/Users/bytedance/warp/app/src/i18n/mod.rs`:

```rust
mod locale;

pub use locale::Locale;

#[cfg(test)]
#[path = "locale_tests.rs"]
mod locale_tests;
```

Create `/Users/bytedance/warp/app/src/i18n/locale.rs`:

```rust
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    JsonSchema,
    settings_value::SettingsValue,
)]
#[serde(rename_all = "kebab-case")]
pub enum Locale {
    #[default]
    #[serde(rename = "en-US")]
    EnUs,
    #[serde(rename = "zh-CN")]
    ZhCn,
}

impl Locale {
    pub const FALLBACK: Self = Self::EnUs;
    const SUPPORTED: [Self; 2] = [Self::EnUs, Self::ZhCn];

    pub fn supported() -> &'static [Self] {
        &Self::SUPPORTED
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value.replace('_', "-").to_ascii_lowercase().as_str() {
            "en-us" | "en" => Some(Self::EnUs),
            "zh-cn" | "zh-hans" | "zh" => Some(Self::ZhCn),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::EnUs => "en-US",
            Self::ZhCn => "zh-CN",
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::EnUs => "English",
            Self::ZhCn => "简体中文",
        }
    }

    pub fn resource_path(self) -> String {
        format!("bundled/i18n/{}.ftl", self.as_str())
    }
}
```

**Step 5: Run the test to verify it passes**

Run:

```bash
cargo test -p warp i18n::locale_tests --lib
```

Expected: PASS.

**Step 6: Commit**

Run:

```bash
git add app/src/lib.rs app/src/i18n/mod.rs app/src/i18n/locale.rs app/src/i18n/locale_tests.rs
git commit -m "feat: add display locale type"
```

If git-lfs hook warning appears after commit, note it but continue if the commit succeeds.

## Task 2: Add translation resources and resource parser

**Files:**
- Create: `/Users/bytedance/warp/app/assets/bundled/i18n/en-US.ftl`
- Create: `/Users/bytedance/warp/app/assets/bundled/i18n/zh-CN.ftl`
- Create: `/Users/bytedance/warp/app/src/i18n/catalog.rs`
- Create: `/Users/bytedance/warp/app/src/i18n/catalog_tests.rs`
- Modify: `/Users/bytedance/warp/app/src/i18n/mod.rs`

**Step 1: Write initial resource files**

Create `/Users/bytedance/warp/app/assets/bundled/i18n/en-US.ftl`:

```text
common.back = Back
common.cancel = Cancel
common.close = Close
common.delete = Delete
common.done = Done
common.next = Next
common.open = Open
common.save = Save
settings.appearance.category.language = Language
settings.appearance.language.label = Display language
settings.appearance.language.description = Choose the language used for Warp's interface.
settings.appearance.language.english = English
settings.appearance.language.zh_cn = Simplified Chinese
settings.appearance.language.changed = Warp will use {language} for supported interface text.
```

Create `/Users/bytedance/warp/app/assets/bundled/i18n/zh-CN.ftl`:

```text
common.back = 返回
common.cancel = 取消
common.close = 关闭
common.delete = 删除
common.done = 完成
common.next = 下一步
common.open = 打开
common.save = 保存
settings.appearance.category.language = 语言
settings.appearance.language.label = 显示语言
settings.appearance.language.description = 选择 Warp 界面使用的语言。
settings.appearance.language.english = English
settings.appearance.language.zh_cn = 简体中文
settings.appearance.language.changed = Warp 将对支持的界面文本使用 {language}。
```

**Step 2: Write failing catalog tests**

Create `/Users/bytedance/warp/app/src/i18n/catalog_tests.rs`:

```rust
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
    assert_eq!(catalog.format("message", &[("name", "Warp")]), Some("Hello Warp"));
}

#[test]
fn preserves_missing_interpolation_arguments() {
    let catalog = Catalog::parse("message = Hello {name}");
    assert_eq!(catalog.format("message", &[]), Some("Hello {name}"));
}

#[test]
fn english_resource_contains_required_keys() {
    let catalog = Catalog::parse(include_str!("../../assets/bundled/i18n/en-US.ftl"));
    for key in I18nKey::required() {
        assert!(catalog.contains(key.as_str()), "missing English key {}", key.as_str());
    }
}

#[test]
fn chinese_resource_contains_required_keys() {
    let catalog = Catalog::parse(include_str!("../../assets/bundled/i18n/zh-CN.ftl"));
    for key in I18nKey::required() {
        assert!(catalog.contains(key.as_str()), "missing Chinese key {}", key.as_str());
    }
}
```

**Step 3: Run the failing tests**

Run:

```bash
cargo test -p warp i18n::catalog_tests --lib
```

Expected: FAIL because `Catalog` and `I18nKey` do not exist.

**Step 4: Implement `Catalog` and `I18nKey`**

Create `/Users/bytedance/warp/app/src/i18n/catalog.rs`:

```rust
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct Catalog {
    messages: HashMap<String, String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum I18nKey {
    CommonBack,
    CommonCancel,
    CommonClose,
    CommonDelete,
    CommonDone,
    CommonNext,
    CommonOpen,
    CommonSave,
    SettingsAppearanceCategoryLanguage,
    SettingsAppearanceLanguageLabel,
    SettingsAppearanceLanguageDescription,
    SettingsAppearanceLanguageEnglish,
    SettingsAppearanceLanguageZhCn,
    SettingsAppearanceLanguageChanged,
}

impl I18nKey {
    const REQUIRED: [Self; 14] = [
        Self::CommonBack,
        Self::CommonCancel,
        Self::CommonClose,
        Self::CommonDelete,
        Self::CommonDone,
        Self::CommonNext,
        Self::CommonOpen,
        Self::CommonSave,
        Self::SettingsAppearanceCategoryLanguage,
        Self::SettingsAppearanceLanguageLabel,
        Self::SettingsAppearanceLanguageDescription,
        Self::SettingsAppearanceLanguageEnglish,
        Self::SettingsAppearanceLanguageZhCn,
        Self::SettingsAppearanceLanguageChanged,
    ];

    pub fn required() -> &'static [Self] {
        &Self::REQUIRED
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::CommonBack => "common.back",
            Self::CommonCancel => "common.cancel",
            Self::CommonClose => "common.close",
            Self::CommonDelete => "common.delete",
            Self::CommonDone => "common.done",
            Self::CommonNext => "common.next",
            Self::CommonOpen => "common.open",
            Self::CommonSave => "common.save",
            Self::SettingsAppearanceCategoryLanguage => "settings.appearance.category.language",
            Self::SettingsAppearanceLanguageLabel => "settings.appearance.language.label",
            Self::SettingsAppearanceLanguageDescription => "settings.appearance.language.description",
            Self::SettingsAppearanceLanguageEnglish => "settings.appearance.language.english",
            Self::SettingsAppearanceLanguageZhCn => "settings.appearance.language.zh_cn",
            Self::SettingsAppearanceLanguageChanged => "settings.appearance.language.changed",
        }
    }
}

impl Catalog {
    pub fn parse(contents: &str) -> Self {
        let messages = contents
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    return None;
                }
                let (key, value) = line.split_once('=')?;
                Some((key.trim().to_owned(), value.trim().to_owned()))
            })
            .collect();
        Self { messages }
    }

    pub fn contains(&self, key: &str) -> bool {
        self.messages.contains_key(key)
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.messages.get(key).map(String::as_str)
    }

    pub fn format(&self, key: &str, args: &[(&str, &str)]) -> Option<String> {
        let template = self.get(key)?;
        Some(format_template(template, args))
    }
}

fn format_template(template: &str, args: &[(&str, &str)]) -> String {
    let mut output = template.to_owned();
    for (key, value) in args {
        output = output.replace(&format!("{{{key}}}"), value);
    }
    output
}
```

Modify `/Users/bytedance/warp/app/src/i18n/mod.rs`:

```rust
mod catalog;
mod locale;

pub use catalog::{Catalog, I18nKey};
pub use locale::Locale;

#[cfg(test)]
#[path = "catalog_tests.rs"]
mod catalog_tests;
#[cfg(test)]
#[path = "locale_tests.rs"]
mod locale_tests;
```

**Step 5: Run tests to verify they pass**

Run:

```bash
cargo test -p warp i18n::catalog_tests --lib
cargo test -p warp i18n::locale_tests --lib
```

Expected: PASS.

**Step 6: Commit**

Run:

```bash
git add app/assets/bundled/i18n app/src/i18n/mod.rs app/src/i18n/catalog.rs app/src/i18n/catalog_tests.rs
git commit -m "feat: add i18n catalogs"
```

## Task 3: Add I18n settings and manager

**Files:**
- Create: `/Users/bytedance/warp/app/src/i18n/settings.rs`
- Create: `/Users/bytedance/warp/app/src/i18n/manager.rs`
- Create: `/Users/bytedance/warp/app/src/i18n/manager_tests.rs`
- Modify: `/Users/bytedance/warp/app/src/i18n/mod.rs`
- Modify: `/Users/bytedance/warp/app/src/settings/init.rs`
- Modify: `/Users/bytedance/warp/app/src/settings/mod.rs`
- Modify: `/Users/bytedance/warp/app/src/lib.rs`

**Step 1: Write failing manager tests**

Create `/Users/bytedance/warp/app/src/i18n/manager_tests.rs`:

```rust
use super::{Catalog, I18n, I18nKey, Locale};

#[test]
fn translates_from_active_locale() {
    let i18n = I18n::from_catalogs_for_test(
        Locale::ZhCn,
        Catalog::parse("common.save = Save"),
        Some(Catalog::parse("common.save = 保存")),
    );

    assert_eq!(i18n.tr(I18nKey::CommonSave), "保存");
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
        Some(Catalog::parse("settings.appearance.language.changed = Warp 将使用 {language}。")),
    );

    assert_eq!(
        i18n.tr_args(I18nKey::SettingsAppearanceLanguageChanged, &[("language", "简体中文")]),
        "Warp 将使用 简体中文。"
    );
}
```

**Step 2: Run failing tests**

Run:

```bash
cargo test -p warp i18n::manager_tests --lib
```

Expected: FAIL because `I18n` does not exist.

**Step 3: Implement settings group**

Create `/Users/bytedance/warp/app/src/i18n/settings.rs`:

```rust
use settings::{macros::define_settings_group, RespectUserSyncSetting, SupportedPlatforms, SyncToCloud};

use super::Locale;

define_settings_group!(I18nSettings, settings: [
    locale: DisplayLanguage {
        type: Locale,
        default: Locale::FALLBACK,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        private: false,
        toml_path: "appearance.language",
        description: "The display language used for Warp's interface.",
    },
]);
```

Modify `/Users/bytedance/warp/app/src/settings/mod.rs` to re-export the settings type:

```rust
pub use crate::i18n::I18nSettings;
```

Modify `/Users/bytedance/warp/app/src/settings/init.rs`:

- Add `I18nSettings` to the `use super::{ ... }` list.
- Call `I18nSettings::register(ctx);` after `ThemeSettings::register(ctx);`.

**Step 4: Implement manager**

Create `/Users/bytedance/warp/app/src/i18n/manager.rs`:

```rust
use rust_embed::RustEmbed;
use warpui::{AppContext, Entity, ModelContext, SingletonEntity};

use super::{Catalog, I18nKey, I18nSettings, Locale};
use crate::{report_if_error, Assets};
use settings::Setting as _;

pub struct I18n {
    locale: Locale,
    fallback: Catalog,
    active: Option<Catalog>,
}

impl I18n {
    pub fn new(ctx: &mut AppContext) -> Self {
        let locale = *I18nSettings::as_ref(ctx).locale.value();
        Self::load(locale)
    }

    pub fn register(ctx: &mut AppContext) {
        ctx.add_singleton_model(Self::new);
        ctx.subscribe_to_model(&I18nSettings::handle(ctx), |_, _, ctx| {
            let locale = *I18nSettings::as_ref(ctx).locale.value();
            Self::handle(ctx).update(ctx, |i18n, ctx| {
                *i18n = Self::load(locale);
                ctx.notify();
            });
        });
    }

    pub fn tr(&self, key: I18nKey) -> String {
        self.lookup(key).to_owned()
    }

    pub fn tr_args(&self, key: I18nKey, args: &[(&str, &str)]) -> String {
        let key_str = key.as_str();
        if let Some(active) = &self.active {
            if active.contains(key_str) {
                return active.format(key_str, args).unwrap_or_else(|| key_str.to_owned());
            }
        }
        self.fallback
            .format(key_str, args)
            .unwrap_or_else(|| key_str.to_owned())
    }

    pub fn locale(&self) -> Locale {
        self.locale
    }

    #[cfg(test)]
    pub fn from_catalogs_for_test(locale: Locale, fallback: Catalog, active: Option<Catalog>) -> Self {
        Self { locale, fallback, active }
    }

    fn lookup(&self, key: I18nKey) -> &str {
        let key_str = key.as_str();
        self.active
            .as_ref()
            .and_then(|catalog| catalog.get(key_str))
            .or_else(|| self.fallback.get(key_str))
            .unwrap_or(key_str)
    }

    fn load(locale: Locale) -> Self {
        let fallback = load_catalog(Locale::FALLBACK).unwrap_or_default();
        let active = (locale != Locale::FALLBACK)
            .then(|| load_catalog(locale))
            .and_then(|result| match result {
                Ok(catalog) => Some(catalog),
                Err(error) => {
                    report_if_error!(Err::<(), _>(error));
                    None
                }
            });

        Self { locale, fallback, active }
    }
}

fn load_catalog(locale: Locale) -> anyhow::Result<Catalog> {
    let path = locale.resource_path();
    let asset = <Assets as RustEmbed>::get(&path)
        .ok_or_else(|| anyhow::anyhow!("missing i18n resource {path}"))?;
    let contents = std::str::from_utf8(asset.data.as_ref())?;
    Ok(Catalog::parse(contents))
}

impl Entity for I18n {
    type Event = ();
}

impl SingletonEntity for I18n {}
```

Modify `/Users/bytedance/warp/app/src/i18n/mod.rs`:

```rust
mod catalog;
mod locale;
mod manager;
mod settings;

pub use catalog::{Catalog, I18nKey};
pub use locale::Locale;
pub use manager::I18n;
pub use settings::I18nSettings;

#[cfg(test)]
#[path = "catalog_tests.rs"]
mod catalog_tests;
#[cfg(test)]
#[path = "locale_tests.rs"]
mod locale_tests;
#[cfg(test)]
#[path = "manager_tests.rs"]
mod manager_tests;
```

Modify `/Users/bytedance/warp/app/src/lib.rs` in `initialize_app` after settings initialization has registered settings, or near other singleton setup after `settings::init(...)` has run, to call:

```rust
crate::i18n::I18n::register(ctx);
```

Place it after `settings::init` so `I18nSettings::handle(ctx)` exists.

**Step 5: Run manager tests**

Run:

```bash
cargo test -p warp i18n::manager_tests --lib
cargo test -p warp i18n::catalog_tests --lib
cargo test -p warp i18n::locale_tests --lib
```

Expected: PASS.

**Step 6: Commit**

Run:

```bash
git add app/src/i18n app/src/settings/init.rs app/src/settings/mod.rs app/src/lib.rs
git commit -m "feat: add i18n settings and manager"
```

## Task 4: Add Appearance language selector

**Files:**
- Modify: `/Users/bytedance/warp/app/src/settings_view/appearance_page.rs`
- Test: `/Users/bytedance/warp/app/src/settings_view/appearance_page.rs` existing unit compile path, or add focused tests only if an existing appearance page test harness is available.

**Step 1: Add action and view field**

In `/Users/bytedance/warp/app/src/settings_view/appearance_page.rs`:

- Add imports:

```rust
use crate::i18n::{I18n, I18nKey, Locale};
```

- Add `SetLocale(Locale)` to `AppearancePageAction`.
- Add field to `AppearanceSettingsPageView`:

```rust
language_dropdown: ViewHandle<Dropdown<AppearancePageAction>>,
```

**Step 2: Build the dropdown**

Add helper inside `impl AppearanceSettingsPageView`:

```rust
fn build_language_dropdown(ctx: &mut ViewContext<Self>) -> ViewHandle<Dropdown<AppearancePageAction>> {
    let current_locale = *crate::i18n::I18nSettings::as_ref(ctx).locale.value();
    ctx.add_typed_action_view(move |ctx| {
        let i18n = I18n::as_ref(ctx);
        let mut dropdown = Dropdown::new(ctx);
        dropdown.set_top_bar_max_width(225.);
        dropdown.set_menu_width(225., ctx);
        dropdown.add_items(
            Locale::supported()
                .iter()
                .map(|locale| {
                    let label = match locale {
                        Locale::EnUs => i18n.tr(I18nKey::SettingsAppearanceLanguageEnglish),
                        Locale::ZhCn => i18n.tr(I18nKey::SettingsAppearanceLanguageZhCn),
                    };
                    DropdownItem::new(label, AppearancePageAction::SetLocale(*locale))
                })
                .collect(),
            ctx,
        );
        dropdown.set_selected_by_action(AppearancePageAction::SetLocale(current_locale), ctx);
        dropdown
    })
}
```

In `new`, initialize `language_dropdown: Self::build_language_dropdown(ctx),`.

**Step 3: Handle action**

In `TypedActionView::handle_action`, add:

```rust
SetLocale(locale) => {
    crate::i18n::I18nSettings::handle(ctx).update(ctx, |settings, ctx| {
        report_if_error!(settings.locale.set_value(*locale, ctx));
    });
    self.language_dropdown.update(ctx, |dropdown, ctx| {
        dropdown.set_selected_by_action(AppearancePageAction::SetLocale(*locale), ctx);
    });
    ctx.notify();
}
```

Make sure `AppearancePageAction` derives or already supports `PartialEq` if required by `set_selected_by_action`. If it does not, add `PartialEq` only if all payload types support it; otherwise manually rebuild selected state by adding a helper in `Dropdown` or choose a string action payload. Prefer deriving `PartialEq` if it compiles.

**Step 4: Add the widget**

Add a `LanguageWidget` near other widget structs:

```rust
#[derive(Default)]
struct LanguageWidget;

impl SettingsWidget for LanguageWidget {
    type View = AppearanceSettingsPageView;

    fn search_terms(&self) -> &str {
        "language display locale localization chinese english"
    }

    fn render(
        &self,
        view: &Self::View,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let i18n = I18n::as_ref(app);
        render_dropdown_item(
            appearance,
            &i18n.tr(I18nKey::SettingsAppearanceLanguageLabel),
            Some(&i18n.tr(I18nKey::SettingsAppearanceLanguageDescription)),
            None,
            LocalOnlyIconState::Hidden,
            None,
            &view.language_dropdown,
        )
    }
}
```

In `build_page`, add a new first category before `Themes`:

```rust
let mut categories = vec![
    Category::new(
        "Language",
        vec![Box::new(LanguageWidget::default())],
    ),
    Category::new(
        "Themes",
        vec![
            Box::new(CreateCustomThemeWidget::default()),
            Box::new(ThemeSelectWidget::default()),
        ],
    ),
];
```

If translating category titles immediately is needed, defer until Task 5 when settings page category rendering accepts dynamic localized labels.

**Step 5: Run focused compile/tests**

Run:

```bash
cargo test -p warp i18n:: --lib
cargo test -p warp settings_view:: --lib
```

If the second command is too broad or no matching tests exist, run:

```bash
cargo check -p warp --lib
```

Expected: PASS or compile success.

**Step 6: Commit**

Run:

```bash
git add app/src/settings_view/appearance_page.rs
git commit -m "feat: add language selector to appearance settings"
```

## Task 5: Localize settings page infrastructure and Appearance labels

**Files:**
- Modify: `/Users/bytedance/warp/app/src/settings_view/settings_page.rs`
- Modify: `/Users/bytedance/warp/app/src/settings_view/appearance_page.rs`
- Modify: `/Users/bytedance/warp/app/src/i18n/catalog.rs`
- Modify: `/Users/bytedance/warp/app/assets/bundled/i18n/en-US.ftl`
- Modify: `/Users/bytedance/warp/app/assets/bundled/i18n/zh-CN.ftl`

**Step 1: Extend keys for settings categories**

Add keys to `I18nKey` and both resource files for at least:

```text
settings.section.account
settings.section.appearance
settings.section.features
settings.section.keybindings
settings.section.privacy
settings.appearance.category.themes
settings.appearance.category.icon
settings.appearance.category.window
settings.appearance.category.input
settings.appearance.category.panes
settings.appearance.category.blocks
settings.appearance.category.text
settings.appearance.category.cursor
settings.appearance.category.tabs
settings.appearance.category.full_screen_apps
```

Suggested zh-CN translations:

```text
settings.section.account = 账户
settings.section.appearance = 外观
settings.section.features = 功能
settings.section.keybindings = 快捷键
settings.section.privacy = 隐私
settings.appearance.category.themes = 主题
settings.appearance.category.icon = 图标
settings.appearance.category.window = 窗口
settings.appearance.category.input = 输入
settings.appearance.category.panes = 面板
settings.appearance.category.blocks = 块
settings.appearance.category.text = 文本
settings.appearance.category.cursor = 光标
settings.appearance.category.tabs = 标签页
settings.appearance.category.full_screen_apps = 全屏应用
```

**Step 2: Make `Category` title localizable**

In `/Users/bytedance/warp/app/src/settings_view/settings_page.rs`, replace `Category.title: &'static str` with a small enum:

```rust
#[derive(Clone, Copy)]
pub(super) enum SettingsText {
    Static(&'static str),
    I18n(crate::i18n::I18nKey),
}

impl SettingsText {
    fn render(self, app: &AppContext) -> String {
        match self {
            Self::Static(text) => text.to_owned(),
            Self::I18n(key) => crate::i18n::I18n::as_ref(app).tr(key),
        }
    }
}
```

Update `Category` and `FilteredCategory` title/subtitle fields to `SettingsText` / `Option<SettingsText>`. Keep `Category::new(title: &'static str, ...)` and add:

```rust
pub(super) fn new_i18n(title: crate::i18n::I18nKey, widgets: Vec<Box<dyn SettingsWidget<View = V>>>) -> Self
```

In `render_page`, call `category.title.render(app)` before passing to `render_sub_header`.

**Step 3: Use localized category titles in Appearance**

In `/Users/bytedance/warp/app/src/settings_view/appearance_page.rs`, change Appearance categories to use `Category::new_i18n(...)` for the categories listed above. Keep feature-gated logic unchanged.

Example:

```rust
Category::new_i18n(
    I18nKey::SettingsAppearanceCategoryThemes,
    vec![Box::new(CreateCustomThemeWidget::default()), Box::new(ThemeSelectWidget::default())],
)
```

**Step 4: Localize settings nav labels**

In `SettingsPage::render_page_button`, replace `self.section.to_string()` with a helper that maps common `SettingsSection` values to `I18nKey` and falls back to `to_string()` for sections not yet migrated.

Example:

```rust
let label = localized_settings_section(self.section, appearance_context_or_app);
.with_text_label(label + &match_data.to_string())
```

Implement `localized_settings_section(section: SettingsSection, app: &AppContext) -> String` in `settings_page.rs`.

**Step 5: Run tests**

Run:

```bash
cargo test -p warp i18n::catalog_tests --lib
cargo check -p warp --lib
```

Expected: PASS.

**Step 6: Commit**

Run:

```bash
git add app/src/settings_view/settings_page.rs app/src/settings_view/appearance_page.rs app/src/i18n/catalog.rs app/assets/bundled/i18n/en-US.ftl app/assets/bundled/i18n/zh-CN.ftl
git commit -m "feat: localize settings navigation and appearance categories"
```

## Task 6: Migrate common buttons and high-frequency UI strings

**Files:**
- Modify as needed: `/Users/bytedance/warp/app/src/settings_view/*.rs`
- Modify as needed: `/Users/bytedance/warp/app/src/modal/**/*.rs` or concrete modal files found by search
- Modify as needed: `/Users/bytedance/warp/app/src/ui_components/**/*.rs`
- Modify: `/Users/bytedance/warp/app/src/i18n/catalog.rs`
- Modify: `/Users/bytedance/warp/app/assets/bundled/i18n/en-US.ftl`
- Modify: `/Users/bytedance/warp/app/assets/bundled/i18n/zh-CN.ftl`

**Step 1: Inventory common strings**

Run:

```bash
rg -n '"(Cancel|Save|Delete|Done|Back|Next|Open|Close|Settings|Appearance|Privacy|Features|Keybindings)"' app/src/settings_view app/src/modal app/src/ui_components
```

Pick a first batch of 20-40 high-confidence user-visible strings. Do not translate logs, telemetry names, test fixtures, or debug-only strings.

**Step 2: Add keys and translations**

For each chosen string, add an `I18nKey` variant, add it to `required()` if it is part of the first-pass required UI, and add entries in `en-US.ftl` and `zh-CN.ftl`.

**Step 3: Replace render-time string literals**

Replace literals only where `AppContext` or `ViewContext` is available. Use:

```rust
let i18n = I18n::as_ref(app);
i18n.tr(I18nKey::CommonCancel)
```

or in event handlers with `ctx`:

```rust
let i18n = I18n::as_ref(ctx);
```

Do not force localization into code paths without context in this task; leave those for a later pass or pass localized strings from the caller.

**Step 4: Run resource and compile tests**

Run:

```bash
cargo test -p warp i18n::catalog_tests --lib
cargo check -p warp --lib
```

Expected: PASS.

**Step 5: Commit**

Run:

```bash
git add app/src app/assets/bundled/i18n/en-US.ftl app/assets/bundled/i18n/zh-CN.ftl
git commit -m "feat: localize common UI labels"
```

## Task 7: Migrate primary Settings pages and AI/Agent entry labels

**Files:**
- Modify likely files:
  - `/Users/bytedance/warp/app/src/settings_view/main_page.rs`
  - `/Users/bytedance/warp/app/src/settings_view/privacy_page.rs`
  - `/Users/bytedance/warp/app/src/settings_view/features_page.rs`
  - `/Users/bytedance/warp/app/src/settings_view/ai_page.rs`
  - `/Users/bytedance/warp/app/src/settings_view/code_page.rs`
  - `/Users/bytedance/warp/app/src/settings_view/mcp_servers_page.rs`
  - `/Users/bytedance/warp/app/src/ai/**` for frequent visible entry/state labels only
- Modify: `/Users/bytedance/warp/app/src/i18n/catalog.rs`
- Modify: `/Users/bytedance/warp/app/assets/bundled/i18n/en-US.ftl`
- Modify: `/Users/bytedance/warp/app/assets/bundled/i18n/zh-CN.ftl`

**Step 1: Inventory visible strings by page**

Run targeted searches:

```bash
rg -n '"[A-Z][^"\\]{2,}"' app/src/settings_view/main_page.rs app/src/settings_view/privacy_page.rs app/src/settings_view/features_page.rs app/src/settings_view/ai_page.rs app/src/settings_view/code_page.rs app/src/settings_view/mcp_servers_page.rs
```

Classify strings into:

- Rendered labels/descriptions/buttons: migrate.
- Search terms: keep English, but optionally append Chinese terms.
- Telemetry/log/debug/test data: do not migrate.
- URLs/identifiers: do not migrate.

**Step 2: Migrate one page at a time**

For each page:

1. Add keys/translations.
2. Replace render-time strings with `I18n::as_ref(app).tr(...)`.
3. Run `cargo check -p warp --lib`.
4. Commit if the page is large enough, or group small pages.

Suggested commit messages:

```bash
git commit -m "feat: localize account settings page"
git commit -m "feat: localize privacy settings page"
git commit -m "feat: localize ai settings page"
```

**Step 3: Migrate frequent AI/Agent labels**

Search:

```bash
rg -n '"(Agent|AI|Ask|Generating|Thinking|Stop|Resume|Accept|Reject|Apply|Run|Approve)"' app/src/ai app/src/pane_group app/src/workspace
```

Migrate only user-visible labels that are rendered with access to `AppContext` / `ViewContext`.

**Step 4: Run tests**

Run:

```bash
cargo test -p warp i18n::catalog_tests --lib
cargo check -p warp --lib
```

Expected: PASS.

**Step 5: Final commit for this task**

Run:

```bash
git add app/src app/assets/bundled/i18n/en-US.ftl app/assets/bundled/i18n/zh-CN.ftl
git commit -m "feat: localize primary settings and agent UI"
```

Skip this commit if each page was already committed separately and there are no remaining staged changes.

## Task 8: Add documentation for adding translations

**Files:**
- Create: `/Users/bytedance/warp/docs/i18n.md`
- Modify: `/Users/bytedance/warp/docs/plans/2026-05-06-i18n-language-switching-design.md` only if implementation decisions materially diverged from the design.

**Step 1: Write docs**

Create `/Users/bytedance/warp/docs/i18n.md`:

```markdown
# Localization

Warp UI text is localized through `app/src/i18n`.

## Adding a string

1. Add an `I18nKey` variant in `app/src/i18n/catalog.rs`.
2. Map the variant to a stable key string in `I18nKey::as_str()`.
3. Add English text to `app/assets/bundled/i18n/en-US.ftl`.
4. Add Simplified Chinese text to `app/assets/bundled/i18n/zh-CN.ftl` when the string is part of primary UI.
5. Render via `I18n::as_ref(app).tr(I18nKey::...)` or `tr_args`.

## Fallback behavior

- Active locale missing a key: English is shown.
- English missing a key: the key is shown.
- Missing interpolation arguments: placeholders remain visible.

## Do not localize

- Terminal and shell output.
- User content.
- AI model responses.
- Logs, telemetry names, protocol identifiers, and debug-only strings.
```

**Step 2: Run docs check by inspection**

Run:

```bash
sed -n '1,220p' docs/i18n.md
```

Expected: Document explains the workflow clearly.

**Step 3: Commit**

Run:

```bash
git add docs/i18n.md docs/plans/2026-05-06-i18n-language-switching-design.md
git commit -m "docs: document localization workflow"
```

If the design doc did not change, omit it from `git add`.

## Task 9: Final verification

**Files:**
- No planned source edits unless verification reveals issues.

**Step 1: Check status**

Run:

```bash
git -c filter.lfs.process= -c filter.lfs.required=false -c filter.lfs.smudge= -c filter.lfs.clean= status --short
```

Expected: only unrelated pre-existing dirty files remain, or no dirty files. Do not revert unrelated files.

**Step 2: Format**

Run:

```bash
cargo fmt
```

Expected: completes successfully.

**Step 3: Run focused tests**

Run:

```bash
cargo test -p warp i18n:: --lib
```

Expected: PASS.

**Step 4: Run compile check**

Run:

```bash
cargo check -p warp --lib
```

Expected: PASS.

**Step 5: Optional broader verification**

If time allows, run:

```bash
cargo test -p warp settings_view:: --lib
```

Expected: PASS, or no matching tests. If this is too slow, document that it was skipped and why.

**Step 6: Summarize**

Prepare a final summary listing:

- i18n framework added.
- Language setting added under Appearance.
- English and Simplified Chinese resources added.
- Main UI surfaces localized.
- Tests and checks run.
- Any known remaining English-only surfaces.
