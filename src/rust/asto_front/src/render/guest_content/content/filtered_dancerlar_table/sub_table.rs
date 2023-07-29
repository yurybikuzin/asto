use super::*;

pub fn render(dancer: Arc<Dancer>, trlar: Vec<Dom>) -> Dom {
    html!("table", {
        .class("score_points")
        .attr("cellspacing", "0")
        .children([
            html!("thead", {
                .child(
                    if dancer.is_beginning(&None, false) {
                        html!("tr", {
                            .children(
                                &mut ColumnPoints::iter().map(|column|
                                    html!("th", {
                                        .class(&column.to_string())
                                    })
                                )
                            )
                        })
                    } else {
                        html!("tr", {
                            .children(
                                &mut ColumnScore::iter().map(|column|
                                    html!("th", {
                                        .class(&column.to_string())
                                    })
                                )
                            )
                        })
                    }
                )
            }),
            html!("tbody", {
                .children(trlar)
            }),
        ])
    })
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
pub enum ColumnScore {
    Category,
    CoupleNumber,
    StScore,
    LaScore,
}

impl ColumnScore {
    pub fn render(&self, item: Arc<EventResult>) -> Dom {
        match self {
            Self::Category => html!("td", {
                .class(&self.to_string())
                .text_signal(
                    APP.data
                        .textlar_map
                        .signal_map_cloned()
                        .key_cloned(item.category)
                        .map(|text| text.map(|text| text.value.clone()))
                        .map(|s_opt| s_opt.unwrap_or_default())
                 )
            }),
            Self::CoupleNumber => html!("td", {
                .class(&self.to_string())
                .text(&item.couple_num.to_string())
            }),
            Self::StScore => html!("td", {
                .class(&self.to_string())
                .class_signal("is_not_active", {
                    let event_id = item.event;
                    let external_id = Some(item.external_id);
                    APP.data.eventlar_map
                        .signal_map_cloned()
                        .key_cloned(event_id)
                        .map(move |event| {
                            let event_date = event.map(|event| event.date);
                            !(is_active(&external_id, &event_date) || ((*APP.data.dancerlar_map.lock_ref()).iter().find(|(_, dancer)| dancer.external_id == external_id).map(|(_, dancer)| dancer.is_beginning(&None, true))).unwrap_or(false))
                        })
                    }
                )
                .text(&{
                    let v = [item.st_score, item.st_la_score.map(|v| if v % 2 == 0 { v / 2 } else { (v + 1) / 2 })].into_iter()
                          .flatten()/* https://rust-lang.github.io/rust-clippy/master/index.html#filter_map_identity */.sum::<i16>();
                    if v > 0 {
                        (v as f64 / 4f64).to_string()
                    } else {
                        "".to_owned()
                    }
                })
            }),
            Self::LaScore => html!("td", {
                .class(&self.to_string())
                .class_signal("is_not_active", {
                    let event_id = item.event;
                    let external_id = Some(item.external_id);
                    APP.data.eventlar_map
                        .signal_map_cloned()
                        .key_cloned(event_id)
                        .map(move |event| {
                            let event_date = event.map(|event| event.date);
                            !(is_active(&external_id, &event_date) || ((*APP.data.dancerlar_map.lock_ref()).iter().find(|(_, dancer)| dancer.external_id == external_id).map(|(_, dancer)| dancer.is_beginning(&None, true))).unwrap_or(false))
                        })
                    }
                )
                .text(&{
                    let v = [item.la_score, item.st_la_score.map(|v| if v % 2 == 0 { v / 2 } else { (v + 1) / 2 })].into_iter()
                          .flatten()/* https://rust-lang.github.io/rust-clippy/master/index.html#filter_map_identity */.sum::<i16>();
                    if v > 0 {
                        (v as f64 / 4f64).to_string()
                    } else {
                        "".to_owned()
                    }
                })
            }),
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
pub enum ColumnPoints {
    Category,
    CoupleNumber,
    // Place,
    Points,
}

impl ColumnPoints {
    pub fn render(&self, item: Arc<EventResult>) -> Dom {
        match self {
            Self::Category => html!("td", {
                .class(&self.to_string())
                .text_signal(
                    APP.data
                        .textlar_map
                        .signal_map_cloned()
                        .key_cloned(item.category)
                        .map(|text| text.map(|text| text.value.clone()))
                        .map(|s_opt| s_opt.unwrap_or_default())
                 )
            }),
            Self::CoupleNumber => html!("td", {
                .class(&self.to_string())
                .text(&item.couple_num.to_string())
            }),
            Self::Points => html!("td", {
                .class(&self.to_string())
                .class_signal("is_not_active", {
                    let event_id = item.event;
                    let external_id = Some(item.external_id);
                    APP.data.eventlar_map
                        .signal_map_cloned()
                        .key_cloned(event_id)
                        .map(move |event| {
                            let event_date = event.map(|event| event.date);
                            !(is_active(&external_id, &event_date) || ((*APP.data.dancerlar_map.lock_ref()).iter().find(|(_, dancer)| dancer.external_id == external_id).map(|(_, dancer)| dancer.is_beginning(&None, true))).unwrap_or(false))
                        })
                    }
                )
                .attr_signal("colspan", APP.data.dancerlar_map
                    .signal_map_cloned()
                    .key_cloned(item.external_id)
                    .map(|dancer| if dancer.map(|dancer| dancer.is_beginning(&None, false)).unwrap_or(false) { "1" } else { "2" })
                )
                .text(&item.points.map(|v|(v as f64 / 10f64)).unwrap_or(1f64).to_string())
            }),
        }
    }
}
