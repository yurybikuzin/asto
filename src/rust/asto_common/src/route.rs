use super::*;

// use lazy_static::lazy::Lazy;
use once_cell::sync::Lazy;
// use std::cmp::Ordering;
use std::collections::HashSet;

// use strum::IntoEnumIterator;
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Route {
    Guest(GuestRoute),
    User(UserRoute),
    // pub context: Context,
    // pub modal_stack: Vec<Modal>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuestRoute {
    pub did_press: bool,
    pub search: Option<String>,
    pub kind: GuestRouteKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, strum::EnumDiscriminants)]
#[strum_discriminants(derive(strum::Display, strum::EnumIter, strum::FromRepr))]
pub enum GuestRouteKind {
    #[strum_discriminants(strum(serialize = "Танцора"))]
    Dancer {
        sort_by: DancerSortBy,
        expanded: HashSet<i32>,
    },
    #[strum_discriminants(strum(serialize = "Судьи"))]
    Judge(JudgeSortBy),
    #[strum_discriminants(strum(serialize = "Тренера"))]
    Trainer(TrainerSortBy),
    #[strum_discriminants(strum(serialize = "Клуба"))]
    Club(ClubSortBy),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserRoute();

// #[cfg(any(feature = "front"))]
// impl Default for Route {
//     fn default() -> Self {
//         // Create the Route based on the current URL
//         Self::from_url(&dominator::routing::url().lock_ref()) //.unwrap_or_else(|| Self::default_value())
//     }
// }

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    strum::FromRepr,
    strum::EnumIter,
    strum::Display,
)]
#[repr(u8)]
pub enum DancerSortBy {
    #[strum(serialize = "Ф.И.О.")]
    Name,
    #[strum(serialize = "№")]
    ExternalId,
    #[strum(serialize = "Возрасту")]
    Age,
    #[strum(serialize = "Классу")]
    Class,
    #[strum(serialize = "Очкам/баллам")]
    PointScore,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    strum::FromRepr,
    strum::EnumIter,
    strum::Display,
)]
#[repr(u8)]
pub enum JudgeSortBy {
    #[strum(serialize = "Ф.И.О.")]
    Name,
    #[strum(serialize = "№")]
    ExternalId,
    #[strum(serialize = "Категории")]
    Categori,
    #[strum(serialize = "Участиям")]
    Participations,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    strum::FromRepr,
    strum::EnumIter,
    strum::Display,
)]
#[repr(u8)]
pub enum TrainerSortBy {
    #[strum(serialize = "Ф.И.О.")]
    Name,
    #[strum(serialize = "Клубу")]
    Club,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    strum::FromRepr,
    strum::EnumIter,
    strum::Display,
)]
#[repr(u8)]
pub enum ClubSortBy {
    #[strum(serialize = "Названию")]
    Name,
    #[strum(serialize = "Городу")]
    Citi,
}

pub fn route_to_url(route: &Route) -> String {
    let ret = format!(
        "#/{}",
        URL_SAFE_BASE64.encode(rmp_serde::encode::to_vec(&route).unwrap())
    );
    ret
}

use base64::{alphabet, engine, Engine as _};
pub static BASE64: Lazy<Arc<engine::general_purpose::GeneralPurpose>> = Lazy::new(|| {
    let config = engine::GeneralPurposeConfig::new()
        .with_decode_allow_trailing_bits(true)
        .with_encode_padding(false)
        .with_decode_padding_mode(engine::DecodePaddingMode::Indifferent);
    Arc::new(engine::GeneralPurpose::new(&alphabet::STANDARD, config))
});

pub static URL_SAFE_BASE64: Lazy<Arc<engine::general_purpose::GeneralPurpose>> = Lazy::new(|| {
    let config = engine::GeneralPurposeConfig::new()
        .with_decode_allow_trailing_bits(true)
        .with_encode_padding(false)
        .with_decode_padding_mode(engine::DecodePaddingMode::Indifferent);
    Arc::new(engine::GeneralPurpose::new(&alphabet::URL_SAFE, config))
});
