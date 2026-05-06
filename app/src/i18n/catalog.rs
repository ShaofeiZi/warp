use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct Catalog {
    messages: HashMap<String, String>,
}

macro_rules! define_i18n_keys {
    ($($variant:ident => $key:literal,)+) => {
        #[allow(dead_code)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum I18nKey {
            $($variant,)+
        }

        impl I18nKey {
            #[cfg(test)]
            const REQUIRED: &'static [Self] = &[
                $(Self::$variant,)+
            ];

            #[cfg(test)]
            pub fn required() -> &'static [Self] {
                Self::REQUIRED
            }

            pub fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $key,)+
                }
            }
        }
    };
}

define_i18n_keys! {
    CommonBack => "common.back",
    CommonCancel => "common.cancel",
    CommonClose => "common.close",
    CommonDelete => "common.delete",
    CommonDone => "common.done",
    CommonNext => "common.next",
    CommonOpen => "common.open",
    CommonSave => "common.save",
    CommonSettings => "common.settings",
    CommonAppearance => "common.appearance",
    CommonPrivacy => "common.privacy",
    CommonFeatures => "common.features",
    CommonKeybindings => "common.keybindings",
    CommonAccept => "common.accept",
    CommonReject => "common.reject",
    CommonApply => "common.apply",
    CommonRun => "common.run",
    CommonStop => "common.stop",
    CommonResume => "common.resume",
    CommonApprove => "common.approve",
    SettingsSectionAccount => "settings.section.account",
    SettingsSectionAppearance => "settings.section.appearance",
    SettingsSectionFeatures => "settings.section.features",
    SettingsSectionKeybindings => "settings.section.keybindings",
    SettingsSectionPrivacy => "settings.section.privacy",
    SettingsSectionBillingAndUsage => "settings.section.billing_and_usage",
    SettingsSectionSharedBlocks => "settings.section.shared_blocks",
    SettingsSectionMcpServers => "settings.section.mcp_servers",
    SettingsSectionWarpDrive => "settings.section.warp_drive",
    SettingsSectionWarpAgent => "settings.section.warp_agent",
    SettingsSectionAgentProfiles => "settings.section.agent_profiles",
    SettingsSectionAgentMcpServers => "settings.section.agent_mcp_servers",
    SettingsSectionKnowledge => "settings.section.knowledge",
    SettingsSectionThirdPartyCliAgents => "settings.section.third_party_cli_agents",
    SettingsSectionCodeIndexing => "settings.section.code_indexing",
    SettingsSectionEditorAndCodeReview => "settings.section.editor_and_code_review",
    SettingsSectionCloudEnvironments => "settings.section.cloud_environments",
    SettingsSectionCloudApiKeys => "settings.section.cloud_api_keys",
    SettingsSectionTeams => "settings.section.teams",
    SettingsSectionWarpify => "settings.section.warpify",
    SettingsSectionReferrals => "settings.section.referrals",
    SettingsSectionAbout => "settings.section.about",
    SettingsSectionAi => "settings.section.ai",
    SettingsSectionCode => "settings.section.code",
    SettingsUmbrellaAgents => "settings.umbrella.agents",
    SettingsUmbrellaCode => "settings.umbrella.code",
    SettingsUmbrellaCloudPlatform => "settings.umbrella.cloud_platform",
    SettingsAppearanceCategoryLanguage => "settings.appearance.category.language",
    SettingsAppearanceCategoryThemes => "settings.appearance.category.themes",
    SettingsAppearanceCategoryIcon => "settings.appearance.category.icon",
    SettingsAppearanceCategoryWindow => "settings.appearance.category.window",
    SettingsAppearanceCategoryInput => "settings.appearance.category.input",
    SettingsAppearanceCategoryPanes => "settings.appearance.category.panes",
    SettingsAppearanceCategoryBlocks => "settings.appearance.category.blocks",
    SettingsAppearanceCategoryText => "settings.appearance.category.text",
    SettingsAppearanceCategoryCursor => "settings.appearance.category.cursor",
    SettingsAppearanceCategoryTabs => "settings.appearance.category.tabs",
    SettingsAppearanceCategoryFullScreenApps => "settings.appearance.category.full_screen_apps",
    SettingsAppearanceLanguageLabel => "settings.appearance.language.label",
    SettingsAppearanceLanguageDescription => "settings.appearance.language.description",
    SettingsAppearanceLanguageEnglish => "settings.appearance.language.english",
    SettingsAppearanceLanguageZhCn => "settings.appearance.language.zh_cn",
    SettingsAppearanceLanguageChanged => "settings.appearance.language.changed",
    SettingsAppearanceLanguageSearchTerms => "settings.appearance.language.search_terms",
    SettingsAppearanceCreateCustomTheme => "settings.appearance.create_custom_theme",
    SettingsAppearanceTheme => "settings.appearance.theme",
    SettingsAppearanceSyncWithOs => "settings.appearance.sync_with_os",
    SettingsAppearanceSyncWithOsDescription => "settings.appearance.sync_with_os.description",
    SettingsAppearanceCustomizeAppIcon => "settings.appearance.customize_app_icon",
    SettingsAppearanceAppIconBundleWarning => "settings.appearance.app_icon.bundle_warning",
    SettingsAppearanceUseThinStrokes => "settings.appearance.use_thin_strokes",
    SettingsAppearanceEnforceMinimumContrast => "settings.appearance.enforce_minimum_contrast",
    SettingsAppearanceTabCloseButtonPosition => "settings.appearance.tab_close_button_position",
    SettingsAppearanceShowTabBar => "settings.appearance.show_tab_bar",
    SettingsAppearanceZoom => "settings.appearance.zoom",
    SettingsAppearanceZoomDescription => "settings.appearance.zoom.description",
    CommonResetToDefault => "common.reset_to_default",
    SettingsAppearanceWindowOpacity => "settings.appearance.window_opacity",
    SettingsAppearanceWindowBlur => "settings.appearance.window_blur",
    SettingsAppearanceInputPosition => "settings.appearance.input_position",
    SettingsAppearanceCursorBlink => "settings.appearance.cursor_blink",
    SettingsAppearanceTerminalFont => "settings.appearance.terminal_font",
    SettingsAppearanceAiFont => "settings.appearance.ai_font",
    SettingsAiAgent => "settings.ai.agent",
    SettingsAiAskAi => "settings.ai.ask_ai",
    SettingsAiGenerating => "settings.ai.generating",
    SettingsAiThinking => "settings.ai.thinking",
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

    #[allow(dead_code)]
    pub fn contains(&self, key: &str) -> bool {
        self.messages.contains_key(key)
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.messages.get(key).map(String::as_str)
    }

    #[allow(dead_code)]
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
