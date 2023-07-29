#![recursion_limit = "1024"] // to fix issue whilst map_ref!: recursion limit reached while expanding `$crate::__internal_map_lets!`

#[allow(unused_imports)]
use anyhow::anyhow;

#[allow(unused_imports)]
use web_sys_utils::{debug, error, warn};

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
use built_info::*;

#[allow(unused_imports)]
use dominator::{clone, events, html, link, routing, routing::go_to_url, with_node, Dom};

#[allow(unused_imports)]
use futures_signals::{
    map_ref,
    signal::{always, Mutable, Signal, SignalExt},
    signal_map::{MapDiff, MutableBTreeMap, SignalMapExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt, VecDiff},
};

#[allow(unused_imports)]
use web_sys::{
    window, ErrorEvent, HtmlInputElement, HtmlSelectElement, MessageEvent, Url, WebSocket,
};

use gloo_timers::callback::Timeout;
use once_cell::sync::Lazy;
use op_mode::OpMode;
use std::sync::Arc;
use wasm_bindgen::{prelude::*, JsCast};

mod common;
use common::*;

mod render;

#[wasm_bindgen(start)]
pub async fn main_js() -> Result<(), JsValue> {
    common::main_js_helper()
}

// ==================================================
// ==================================================

fn process_server_message(server_message: ServerMessage) {
    let server_message_discriminants = ServerMessageDiscriminants::from(&server_message);
    match server_message_discriminants {
        ServerMessageDiscriminants::Pong => common::respond_to_pong(server_message),
        ServerMessageDiscriminants::Version => common::respond_to_version(server_message),
        // ServerMessageDiscriminants::NeedLogin => common::respond_to_need_login(server_message),
        // ServerMessageDiscriminants::Login => common::respond_to_login(server_message),
        ServerMessageDiscriminants::InitData => common::respond_to_init_data(server_message),
        // ==================================================
        // ==================================================
        // You have to customize:
        // - here
        ServerMessageDiscriminants::Commit => respond_to_commit(server_message),
        // ==================================================
        // ==================================================
    }
}

// ==================================================
// ==================================================
// You have to customize:
// - here

use asto_common::*;
use render::DancerScorePeriod;

pub fn respond_to_commit(_server_message: ServerMessage) {
    todo!();
}

pub struct App {
    pub data: AppData,
}

#[derive(Default)]
pub struct AppData {
    pub is_alive_ws: Mutable<bool>,
    pub route: Mutable<Option<Route>>,
    pub is_in_commit: Mutable<bool>,

    pub dancerlar: Mutable<Vec<Arc<Dancer>>>,
    pub judgelar: Mutable<Vec<Arc<Judge>>>,
    pub trainerlar: Mutable<Vec<Arc<Trainer>>>,
    pub clublar: Mutable<Vec<Arc<Club>>>,

    pub textlar_map: MutableBTreeMap<i32, Arc<Text>>,
    pub dancerlar_map: MutableBTreeMap<i32, Arc<Dancer>>,
    pub citilar_map: MutableBTreeMap<i16, Arc<Citi>>,
    pub clublar_map: MutableBTreeMap<i16, Arc<Club>>,
    pub categorilar_map: MutableBTreeMap<i8, Arc<Categori>>,
    pub classlar_map: MutableBTreeMap<i8, Arc<Class>>,
    pub first_namelar_map: MutableBTreeMap<i16, Arc<FirstName>>,
    pub second_namelar_map: MutableBTreeMap<i16, Arc<SecondName>>,
    pub last_namelar_map: MutableBTreeMap<i16, Arc<LastName>>,
    pub nick_namelar_map: MutableBTreeMap<i16, Arc<NickName>>,
    pub genderlar_map: MutableBTreeMap<i8, Arc<Gender>>,
    pub personlar_map: MutableBTreeMap<i32, Arc<Person>>,
    pub judgelar_map: MutableBTreeMap<i32, Arc<Judge>>,
    pub trainerlar_map: MutableBTreeMap<i32, Arc<Trainer>>,
    pub suggest_only: Mutable<Option<String>>,

    pub eventlar_map: MutableBTreeMap<i32, Arc<Event>>,
    pub event_resultlar: MutableBTreeMap<i32 /* external_id */, Vec<Arc<EventResult>>>,

    pub public_search: Mutable<Option<String>>,
    pub did_press: Mutable<bool>,

    pub suggest_input: Mutable<Option<String>>,
    pub suggest_selected: Mutable<bool>,

    pub dancer_filter: Mutable<Option<(String, usize)>>,
    pub filtered_dancerlar: MutableVec<Arc<Dancer>>,

    pub judge_filter: Mutable<Option<(String, usize)>>,
    pub filtered_judgelar: MutableVec<Arc<Judge>>,

    pub today: Mutable<Option<chrono::NaiveDate>>,
    pub classlar: MutableVec<Arc<Class>>,

    pub last_namelar: MutableVec<Arc<LastName>>,

    pub personlar: MutableVec<Arc<Person>>,
}

impl AppData {
    pub fn clublar_signal_vec(&self) -> impl SignalVec<Item = Arc<Club>> {
        self.clublar_map
            .entries_cloned()
            .map(|(_, value)| value)
            .sort_by_cloned(|a, b| a.value.partial_cmp(&b.value).unwrap())
    }
}

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;

use asto_common::route::*;

impl App {
    pub fn new_helper() -> Self {
        Self {
            data: AppData::default(),
        }
    }
    pub fn init_data_key() -> InitDataKey {
        let op_mode = *OP_MODE.read().unwrap();
        InitDataKey { op_mode }
    }
    pub fn init(init_data: InitData) {
        APP.data.today.lock_mut().replace(init_data.today.unwrap());

        APP.data.eventlar_map.lock_mut().replace_cloned(
            init_data
                .eventlar
                .iter()
                .map(|item| (item.id, Arc::new(item.clone())))
                .collect(),
        );
        APP.data.event_resultlar.lock_mut().replace_cloned({
            let mut ret = std::collections::HashMap::<i32, Vec<Arc<EventResult>>>::new();
            for item in init_data.event_resultlar.iter().cloned().map(Arc::new) {
                common_macros2::entry!(ret, item.external_id
                =>
                       and_modify |e| { e.push(item) }
                       or_insert vec![item]
                );
            }
            ret.into_iter().collect()
        });
        APP.data.textlar_map.lock_mut().replace_cloned(
            init_data
                .textlar
                .iter()
                .map(|item| (item.id, Arc::new(item.clone())))
                .collect(),
        );
        APP.data.dancerlar_map.lock_mut().replace_cloned(
            init_data
                .dancerlar
                .iter()
                .map(|item| (item.id, Arc::new(item.clone())))
                .collect(),
        );
        APP.data.citilar_map.lock_mut().replace_cloned(
            init_data
                .citilar
                .iter()
                .map(|item| (item.id, Arc::new(item.clone())))
                .collect(),
        );
        APP.data.clublar_map.lock_mut().replace_cloned(
            init_data
                .clublar
                .iter()
                .map(|item| (item.id, Arc::new(item.clone())))
                .collect(),
        );
        APP.data.categorilar_map.lock_mut().replace_cloned(
            init_data
                .categorilar
                .iter()
                .map(|item| (item.id, Arc::new(item.clone())))
                .collect(),
        );
        APP.data.classlar_map.lock_mut().replace_cloned(
            init_data
                .classlar
                .iter()
                .map(|item| (item.id, Arc::new(item.clone())))
                .collect(),
        );
        APP.data.first_namelar_map.lock_mut().replace_cloned(
            init_data
                .first_namelar
                .iter()
                .map(|item| (item.id, Arc::new(item.clone())))
                .collect(),
        );
        APP.data.second_namelar_map.lock_mut().replace_cloned(
            init_data
                .second_namelar
                .iter()
                .map(|item| (item.id, Arc::new(item.clone())))
                .collect(),
        );
        APP.data.last_namelar_map.lock_mut().replace_cloned(
            init_data
                .last_namelar
                .iter()
                .map(|item| (item.id, Arc::new(item.clone())))
                .collect(),
        );
        APP.data.nick_namelar_map.lock_mut().replace_cloned(
            init_data
                .nick_namelar
                .iter()
                .map(|item| (item.id, Arc::new(item.clone())))
                .collect(),
        );
        APP.data.genderlar_map.lock_mut().replace_cloned(
            init_data
                .genderlar
                .iter()
                .map(|item| (item.id, Arc::new(item.clone())))
                .collect(),
        );
        APP.data.personlar_map.lock_mut().replace_cloned(
            init_data
                .personlar
                .iter()
                .map(|item| (item.id, Arc::new(item.clone())))
                .collect(),
        );
        APP.data.judgelar_map.lock_mut().replace_cloned(
            init_data
                .judgelar
                .iter()
                .map(|item| (item.id, Arc::new(item.clone())))
                .collect(),
        );
        APP.data.trainerlar_map.lock_mut().replace_cloned(
            init_data
                .trainerlar
                .iter()
                .map(|item| (item.id, Arc::new(item.clone())))
                .collect(),
        );
        loaded()
    }
}

#[derive(Clone)]
pub struct GuestMenuItem {
    caption: &'static str,
    href: &'static str,
}
const GUEST_MENU_ITEMLAR: &[GuestMenuItem] = &[
    GuestMenuItem {
        caption: "Об ассоциации",
        href: "http://hundred.su/about",
    },
    GuestMenuItem {
        caption: "Новости ",
        href: "http://hundred.su/#rec231191460",
    },
    GuestMenuItem {
        caption: "Расписание фестивалей",
        href: "http://hundred.su/#rec238907862",
    },
    GuestMenuItem {
        caption: "Документы ",
        href: "http://hundred.su/docs",
    },
    GuestMenuItem {
        caption: "Руководство",
        href: "http://hundred.su/#rec225751762",
    },
    GuestMenuItem {
        caption: "Объявления",
        href: "http://hundred.su/#rec230107842",
    },
    GuestMenuItem {
        caption: "Контакты",
        href: "http://hundred.su/#rec225745394",
    },
];

// ==================================================

fn route_default() -> Route {
    // Create the Route based on the current URL
    route_from_url(&dominator::routing::url().lock_ref()) //.unwrap_or_else(|| Self::default_value())
}

// ==================================================

pub fn trainer_sort_by_cmp(
    trainer_sort_by: &TrainerSortBy,
    a: &Arc<Trainer>,
    b: &Arc<Trainer>,
    personlar_map: &std::collections::BTreeMap<i32, Arc<Person>>,
) -> Ordering {
    match trainer_sort_by {
        TrainerSortBy::Name => {
            let a_person = personlar_map.get(&a.person);
            let b_person = personlar_map.get(&b.person);
            sort_person_by_name(a_person, b_person)
        }
        TrainerSortBy::Club => sort_trainer_by_club(a, b),
    }
}

// ==================================================

pub fn judge_sort_by_cmp(
    judge_sort_by: &JudgeSortBy,
    a: &Arc<Judge>,
    b: &Arc<Judge>,
    personlar_map: &std::collections::BTreeMap<i32, Arc<Person>>,
) -> Ordering {
    match judge_sort_by {
        JudgeSortBy::Name => {
            let a_person = personlar_map.get(&a.person);
            let b_person = personlar_map.get(&b.person);
            sort_person_by_name(a_person, b_person)
        }
        JudgeSortBy::ExternalId => match (a.external_id, b.external_id) {
            (Some(a_external_id), Some(b_external_id)) => a_external_id.cmp(&b_external_id),
            (None, Some(_)) => Ordering::Greater,
            (Some(_), None) => Ordering::Less,
            (None, None) => {
                let a_person = personlar_map.get(&a.person);
                let b_person = personlar_map.get(&b.person);
                sort_person_by_name(a_person, b_person)
            }
        },
        JudgeSortBy::Categori => {
            let ret = a.categori.cmp(&b.categori);
            if !matches!(ret, Ordering::Equal) {
                ret
            } else {
                let a_person = personlar_map.get(&a.person);
                let b_person = personlar_map.get(&b.person);
                sort_person_by_name(a_person, b_person)
            }
        }
        JudgeSortBy::Participations => {
            let ret = b
                .number_of_participation_in_festivals
                .cmp(&a.number_of_participation_in_festivals);
            if !matches!(ret, Ordering::Equal) {
                ret
            } else {
                let a_person = personlar_map.get(&a.person);
                let b_person = personlar_map.get(&b.person);
                sort_person_by_name(a_person, b_person)
            }
        }
    }
}

// ==================================================

pub fn sort_person_by_age(
    a_person: Option<&Arc<Person>>,
    b_person: Option<&Arc<Person>>,
) -> Ordering {
    match (a_person, b_person) {
        (Some(a_person), Some(b_person)) => {
            let a_birth_date = get_birth_date_opt(a_person.birth_date);
            let b_birth_date = get_birth_date_opt(b_person.birth_date);
            match (a_birth_date, b_birth_date) {
                (Some(a_birth_date), Some(b_birth_date)) => b_birth_date.cmp(&a_birth_date),
                (None, Some(_)) => Ordering::Less,
                (Some(_), None) => Ordering::Greater,
                (None, None) => Ordering::Equal,
            }
        }
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

// ==================================================

pub fn get_birth_date_opt(birth_date: NaiveDate) -> Option<NaiveDate> {
    (birth_date != chrono::NaiveDate::from_ymd_opt(1900, 1, 1).unwrap()
        && birth_date != chrono::NaiveDate::from_ymd_opt(2021, 1, 1).unwrap())
    .then_some(birth_date)
}

// ==================================================

pub fn sort_person_by_name(
    a_person: Option<&Arc<Person>>,
    b_person: Option<&Arc<Person>>,
) -> Ordering {
    match (a_person, b_person) {
        (Some(a_person), Some(b_person)) => {
            let ret = {
                let last_namelar_map = &APP.data.last_namelar_map.lock_ref();
                let textlar_map = &APP.data.textlar_map.lock_ref();
                let a_last_name_text = last_namelar_map
                    .get(&a_person.last_name)
                    .and_then(|last_name| textlar_map.get(&last_name.value))
                    .map(|text| text.value.clone());
                let b_last_name_text = last_namelar_map
                    .get(&b_person.last_name)
                    .and_then(|last_name| textlar_map.get(&last_name.value))
                    .map(|text| text.value.clone());
                match (a_last_name_text, b_last_name_text) {
                    (Some(a_last_name_text), Some(b_last_name_text)) => {
                        a_last_name_text.cmp(&b_last_name_text)
                    }
                    (None, Some(_)) => Ordering::Less,
                    (Some(_), None) => Ordering::Greater,
                    (None, None) => Ordering::Equal,
                }
            };
            let ret = if !matches!(ret, Ordering::Equal) {
                ret
            } else {
                let first_namelar_map = &APP.data.first_namelar_map.lock_ref();
                let textlar_map = &APP.data.textlar_map.lock_ref();
                let a_first_name = first_namelar_map
                    .get(&a_person.first_name)
                    .and_then(|first_name| textlar_map.get(&first_name.value))
                    .map(|text| text.value.clone());
                let b_first_name = first_namelar_map
                    .get(&b_person.first_name)
                    .and_then(|first_name| textlar_map.get(&first_name.value))
                    .map(|text| text.value.clone());
                match (a_first_name, b_first_name) {
                    (Some(a_first_name), Some(b_first_name)) => a_first_name.cmp(&b_first_name),
                    (None, Some(_)) => Ordering::Less,
                    (Some(_), None) => Ordering::Greater,
                    (None, None) => Ordering::Equal,
                }
            };
            let ret = if !matches!(ret, Ordering::Equal) {
                ret
            } else {
                let second_namelar_map = &APP.data.second_namelar_map.lock_ref();
                let textlar_map = &APP.data.textlar_map.lock_ref();
                let a_second_name = second_namelar_map
                    .get(&a_person.second_name)
                    .and_then(|second_name| textlar_map.get(&second_name.value))
                    .map(|text| text.value.clone());
                let b_second_name = second_namelar_map
                    .get(&b_person.second_name)
                    .and_then(|second_name| textlar_map.get(&second_name.value))
                    .map(|text| text.value.clone());
                match (a_second_name, b_second_name) {
                    (Some(a_second_name), Some(b_second_name)) => a_second_name.cmp(&b_second_name),
                    (None, Some(_)) => Ordering::Less,
                    (Some(_), None) => Ordering::Greater,
                    (None, None) => Ordering::Equal,
                }
            };
            let ret = if !matches!(ret, Ordering::Equal) {
                ret
            } else {
                let nick_namelar_map = &APP.data.nick_namelar_map.lock_ref();
                let textlar_map = &APP.data.textlar_map.lock_ref();
                let a_nick_name = nick_namelar_map
                    .get(&a_person.nick_name)
                    .and_then(|nick_name| textlar_map.get(&nick_name.value))
                    .map(|text| text.value.clone());
                let b_nick_name = nick_namelar_map
                    .get(&b_person.nick_name)
                    .and_then(|nick_name| textlar_map.get(&nick_name.value))
                    .map(|text| text.value.clone());
                match (a_nick_name, b_nick_name) {
                    (Some(a_nick_name), Some(b_nick_name)) => a_nick_name.cmp(&b_nick_name),
                    (None, Some(_)) => Ordering::Less,
                    (Some(_), None) => Ordering::Greater,
                    (None, None) => Ordering::Equal,
                }
            };
            ret
        }
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

// ==================================================

pub fn sort_trainer_by_club(a_trainer: &Arc<Trainer>, b_trainer: &Arc<Trainer>) -> Ordering {
    let clublar_map = &APP.data.clublar_map.lock_ref();
    let textlar_map = &APP.data.textlar_map.lock_ref();
    let a = clublar_map
        .get(&a_trainer.club)
        .and_then(|club| textlar_map.get(&club.value))
        .map(|text| text.value.clone());
    let b = clublar_map
        .get(&b_trainer.club)
        .and_then(|club| textlar_map.get(&club.value))
        .map(|text| text.value.clone());
    match (a, b) {
        (Some(a), Some(b)) => a.cmp(&b),
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

// ==================================================

pub fn sort_club_by_name(a_club: &Arc<Club>, b_club: &Arc<Club>) -> Ordering {
    let textlar_map = &APP.data.textlar_map.lock_ref();
    let a = textlar_map
        .get(&a_club.value)
        .map(|text| text.value.clone());
    let b = textlar_map
        .get(&b_club.value)
        .map(|text| text.value.clone());
    match (a, b) {
        (Some(a), Some(b)) => a.cmp(&b),
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

pub fn sort_club_by_citi(a_club: &Arc<Club>, b_club: &Arc<Club>) -> Ordering {
    let citilar_map = &APP.data.citilar_map.lock_ref();
    let textlar_map = &APP.data.textlar_map.lock_ref();
    let a = citilar_map
        .get(&a_club.citi)
        .and_then(|citi| textlar_map.get(&citi.value))
        .map(|text| text.value.clone());
    let b = citilar_map
        .get(&b_club.citi)
        .and_then(|citi| textlar_map.get(&citi.value))
        .map(|text| text.value.clone());
    match (a, b) {
        (Some(a), Some(b)) => a.cmp(&b),
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

// ==================================================

pub fn dancer_sort_by_cmp(
    dancer_sort_by: &DancerSortBy,
    a: &Arc<Dancer>,
    b: &Arc<Dancer>,
    personlar_map: &std::collections::BTreeMap<i32, Arc<Person>>,
) -> Ordering {
    match dancer_sort_by {
        DancerSortBy::PointScore => {
            match (a.is_beginning(&None, false), b.is_beginning(&None, false)) {
                (true, false) => Ordering::Greater,
                (false, true) => Ordering::Less,
                (true, true) => APP
                    .data
                    .dancer_points(b, DancerScorePeriod::FromUpgrade)
                    .cmp(&APP.data.dancer_points(a, DancerScorePeriod::FromUpgrade)),
                (false, false) => APP
                    .data
                    .dancer_score(b, DancerScorePeriod::FromUpgrade)
                    .values()
                    .sum::<i16>()
                    .cmp(
                        &APP.data
                            .dancer_score(a, DancerScorePeriod::FromUpgrade)
                            .values()
                            .sum::<i16>(),
                    ),
            }
        }
        DancerSortBy::Name => {
            let a_person = personlar_map.get(&a.person);
            let b_person = personlar_map.get(&b.person);
            sort_person_by_name(a_person, b_person)
        }
        DancerSortBy::ExternalId => match (a.external_id, b.external_id) {
            (Some(a_external_id), Some(b_external_id)) => a_external_id.cmp(&b_external_id),
            (None, Some(_)) => Ordering::Greater,
            (Some(_), None) => Ordering::Less,
            (None, None) => {
                let a_person = personlar_map.get(&a.person);
                let b_person = personlar_map.get(&b.person);
                sort_person_by_name(a_person, b_person)
            }
        },
        DancerSortBy::Age => {
            let a_person = personlar_map.get(&a.person);
            let b_person = personlar_map.get(&b.person);
            let ret = sort_person_by_age(a_person, b_person);
            if !matches!(ret, Ordering::Equal) {
                ret
            } else {
                sort_person_by_name(a_person, b_person)
            }
        }
        DancerSortBy::Class => {
            let a_class = std::cmp::min(a.st_class(), a.la_class());
            let b_class = std::cmp::min(b.st_class(), b.la_class());
            let ret = a_class.cmp(&b_class);
            if !matches!(ret, Ordering::Equal) {
                ret
            } else {
                let a_person = personlar_map.get(&a.person);
                let b_person = personlar_map.get(&b.person);
                let ret = sort_person_by_age(a_person, b_person);
                if !matches!(ret, Ordering::Equal) {
                    ret
                } else {
                    sort_person_by_name(a_person, b_person)
                }
            }
        }
    }
}

// ==================================================

pub fn club_sort_by_cmp(club_sort_by: &ClubSortBy, a: &Arc<Club>, b: &Arc<Club>) -> Ordering {
    match club_sort_by {
        ClubSortBy::Name => sort_club_by_name(a, b),
        ClubSortBy::Citi => sort_club_by_citi(a, b),
    }
}

// ==================================================

pub fn route_from_url(url: &str) -> Route {
    let url = web_sys::Url::new(url).unwrap();
    if url.pathname().ends_with("/admin/") {
        todo!();
    } else {
        if url.hash().as_str().starts_with("#/") {
            route_from_url_hash(url.hash().as_str())
        } else {
            None
        }
        .unwrap_or_else(|| {
            Route::Guest(GuestRoute {
                kind: GuestRouteKind::Dancer {
                    sort_by: DancerSortBy::iter().next().unwrap(),
                    expanded: HashSet::new(),
                },
                did_press: false,
                search: None,
            })
        })
    }
}

pub fn route_from_url_hash(url_hash: &str) -> Option<Route> {
    URL_SAFE_BASE64
        .decode(&url_hash[2..])
        .ok()
        .and_then(|buf| rmp_serde::from_slice::<Route>(&buf).ok())
        .or_else(|| {
            // for backward compatibility
            BASE64
                .decode(&url_hash[2..])
                .ok()
                .and_then(|buf| rmp_serde::from_slice::<Route>(&buf).ok())
        })
}

use base64::engine::Engine;

// ==================================================
