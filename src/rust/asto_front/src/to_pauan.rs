use dominator::clone;
use futures_signals::{
    signal::{Signal, SignalExt},
    signal_map::{MutableBTreeMap, SignalMapExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use once_cell::sync::Lazy;
use std::sync::Arc;

pub static APP: Lazy<Arc<App>> = Lazy::new(App::new);

pub struct App {
    pub data: AppData,
}

#[derive(Default)]
pub struct AppData {
    pub cities_selected: MutableBTreeMap<u16, bool>,
    pub clubs: MutableVec<Arc<Club>>,
    pub clubs_map: MutableBTreeMap<u16, Arc<Club>>,
    pub judges: MutableVec<Arc<Judge>>,
}

pub struct City {
    pub id: u16,
    pub name: String,
}

pub struct Club {
    pub id: u16,
    pub city: u16,
}

pub struct Judge {
    pub club: u16,
}

pub struct Dancer {
    pub club: u16,
}

impl App {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            data: AppData::default(),
        })
    }
    pub fn is_selected_city_of_club_of_the_dancer(dancer: Arc<Dancer>) -> impl Signal<Item = bool> {
        APP.data
            .clubs_map
            .signal_map_cloned()
            .key_cloned(dancer.club)
            .map(|club| {
                APP.data
                    .cities_selected
                    .signal_map_cloned()
                    .key_cloned(club.unwrap().city)
            })
            .flatten()
            .map(|is_selected| is_selected.unwrap_or(false))
    }
    pub fn sum_of_clubs_of_the_city(city: Arc<City>) -> impl Signal<Item = usize> {
        APP.data
            .clubs
            .signal_vec_cloned()
            .filter_map(clone!(city =>
               move |club|
                    if club.city == city.id {
                        Some(1)
                    } else {
                        None
                    }
            ))
            .sum()
    }
    pub fn sum_of_judges_from_clubs_of_the_city(city: Arc<City>) -> impl Signal<Item = usize> {
        // ## Question:
        // APP.data
        //     .clubs
        //     .signal_vec_cloned()
        //     .filter_map(clone!(city => move |club|
        //         if club.city == city.id {
        //             Some(
        //                 APP.data.judges.signal_vec_cloned()
        //                 .filter_map(clone!(club => move |judge|
        //                     (judge.club == club.id).then_some(1))
        //                 ).sum()
        //             )
        //         } else {
        //             None
        //         }
        //     ))
        //     .flatten()
        //     .sum()
        // ## Pauan solution:
        APP.data
            .clubs
            .signal_vec_cloned()
            .filter(clone!(city => move |club| club.city == city.id))
            .map_signal(move |club| {
                APP.data
                    .judges
                    .signal_vec_cloned()
                    .filter_map(clone!(club => move |judge|
                        (judge.club == club.id).then_some(1)))
                    .sum()
            })
            .sum()
    }
}
