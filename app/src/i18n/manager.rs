use rust_embed::RustEmbed;
use settings::Setting as _;
use warpui::{AppContext, Entity, ModelContext, SingletonEntity};

use super::{Catalog, I18nKey, I18nSettings, Locale};
use crate::{report_if_error, Assets};

pub struct I18n {
    locale: Locale,
    fallback: Catalog,
    active: Option<Catalog>,
}

impl I18n {
    pub fn new(ctx: &mut ModelContext<Self>) -> Self {
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

    #[allow(dead_code)]
    pub fn tr_args(&self, key: I18nKey, args: &[(&str, &str)]) -> String {
        let key_str = key.as_str();
        if let Some(active) = &self.active {
            if active.contains(key_str) {
                return active
                    .format(key_str, args)
                    .unwrap_or_else(|| key_str.to_owned());
            }
        }
        self.fallback
            .format(key_str, args)
            .unwrap_or_else(|| key_str.to_owned())
    }

    #[allow(dead_code)]
    pub fn locale(&self) -> Locale {
        self.locale
    }

    #[cfg(test)]
    pub fn from_catalogs_for_test(
        locale: Locale,
        fallback: Catalog,
        active: Option<Catalog>,
    ) -> Self {
        Self {
            locale,
            fallback,
            active,
        }
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

        Self {
            locale,
            fallback,
            active,
        }
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
