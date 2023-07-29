use super::*;

// mod thead;

pub fn render(rows: Vec<Arc<Judge>>) -> Dom {
    html!("table", {
        .class("filtered")
        .class("judgelar")
        .class_signal("many",
            App::filtered_judgelar_signal().map(|vec| vec.len() >= 3)
        )
        .children([
            // thead::render(),
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
                .children(
                    rows.into_iter().enumerate().map(|(ord, judge)|
                        html!("tr", {
                            .children(
                                &mut Column::iter().map(|column|
                                    column.render(ord, judge.clone())
                                )
                            )
                        })
                    )
                )
            })
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
enum Column {
    Ord,
    ExternalId,
    Name,
    Categori,
    AssignmentDate,
    Club,
    Citi,
    NumberOfParticipationInFestivals,
}
impl Column {
    fn render(&self, ord: usize, judge: Arc<Judge>) -> Dom {
        match self {
            Self::Ord | Self::ExternalId | Self::NumberOfParticipationInFestivals => html!("td", {
                .class(&self.to_string())
                .text(
                    &match self {
                        Self::Ord => self.ord_text(ord),
                        Self::ExternalId => self.external_id_text(judge),
                        Self::NumberOfParticipationInFestivals => self.number_of_participation_in_festivals_text(judge),
                        _ => unreachable!(),
                    }
                )
            }),
            Self::Citi => html!("td", {
                .class(&self.to_string())
                .text_signal(self.citi_text_signal(judge))
            }),
            Self::AssignmentDate => html!("td", {
                .class(&self.to_string())
                .text(&judge.assignment_date.to_string())
            }),
            Self::Name => html!("td", {
                .class(&self.to_string())
                .text_signal(
                    App::person_name_signal(judge.person)
                )
            }),
            Self::Club => html!("td", {
                .class(&self.to_string())
                .text_signal(self.club_text_signal(judge))
            }),
            Self::Categori => {
                html!("td", {
                    .class(&self.to_string())
                    .text_signal(self.categori_text_signal(judge))
                })
            }
        }
    }
    fn citi_text_signal(&self, judge: Arc<Judge>) -> impl Signal<Item = String> {
        APP.data
            .clublar_map
            .signal_map_cloned()
            .key_cloned(judge.club)
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
    fn ord_text(&self, ord: usize) -> String {
        (ord + 1).to_string()
    }
    fn external_id_text(&self, judge: Arc<Judge>) -> String {
        judge
            .external_id
            .as_ref()
            .map(|i| i.to_string())
            .unwrap_or_default()
    }

    fn number_of_participation_in_festivals_text(&self, judge: Arc<Judge>) -> String {
        (judge.number_of_participation_in_festivals > 0)
            .then(|| judge.number_of_participation_in_festivals.to_string())
            .unwrap_or_default()
    }
    fn club_text_signal(&self, judge: Arc<Judge>) -> impl Signal<Item = String> {
        APP.data
            .clublar_map
            .signal_map_cloned()
            .key_cloned(judge.club)
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
    fn categori_text_signal(&self, judge: Arc<Judge>) -> impl Signal<Item = String> {
        APP.data
            .categorilar_map
            .signal_map_cloned()
            .key_cloned(judge.categori)
            .map(move |categori| categori.map(|categori| categori.value.clone()))
            .map(|s_opt| s_opt.unwrap_or_default())
    }
}
