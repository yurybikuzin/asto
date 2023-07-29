use super::*;

mod sub_table;

pub fn render(rows: Vec<Arc<Dancer>>, route: Route) -> Dom {
    html!("table", {
        .class("filtered")
        .class("dancerlar")
        .class_signal("many",
            App::filtered_dancerlar_signal().map(|vec| vec.len() >= 3)
        )
        .children([
            html!("thead", {
                .child(html!("tr", {
                    .children(
                        &mut Column::iter().map(|column|
                            html!("th", {
                                .class(&column.to_string())
                            })
                        )
                    )
                }))
            }),
            html!("tbody", {
                .children({
                    let mut ret = vec![];
                    for (ord, dancer) in rows.into_iter().enumerate() {
                        ret.push(
                            html!("tr", {
                                .children(
                                    &mut Column::iter().map(|column|
                                        column.render(ord, dancer.clone())
                                    )
                                )
                            })
                        );
                        let mut trlar = vec![];
                        let expanded = dancer.external_id.and_then(|external_id| if let Route::Guest(GuestRoute { search: _, did_press: _, kind: GuestRouteKind::Dancer { sort_by: _, expanded}, ..  }) = route.clone() {
                            expanded.contains(&external_id).then_some(external_id)
                        } else {
                            None
                        });
                        if let Some(external_id) = expanded {
                            if let Some(event_resultlar) = APP.data.event_resultlar.lock_ref() .get(&external_id).cloned() {
                                let mut event_resultlar_map = HashMap::<i32, Vec<Arc<EventResult>>>::new();
                                for item in event_resultlar.iter().cloned() {
                                    common_macros2::entry!(event_resultlar_map, item.event
                                    =>
                                        and_modify |e| {
                                            e.push(item);
                                        }
                                        or_insert vec![item]
                                    );
                                }
                                let mut event_resultlar_vec = event_resultlar_map.into_iter().collect::<Vec<_>>();
                                event_resultlar_vec.sort_by_key(|(event, _item)|{
                                    APP.data.eventlar_map.lock_ref().get(event).unwrap().date
                                });
                                let mut before_class_upgrade_event_resultlar_vec = vec![];
                                if  let Some((class_updrade_date, _to_st_class, _to_la_class)) = dancer.class_upgrade() {
                                    let mut after_class_upgrade_event_resultlar_vec = vec![];
                                    for i in event_resultlar_vec.into_iter() {
                                        let date = APP.data.eventlar_map.lock_ref().get(&i.0).unwrap().date;
                                        if date >= class_updrade_date {
                                            after_class_upgrade_event_resultlar_vec.push(i);
                                        } else {
                                            before_class_upgrade_event_resultlar_vec.push(i);
                                        }
                                    }
                                    event_resultlar_vec = before_class_upgrade_event_resultlar_vec;
                                    after_class_upgrade_event_resultlar_vec.reverse();
                                    fill_trlar(&mut trlar, &after_class_upgrade_event_resultlar_vec, &dancer);
                                    fill_trlar_with_after_upgrade_state(&mut trlar, &dancer);
                                    fill_trlar_with_before_upgrade_state(&mut trlar, &dancer);
                                }
                                event_resultlar_vec.reverse();
                                fill_trlar(&mut trlar, &event_resultlar_vec, &dancer);
                                fill_trlar_with_base_state(&mut trlar, &dancer);
                            } else if dancer.class_upgrade().is_some() {
                                fill_trlar_with_after_upgrade_state(&mut trlar, &dancer);
                                fill_trlar_with_before_upgrade_state(&mut trlar, &dancer);
                            }
                        }
                        if !trlar.is_empty() {
                            ret.push(
                                html!("tr", {
                                    .class("score_points")
                                    .child(html!("td", {
                                        .attr("colspan", &Column::iter().count().to_string())
                                        .child(sub_table::render(dancer, trlar))
                                    }))
                                })
                            );
                        }
                    }
                    ret
                })
            })
        ])
    })
}

fn fill_trlar_with_after_upgrade_state(trlar: &mut Vec<Dom>, dancer: &Arc<Dancer>) {
    if let Some((class_updrade_date, to_st_class, to_la_class)) = dancer.class_upgrade() {
        let mut tdlar = vec![];
        tdlar.push(html!("td", {
            .class("event")
            .attr("colspan", "5")
            .text(&format!("На {} класс {}, сумма {}: 0",
               class_updrade_date.format("%Y-%m-%d"),
                if to_st_class == to_la_class {
                    APP.data.classlar_map.lock_ref().get(&to_st_class).as_ref().unwrap().value.clone()

                } else {
                    unreachable!();
                },
                if dancer.is_beginning(&None, false) {
                    "баллов"
                } else {
                    "очков"
                },
            ))
        }));
        trlar.push(html!("tr", {
            .class("event")
            .children(tdlar)
        }));
    }
}

fn fill_trlar_with_before_upgrade_state(trlar: &mut Vec<Dom>, dancer: &Arc<Dancer>) {
    if let Some((class_updrade_date, _, _)) = dancer.class_upgrade() {
        let class_updrade_date_before = class_updrade_date - chrono::Duration::days(1);
        let mut tdlar = vec![];
        if dancer.is_beginning(&Some(class_updrade_date_before), false) {
            let dancer_points = APP
                .data
                .dancer_points(dancer, DancerScorePeriod::TillUpgrade);
            tdlar.push(html!("td", {
                .class("event")
                .attr("colspan", "2")
                .text(&
                    format!("На {} класс {}, сумма баллов:",
                        class_updrade_date_before.format("%Y-%m-%d"),
                        if dancer.st_class == dancer.la_class {
                            APP.data.classlar_map.lock_ref().get(&dancer.st_class).as_ref().unwrap().value.clone()
                        } else {
                            unreachable!();
                        },
                    )
                )
            }));
            tdlar.push(html!("td", {
                .class("points")
                .attr("colspan", if dancer.is_beginning(&None, false) { "1" } else { "2" })
                .text(&(dancer_points as f64 / 10f64).to_string())
            }));
        } else {
            let dancer_score = APP
                .data
                .dancer_score(dancer, DancerScorePeriod::TillUpgrade);
            let dancer_score_sum = dancer_score.values().sum::<i16>();
            tdlar.push(html!("td", {
                .class("event")
                .attr("colspan", "2")
                .text(&
                    format!("На {} класс {}, сумма очков: {}",
                        (class_updrade_date - chrono::Duration::days(1)).format("%Y-%m-%d"),
                        if dancer.st_class == dancer.la_class {
                            APP.data.classlar_map.lock_ref().get(&dancer.st_class).as_ref().unwrap().value.clone()

                        } else {
                            unreachable!();
                        },
                        dancer_score_sum as f64 / 4f64
                    )
                )
            }));
            tdlar.push(html!("td", {
                .class("st_score")
                .text(&dancer_score.get(&DancerScoreKind::St).map(|v| (*v as f64 / 4f64).to_string()).unwrap_or_default())
            }));
            tdlar.push(html!("td", {
                .class("la_score")
                .text(&dancer_score.get(&DancerScoreKind::La).map(|v| (*v as f64 / 4f64).to_string()).unwrap_or_default())
            }));
        }
        trlar.push(html!("tr", {
            .class("event")
            .children(tdlar)
        }));
    }
}

fn fill_trlar_with_base_state(trlar: &mut Vec<Dom>, dancer: &Dancer) {
    let mut tdlar = vec![];
    let is_beginning = dancer.is_beginning(&Some(*BASE_DATE), false);
    tdlar.push(html!("td", {
        .class("event")
        .attr("colspan", "2")
        .text(&format!("На {} класс {}, сумма {}: {}", 
            BASE_DATE.format("%Y-%m-%d"),
            if dancer.st_class == dancer.la_class {
                APP.data.classlar_map.lock_ref().get(&dancer.st_class).as_ref().unwrap().value.clone()

            } else {
                unreachable!();
            },
            if is_beginning {
                "баллов"
            } else {
                "очков"
            },
            if is_beginning {
                "".to_owned()
            } else {
                let mut ret = dancer.st_la_score;
                if ret % 2 != 0 {
                    ret += 1;
                }
                ret += dancer.st_score;
                ret += dancer.la_score;
                (ret as f64 / 4f64).to_string()
            }
        ))
    }));
    if is_beginning {
        let points = dancer.points;
        tdlar.push(html!("td", {
            .class("points")
            .attr("colspan", if dancer.is_beginning(&None, false) { "1" } else { "2" })
            .text(&(points as f64 / 10f64).to_string())
        }));
    } else {
        let mut st_la_score = dancer.st_la_score;
        if st_la_score % 2 != 0 {
            st_la_score += 1;
        }
        let st_score = dancer.st_score + st_la_score / 2;
        let la_score = dancer.la_score + st_la_score / 2;
        tdlar.push(html!("td", {
            .class("st_score")
            .text(&(st_score > 0).then(|| (st_score as f64 / 4f64).to_string()).unwrap_or_default())
        }));
        tdlar.push(html!("td", {
            .class("la_score")
            .text(&(la_score > 0).then(|| (la_score as f64 / 4f64).to_string()).unwrap_or_default())
        }));
    }
    trlar.push(html!("tr", {
        .class("event")
        .children(tdlar)
    }));
}

fn fill_trlar(
    trlar: &mut Vec<Dom>,
    event_resultlar_vec: &[(i32, Vec<Arc<EventResult>>)],
    dancer: &Dancer,
) {
    for (event, event_resultlar) in event_resultlar_vec.iter() {
        let event_date = APP
            .data
            .eventlar_map
            .lock_ref()
            .get(event)
            .map(|event| event.date);
        let is_beginning = dancer.is_beginning(&event_date, false);
        {
            let mut tdlar = vec![];
            tdlar.push(html!("td", {
                .class("event")
                .attr("colspan", "2")
                .text(&{
                    let eventlar_map = &*APP.data.eventlar_map.lock_ref();
                    let event = eventlar_map.get(event).unwrap();
                    let title_value = APP.data.textlar_map.lock_ref().get(&event.title).unwrap().value.clone();
                    format!("{}, {title_value}", event.date)
                })
            }));

            if is_beginning {
                let points = itertools::fold(event_resultlar, 0, |a, b| a + b.points.unwrap_or(10));
                tdlar.push(html!("td", {
                    .class("points")
                    .attr("colspan", if dancer.is_beginning(&None, false) { "1" } else { "2" })
                    .text(&(points as f64 / 10f64).to_string())
                }));
            } else {
                let mut st_la_score =
                    itertools::fold(event_resultlar, 0, |a, b| a + b.st_la_score.unwrap_or(0));
                if st_la_score % 2 != 0 {
                    st_la_score += 1;
                }
                let st_score = st_la_score / 2
                    + itertools::fold(event_resultlar, 0, |a, b| a + b.st_score.unwrap_or(0));
                let la_score = st_la_score / 2
                    + itertools::fold(event_resultlar, 0, |a, b| a + b.la_score.unwrap_or(0));
                tdlar.push(html!("td", {
                    .class("st_score")
                    .text(&(st_score > 0).then(|| (st_score as f64 / 4f64).to_string()).unwrap_or_default())
                }));
                tdlar.push(html!("td", {
                    .class("la_score")
                    .text(&(la_score > 0).then(|| (la_score as f64 / 4f64).to_string()).unwrap_or_default())
                }));
            }
            let external_id = Some(event_resultlar.first().unwrap().external_id);
            let is_not_active_signal = APP
                .data
                .eventlar_map
                .signal_map_cloned()
                .key_cloned(*event)
                .map(move |event| {
                    let event_date = event.map(|event| event.date);
                    !(is_active(&external_id, &event_date)
                        || ((*APP.data.dancerlar_map.lock_ref())
                            .iter()
                            .find(|(_, dancer)| dancer.external_id == external_id)
                            .map(|(_, dancer)| dancer.is_beginning(&None, true)))
                        .unwrap_or(false))
                });
            trlar.push(html!("tr", {
                .class("event")
                .class_signal("is_not_active", is_not_active_signal)
                .children(tdlar)
            }));
        }
        for item in event_resultlar {
            trlar.push(if is_beginning {
                html!("tr", {
                    .children(
                        sub_table::ColumnPoints::iter().map(|column|
                            column.render(item.clone())
                        )
                    )
                })
            } else {
                html!("tr", {
                    .children(
                        sub_table::ColumnScore::iter().map(|column|
                            column.render(item.clone())
                        )
                    )
                })
            });
        }
    }
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
#[strum(serialize_all = "snake_case")]
#[repr(u8)]
enum Column {
    Ord,
    ExternalId,
    IsActive,
    Name,
    Age,
    BirthDate,
    Class,
    ScorePoints,
    Trainer,
    Citi,
    Club,
    Chief,
}
impl Column {
    fn render(&self, ord: usize, dancer: Arc<Dancer>) -> Dom {
        match self {
            Self::Ord | Self::ExternalId => html!("td", {
                .class(&self.to_string())
                .text(
                    &match self {
                        Self::Ord => self.ord_text(ord),
                        Self::ExternalId => self.external_id_text(dancer),
                        _ => unreachable!(),
                    }
                )
            }),
            Self::IsActive => {
                let status = if is_active(&dancer.external_id, &None) {
                    Some(true)
                } else if dancer.is_beginning(&None, true) {
                    None
                } else {
                    Some(false)
                };
                html!("td", {
                    .class(&self.to_string())
                    .class(
                        match status {
                            Some(true) => "true",
                            Some(false) => "false",
                            None => "none",
                        }
                    )
                    .text(
                        match status {
                            Some(true) => "Есть",
                            Some(false) => "Нет",
                            None => "—",
                        }
                    )
                })
            }
            Self::Citi => html!("td", {
                .class(&self.to_string())
                .text_signal(self.citi_text_signal(dancer))
            }),
            Self::Trainer => html!("td", {
                .class(&self.to_string())
                .child(html!("div", {
                    .text_signal(self.trainer_text_signal(dancer.clone()))
                }))
                .child_signal(
                    APP.data
                        .trainerlar_map
                        .signal_map_cloned()
                        .key_cloned(dancer.trainer2)
                        .map(move |trainer| {
                            App::person_name_signal(
                                trainer
                                    .as_ref()
                                    .map(|trainer| trainer.person)
                                    .unwrap_or_default(),
                            )
                        })
                        .flatten().map(|s|
                            if s.is_empty() {
                                None
                            } else {
                                Some(html!("div", {
                                    .text(&s)
                                }))
                            }
                        )
                )
            }),
            Self::BirthDate => html!("td", {
                .class(&self.to_string())
                .text_signal(self.birth_date_text_signal(dancer))
            }),
            Self::Name => html!("td", {
                .class(&self.to_string())
                .text_signal(
                    App::person_name_signal(dancer.person)
                )
            }),
            Self::Age => html!("td", {
                .class(&self.to_string())
                .text_signal(self.age_text_signal(dancer))
            }),
            Self::Club => html!("td", {
                .class(&self.to_string())
                .text_signal(self.club_text_signal(dancer))
            }),
            Self::Chief => html!("td", {
                .class(&self.to_string())
                .text_signal(self.club_chief_text_signal(dancer))
            }),
            Self::ScorePoints => {
                html!("td", {
                    .class(if dancer.is_beginning(&None, false) { "point" } else { "score"})
                    .child_signal({
                        let external_id = dancer.external_id.unwrap_or_default();
                        map_ref!{
                            let event_resultlar = APP.data.event_resultlar.signal_map_cloned().key_cloned(external_id)
                            , let route = APP.data.route.signal_cloned()
                        => move {
                            let text = APP.data.score_points_text(dancer.clone());
                            Some(if event_resultlar.is_none() && dancer.class_upgrade().is_none() {
                                html!("div", {
                                    .text(&text)
                                })
                            } else {
                                let Route::Guest(GuestRoute { search, did_press, kind: GuestRouteKind::Dancer { sort_by, mut expanded}, ..  }) = route.clone().unwrap_or_else(route_default) else {
                                     unreachable!();
                                };
                                let url = route_to_url(&Route::Guest(
                                    GuestRoute {
                                        search,
                                        did_press,
                                        kind: GuestRouteKind::Dancer {
                                            sort_by,
                                            expanded: {
                                                if expanded.contains(&external_id) {
                                                    expanded.remove(&external_id);
                                                } else {
                                                    expanded.insert(external_id);
                                                }
                                                expanded
                                            },
                                        }
                                    }
                                ));
                                link!(url, {
                                    .text(&text)
                                })
                            })
                        }}
                    })
                })
            }
            Self::Class => {
                if dancer.st_class() == dancer.la_class() {
                    html!("td", {
                        .class(&self.to_string())
                        .text_signal(self.same_class_text_signal(dancer))
                    })
                } else {
                    html!("td", {
                        .class(&self.to_string())
                        .text_signal(self.st_la_class_text_signal(dancer))
                    })
                }
            }
        }
    }
    fn citi_text_signal(&self, dancer: Arc<Dancer>) -> impl Signal<Item = String> {
        APP.data
            .clublar_map
            .signal_map_cloned()
            .key_cloned(dancer.club)
            .map(move |club| {
                APP.data
                    .citilar_map
                    .signal_map_cloned()
                    .key_cloned(club.map(|club| club.citi).unwrap_or_default())
                    .map(move |citi| {
                        APP.data
                            .textlar_map
                            .signal_map_cloned()
                            .key_cloned(citi.map(|citi| citi.value).unwrap_or_default())
                            .map(|text| text.map(|text| text.value.clone()))
                    })
                    .flatten()
            })
            .flatten()
            .map(|s_opt| s_opt.unwrap_or_default())
    }
    fn trainer_text_signal(&self, dancer: Arc<Dancer>) -> impl Signal<Item = String> {
        APP.data
            .trainerlar_map
            .signal_map_cloned()
            .key_cloned(dancer.trainer)
            .map(move |trainer| {
                App::person_name_signal(
                    trainer
                        .as_ref()
                        .map(|trainer| trainer.person)
                        .unwrap_or_default(),
                )
            })
            .flatten()
    }
    fn birth_date_text_signal(&self, dancer: Arc<Dancer>) -> impl Signal<Item = String> {
        APP.data
            .personlar_map
            .signal_map_cloned()
            .key_cloned(dancer.person)
            .map(move |person| {
                get_person_birth_date(person.as_ref()).map(|birth_date| birth_date.to_string())
            })
            .map(|s_opt| s_opt.unwrap_or_default())
    }
    fn ord_text(&self, ord: usize) -> String {
        (ord + 1).to_string()
    }
    fn external_id_text(&self, dancer: Arc<Dancer>) -> String {
        dancer
            .external_id
            .as_ref()
            .map(|i| i.to_string())
            .unwrap_or_default()
    }
    fn same_class_text_signal(&self, dancer: Arc<Dancer>) -> impl Signal<Item = String> {
        APP.data
            .classlar_map
            .signal_map_cloned()
            .key_cloned(dancer.st_class())
            .map(move |class| class.map(|class| class.value.clone()))
            .map(|s_opt| s_opt.unwrap_or_default())
    }
    fn st_la_class_text_signal(&self, dancer: Arc<Dancer>) -> impl Signal<Item = String> {
        map_ref! {
            let st_class_text = APP.data.classlar_map.signal_map_cloned()
                .key_cloned(dancer.st_class())
                .map(move |class|
                    class.map(|class|
                        format!("{} (St)", class.value)
                    )
                )
            , let la_class_text = APP.data.classlar_map.signal_map_cloned()
                .key_cloned(dancer.la_class())
                .map(move |class|
                    class.map(|class|
                        format!("{} (La)", class.value)
                    )
                )
        => {
            let mut ret = String::new();
            if let Some(s) = st_class_text {
                ret.push_str(s);
            }
            if let Some(s) = la_class_text {
                if !ret.is_empty() {
                    ret.push_str(" / ");
                }
                ret.push_str(s);
            }
            ret
        }}
    }
    fn club_text_signal(&self, dancer: Arc<Dancer>) -> impl Signal<Item = String> {
        APP.data
            .clublar_map
            .signal_map_cloned()
            .key_cloned(dancer.club)
            .map(move |club| {
                APP.data
                    .textlar_map
                    .signal_map_cloned()
                    .key_cloned(club.map(|club| club.value).unwrap_or_default())
                    .map(|text| text.map(|text| text.value.clone()))
            })
            .flatten()
            .map(|s_opt| s_opt.unwrap_or_default())
    }
    fn club_chief_text_signal(&self, dancer: Arc<Dancer>) -> impl Signal<Item = String> {
        APP.data
            .clublar_map
            .signal_map_cloned()
            .key_cloned(dancer.club)
            .map(move |club| {
                App::person_name_signal(club.and_then(|club| club.chief).unwrap_or_default())
            })
            .flatten()
        // .map(|s_opt| s_opt.unwrap_or_default())
    }
    fn age_text_signal(&self, dancer: Arc<Dancer>) -> impl Signal<Item = String> {
        map_ref! {
            let today = APP.data.today.signal_cloned()
            , let person = APP.data.personlar_map.signal_map_cloned()
                .key_cloned(dancer.person)
        => {
            get_person_birth_date(person.as_ref()).and_then(|birth_date| {
                today.map(|today| {
                    let age = get_age_of_today_by_birth_date(&today, &birth_date);

                    format!(
                        "{age} {}",
                        common_macros2::plural!(age,
                            1 "год"
                            2 "года"
                            5 "лет"
                        )
                    )
                })
            })
            .unwrap_or_default()
        }}
    }
}

impl AppData {
    pub fn score_points_text(&self, dancer: Arc<Dancer>) -> String {
        if dancer.is_beginning(&None, false) {
            self.point_text(dancer, DancerScorePeriod::FromUpgrade)
        } else {
            self.score_text(dancer, DancerScorePeriod::FromUpgrade)
        }
    }
    pub fn point_text(&self, dancer: Arc<Dancer>, period: DancerScorePeriod) -> String {
        (self.dancer_points(&dancer, period) as f64 / 10f64).to_string()
    }
    pub fn score_text(&self, dancer: Arc<Dancer>, period: DancerScorePeriod) -> String {
        let dance_score = self.dancer_score(&dancer, period);
        use std::fmt::Write;
        let mut ret = String::new();
        if let Some(v) = dance_score.get(&DancerScoreKind::St) {
            let _ = write!(ret, "{}{}(St)", *v as f64 / 4f64, 160u8 as char);
        }
        if let Some(v) = dance_score.get(&DancerScoreKind::La) {
            if !ret.is_empty() {
                ret.push_str(", ");
            }
            let _ = write!(ret, "{}{}(La)", *v as f64 / 4f64, 160u8 as char);
        }
        if ret.is_empty() {
            ret.push('0');
        }
        ret
    }
    pub fn dancer_points(&self, dancer: &Arc<Dancer>, period: DancerScorePeriod) -> i16 {
        let mut ret = match period {
            DancerScorePeriod::TillUpgrade => dancer.points,
            DancerScorePeriod::FromUpgrade => {
                if dancer.class_upgrade().is_some() {
                    0
                } else {
                    dancer.points
                }
            }
        };
        if let Some(event_resultlar) = dancer
            .external_id
            .and_then(|external_id| self.event_resultlar.lock_ref().get(&external_id).cloned())
        {
            let eventlar_map = &*APP.data.eventlar_map.lock_ref();
            let class_upgrade_date = dancer
                .class_upgrade()
                .map(|(class_upgrade_date, _, _)| class_upgrade_date);
            for item in event_resultlar.iter().filter(|i| {
                eventlar_map
                    .get(&i.event)
                    .map(|event| match period {
                        DancerScorePeriod::TillUpgrade => {
                            if let Some(class_upgrade_date) = class_upgrade_date {
                                event.date < class_upgrade_date
                            } else {
                                true
                            }
                        }
                        DancerScorePeriod::FromUpgrade => {
                            // let is_not_active =
                            let is_not_active =
                                !(is_active(&dancer.external_id, &Some(event.date))
                                    || dancer.is_beginning(&None, true));
                            if is_not_active {
                                // if is_not_active(&dancer.external_id, &Some(event.date)) {
                                false
                            } else if let Some(class_upgrade_date) = class_upgrade_date {
                                event.date >= class_upgrade_date
                            } else {
                                true
                            }
                        }
                    })
                    .unwrap_or(false)
            }) {
                ret += item.points.unwrap_or(10); // Танцор Н, участвуя в соревнованиях по скейтингу, вне зависимости от занятно места получает 1 балл
            }
        }
        ret
    }
    pub fn dancer_score(
        &self,
        dancer: &Arc<Dancer>,
        period: DancerScorePeriod,
    ) -> HashMap<DancerScoreKind, i16> {
        let mut ret = HashMap::new();
        let (mut st_score, mut la_score, mut st_la_score) = match period {
            DancerScorePeriod::TillUpgrade => {
                (dancer.st_score, dancer.la_score, dancer.st_la_score)
            }
            DancerScorePeriod::FromUpgrade => {
                if dancer.class_upgrade().is_some() {
                    (0, 0, 0)
                } else {
                    (dancer.st_score, dancer.la_score, dancer.st_la_score)
                }
            }
        };
        if let Some(event_resultlar) = dancer
            .external_id
            .and_then(|external_id| self.event_resultlar.lock_ref().get(&external_id).cloned())
        {
            let eventlar_map = &*APP.data.eventlar_map.lock_ref();
            let class_upgrade_date = dancer
                .class_upgrade()
                .map(|(class_upgrade_date, _, _)| class_upgrade_date);
            for item in event_resultlar.iter().filter(|i| {
                eventlar_map
                    .get(&i.event)
                    .map(|event| match period {
                        DancerScorePeriod::TillUpgrade => {
                            if let Some(class_upgrade_date) = class_upgrade_date {
                                event.date < class_upgrade_date
                            } else {
                                true
                            }
                        }
                        DancerScorePeriod::FromUpgrade => {
                            let is_not_active =
                                !(is_active(&dancer.external_id, &Some(event.date))
                                    || ((*APP.data.dancerlar_map.lock_ref())
                                        .iter()
                                        .find(|(_, i)| i.external_id == dancer.external_id)
                                        .map(|(_, i)| i.is_beginning(&None, true)))
                                    .unwrap_or(false));
                            if is_not_active {
                                // if is_not_active(&dancer.external_id, &Some(event.date)) {
                                false
                            } else if let Some(class_upgrade_date) = class_upgrade_date {
                                event.date >= class_upgrade_date
                            } else {
                                true
                            }
                        }
                    })
                    .unwrap_or(false)
            }) {
                if let Some(inc) = item.st_score {
                    st_score += inc;
                }
                if let Some(inc) = item.la_score {
                    la_score += inc;
                }
                if let Some(inc) = item.st_la_score {
                    st_la_score += inc;
                }
            }
        }
        if st_la_score > 0 {
            if st_la_score % 2 != 0 {
                st_la_score += 1;
            }
            st_score += st_la_score / 2;
            la_score += st_la_score / 2;
        }
        if st_score > 0 {
            ret.insert(DancerScoreKind::St, st_score);
        }
        if la_score > 0 {
            ret.insert(DancerScoreKind::La, la_score);
        }
        ret
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum DancerScoreKind {
    St,
    La,
}

pub enum DancerScorePeriod {
    TillUpgrade,
    FromUpgrade,
}
