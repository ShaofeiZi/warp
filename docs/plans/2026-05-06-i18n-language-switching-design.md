# I18n Language Switching Design

Date: 2026-05-06

## Goal

Add application-level localization to Warp with an in-app language switcher. The initial release supports:

- `en-US`: default and fallback locale.
- `zh-CN`: Simplified Chinese.

The architecture should allow future locales such as `zh-TW` and `ja-JP` without redesigning the system.

## Non-goals

This first localization pass does not translate:

- Terminal or shell output.
- User project files or user-generated content.
- AI model responses.
- Telemetry event names, debug dumps, or developer logs.
- Third-party licenses, README files, developer documentation, or test fixtures.

## User Experience

Add a language selector to **Settings -> Appearance -> Language**.

The selector initially offers:

- English
- Simplified Chinese

When the user selects a language, Warp persists the preference and updates the application UI as immediately as the existing rendering architecture allows. If some windows or views do not refresh immediately, they should pick up the selected locale on the next render or app restart.

## Architecture

Add a new `app/src/i18n/` module with these responsibilities.

### Locale

`Locale` represents supported display languages. It should:

- Parse persisted locale identifiers such as `en-US` and `zh-CN`.
- Provide a stable identifier for storage.
- Provide a localized or native display name for settings UI.
- Provide the resource file name for the locale.
- Fall back safely to `en-US` when a persisted value is unknown.

### I18nSettings

`I18nSettings` integrates with the existing settings and user preferences system. It stores the selected display language and emits settings changes when the user switches languages.

### LocalizationManager / I18n

The app-level localization manager should:

- Load the default English resource and the active locale resource.
- Provide translation helpers such as `tr(key)` and `tr_args(key, args)`.
- Fall back to English when the active locale is missing a key.
- Fall back to the key itself when English is also missing the key.
- Never panic because of missing translations, invalid locale values, or missing interpolation arguments.
- Notify or otherwise trigger UI refreshes when the active locale changes.

### Translation Keys

Use stable keys rather than English source strings. Example keys:

- `settings.appearance.language.label`
- `settings.appearance.language.description`
- `common.cancel`
- `common.save`

Rust code should use constants or typed key wrappers where practical to reduce typo risk while keeping translation resources editable outside UI source files.

### Resource Files

Store translation resources under `app/resources/i18n/`:

- `app/resources/i18n/en-US.ftl`
- `app/resources/i18n/zh-CN.ftl`

Prefer Fluent-style resources because they support interpolation and future pluralization rules. If Fluent integration proves too costly during implementation, a TOML or JSON key-value format is acceptable as long as the public `tr` and `tr_args` interface remains stable.

English resources are always loaded as fallback. The active locale resource is loaded in addition when the active locale is not English.

## Data Flow

1. Warp starts and initializes settings.
2. `I18nSettings` reads the persisted locale, defaulting to `en-US`.
3. The localization manager loads English resources and the active locale resources.
4. UI code requests display strings through `tr` or `tr_args`.
5. The user opens Settings -> Appearance and changes Language.
6. The setting is persisted.
7. The localization manager switches active resources.
8. Relevant UI rerenders and displays the selected language.

## Initial UI Coverage

The first implementation should cover the primary visible app UI, prioritizing high-frequency surfaces:

- Settings navigation and major settings pages.
- Appearance page, including the new Language setting.
- Common buttons and actions such as Cancel, Save, Delete, Done, Back, Next, Open, and Close.
- Common confirmation dialogs, empty states, and status labels.
- Main workspace and panel labels.
- Frequent AI and Agent entry points and state labels.

Lower-frequency or developer-facing strings can remain English initially if translating them would create excessive risk or scope.

## Error Handling

The localization layer is failure-safe:

- Missing active-locale resource: fall back to `en-US` and log a warning.
- Missing active-locale key: show the English translation.
- Missing English key: show the key itself and log a warning.
- Missing interpolation argument: preserve the placeholder when possible and log a warning.
- Unknown persisted locale: use `en-US` and keep the settings UI valid.

## Testing

Add tests for:

1. Locale parsing and formatting.
2. Unknown locale fallback behavior.
3. Settings persistence for `zh-CN`.
4. Resource loading for `en-US` and `zh-CN`.
5. Active-locale key fallback to English.
6. Missing English key fallback to the key itself.
7. Interpolation behavior and missing interpolation arguments.
8. Appearance page language selector rendering and update behavior.
9. Resource completeness for required first-pass keys.

English should be complete for every defined key. Chinese should be complete for the required first-pass UI key set. Optional or future keys may fall back to English during incremental migration.

## Migration Strategy

Implement in phases:

1. Add the i18n module, locale type, settings group, resources, and tests.
2. Add the Appearance page language selector.
3. Convert common shared labels and buttons.
4. Convert Settings navigation and major settings pages.
5. Convert main workspace labels, common dialogs, empty states, and frequent AI/Agent UI.
6. Add resource completeness checks and document the process for future translations.

Each migration step should avoid changing business logic. Existing English behavior remains the default.

## Risk Controls

- Keep English as the default and fallback.
- Avoid panics from translation failures.
- Bundle resources with the app; do not rely on network access.
- Use stable keys so translations can evolve without code churn.
- Keep terminal output and user content untouched.
- Do not touch unrelated existing working tree changes, including Windows PDB files and `.codex/`.
