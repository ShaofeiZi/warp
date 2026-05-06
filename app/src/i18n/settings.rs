use settings::{define_settings_group, RespectUserSyncSetting, SupportedPlatforms, SyncToCloud};

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
