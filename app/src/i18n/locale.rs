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
