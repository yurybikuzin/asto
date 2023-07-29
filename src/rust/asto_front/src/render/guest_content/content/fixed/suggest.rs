use super::*;

pub fn render_signal() -> impl Signal<Item = Option<Dom>> {
    APP.data
        .suggestlar_signal()
        .map(|suggestlar| suggestlar.map(render))
}

fn render(suggestlar: Vec<String>) -> Dom {
    html!("div", {
        .class("suggestlar")
        .children(suggestlar.into_iter().map(suggest))
    })
}

fn suggest(suggest: String) -> Dom {
    link!(route_to_url(&APP.data.suggest_route(Some(suggest.clone()))), {
        .text(&suggest)
    })
}

impl AppData {
    pub fn suggest_route(&self, suggest: Option<String>) -> Route {
        match (*self.route.lock_ref())
            .clone()
            .unwrap_or_else(route_default)
        {
            Route::Guest(GuestRoute { kind, .. }) => match kind {
                GuestRouteKind::Dancer { sort_by, .. } => Route::Guest(GuestRoute {
                    did_press: false,
                    search: suggest,
                    kind: GuestRouteKind::Dancer {
                        expanded: HashSet::new(),
                        sort_by,
                    },
                }),
                GuestRouteKind::Judge(sort_by) => Route::Guest(GuestRoute {
                    did_press: false,
                    search: suggest,
                    kind: GuestRouteKind::Judge(sort_by),
                }),
                GuestRouteKind::Trainer(sort_by) => Route::Guest(GuestRoute {
                    did_press: false,
                    search: suggest,
                    kind: GuestRouteKind::Trainer(sort_by),
                }),
                GuestRouteKind::Club(sort_by) => Route::Guest(GuestRoute {
                    did_press: false,
                    search: suggest,
                    kind: GuestRouteKind::Club(sort_by),
                }),
            },
            _ => unreachable!(),
        }
    }
    pub fn suggestlar_signal(&self) -> impl Signal<Item = Option<Vec<String>>> {
        map_ref! {
            let suggest_input = APP.data.suggest_input.signal_cloned()
            , let suggest_selected = APP.data.suggest_selected.signal()
            , let route = APP.data.route.signal_cloned()
            , let is_empty_dancerlar = App::filtered_dancerlar_signal().map(|vec| vec.is_empty())
            , let is_empty_judgelar = App::filtered_judgelar_signal().map(|vec| vec.is_empty())
            , let is_empty_trainerlar = App::filtered_trainerlar_signal().map(|vec| vec.is_empty())
            , let is_empty_clublar = App::filtered_clublar_signal().map(|vec| vec.is_empty())
        => {
            suggest_input.as_ref().and_then(|suggest_input|
                (
                    !suggest_selected
                     && match route.clone().unwrap_or_else(route_default) {
                        Route::Guest(GuestRoute{ kind, ..}) => {
                            match kind {
                                GuestRouteKind::Dancer { .. } => *is_empty_dancerlar,
                                GuestRouteKind::Judge { .. } => *is_empty_judgelar,
                                GuestRouteKind::Trainer { .. } => *is_empty_trainerlar,
                                GuestRouteKind::Club { .. } => *is_empty_clublar,
                            }
                        }
                        _ => unreachable!(),
                     }
                 ).then(|| {
                    match route.clone().unwrap_or_else(route_default) {
                        Route::Guest(GuestRoute{ kind, ..}) => {
                            match kind {
                                GuestRouteKind::Dancer { .. } => {
                                    let suggest_input = prepare_to_check_if_same_name(suggest_input);
                                    let personlar_map = APP.data.personlar_map.lock_ref();
                                    let last_namelar_map = APP.data.last_namelar_map.lock_ref();
                                    let first_namelar_map = APP.data.first_namelar_map.lock_ref();
                                    let textlar_map = APP.data.textlar_map.lock_ref();

                                    APP.data.dancerlar.lock_ref().iter().filter_map(|dancer|
                                        if let Some(person) = personlar_map.get(&dancer.person) {
                                            last_namelar_map.get(&person.last_name)
                                            .and_then(|last_name| textlar_map.get(&last_name.value))
                                            .and_then(|text| {
                                                let value = prepare_to_check_if_same_name(&text.value);
                                                value.starts_with(&suggest_input)
                                                    .then(||
                                                        format!("{} {}",
                                                            text.value.clone(),
                                                            first_namelar_map.get(&person.first_name)
                                                                .and_then(|first_name| textlar_map.get(&first_name.value))
                                                                .map(|text| text.value.clone())
                                                                .unwrap_or_default()
                                                        )
                                                    )
                                            })
                                        } else { None }
                                    ).collect::<HashSet<String>>()
                                },
                                GuestRouteKind::Judge { .. } => {
                                    let suggest_input = prepare_to_check_if_same_name(suggest_input);
                                    let personlar_map = APP.data.personlar_map.lock_ref();
                                    let last_namelar_map = APP.data.last_namelar_map.lock_ref();
                                    let first_namelar_map = APP.data.first_namelar_map.lock_ref();
                                    let textlar_map = APP.data.textlar_map.lock_ref();

                                     APP.data.judgelar.lock_ref().iter().filter_map(|judge|
                                        if let Some(person) = personlar_map.get(&judge.person) {
                                            last_namelar_map.get(&person.last_name)
                                            .and_then(|last_name| textlar_map.get(&last_name.value))
                                            .and_then(|text| {
                                                let value = prepare_to_check_if_same_name(&text.value);
                                                value.starts_with(&suggest_input)
                                                    .then(||
                                                        format!("{} {}",
                                                            text.value.clone(),
                                                            first_namelar_map.get(&person.first_name)
                                                                .and_then(|first_name| textlar_map.get(&first_name.value))
                                                                .map(|text| text.value.clone())
                                                                .unwrap_or_default()
                                                        )
                                                    )
                                            })
                                        } else { None }
                                    ).collect::<HashSet<String>>()
                                },
                                GuestRouteKind::Trainer { .. } => {
                                    let suggest_input = prepare_to_check_if_same_name(suggest_input);
                                    let personlar_map = APP.data.personlar_map.lock_ref();
                                    let last_namelar_map = APP.data.last_namelar_map.lock_ref();
                                    let first_namelar_map = APP.data.first_namelar_map.lock_ref();
                                    let textlar_map = APP.data.textlar_map.lock_ref();

                                    APP.data.trainerlar.lock_ref().iter().filter_map(|trainer|
                                        if let Some(person) = personlar_map.get(&trainer.person) {
                                            last_namelar_map.get(&person.last_name)
                                            .and_then(|last_name| textlar_map.get(&last_name.value))
                                            .and_then(|text| {
                                                let value = prepare_to_check_if_same_name(&text.value);
                                                value.starts_with(&suggest_input)
                                                    .then(||
                                                        format!("{} {}",
                                                            text.value.clone(),
                                                            first_namelar_map.get(&person.first_name)
                                                                .and_then(|first_name| textlar_map.get(&first_name.value))
                                                                .map(|text| text.value.clone())
                                                                .unwrap_or_default()
                                                        )
                                                    )
                                            })
                                        } else { None }
                                    ).collect::<HashSet<String>>()
                                },
                                GuestRouteKind::Club { .. } => {

                                    let suggest_input = prepare_to_check_if_same_name(suggest_input);
                                    let textlar_map = APP.data.textlar_map.lock_ref();

                                    APP.data.clublar.lock_ref().iter().filter_map(|club|
                                        if let Some(text) = textlar_map.get(&club.value) {
                                            text.value
                                                .split(' ')
                                                .filter(|s| !s.is_empty())
                                                .map(prepare_to_check_if_same_name)
                                                .any(|value|value.starts_with(&suggest_input))
                                                .then(|| text.value.clone())
                                        } else { None }
                                    ).collect::<HashSet<String>>()
                                }
                            }
                        }
                        _ => unreachable!(),
                    }
                }).and_then(|suggestlar|
                    (!suggestlar.is_empty()).then(|| {
                        let mut suggestlar = suggestlar.into_iter().collect::<Vec<_>>();
                        suggestlar.sort();
                        suggestlar
                    })
                )
            )
        }}
    }
}
