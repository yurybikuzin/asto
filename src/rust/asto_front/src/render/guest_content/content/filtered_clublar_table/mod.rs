use super::*;

// mod thead;

pub fn render(rows: Vec<Arc<Club>>) -> Dom {
    html!("table", {
        .class("filtered")
        .class("clublar")
        .class_signal("many",
            App::filtered_clublar_signal().map(|vec| vec.len() >= 3)
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
                    rows.into_iter().enumerate().map(|(ord, club)|
                        html!("tr", {
                            .children(
                                &mut Column::iter().map(|column|
                                    column.render(ord, club.clone())
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
    Club,
    Citi,
    Chief,
}
impl Column {
    fn render(&self, ord: usize, club: Arc<Club>) -> Dom {
        match self {
            Self::Ord => html!("td", {
                .class(&self.to_string())
                .text(&self.ord_text(ord))
            }),
            Self::Citi => html!("td", {
                .class(&self.to_string())
                .text_signal(self.citi_text_signal(club))
            }),
            Self::Club => html!("td", {
                .class(&self.to_string())
                .text_signal(self.club_text_signal(club))
            }),
            Self::Chief => html!("td", {
                .class(&self.to_string())
                .text_signal(App::person_name_signal(club.chief.unwrap_or_default()))
            }),
        }
    }
    fn citi_text_signal(&self, club: Arc<Club>) -> impl Signal<Item = String> {
        APP.data
            .citilar_map
            .signal_map_cloned()
            .key_cloned(club.citi)
            .map(move |citi| {
                APP.data
                    .textlar_map
                    .signal_map_cloned()
                    .key_cloned(citi.map(|citi| citi.value).unwrap_or_default())
                    .map(|text| text.map(|text| text.value.clone()))
            })
            .flatten()
            .map(|s_opt| s_opt.unwrap_or_default())
    }
    fn ord_text(&self, ord: usize) -> String {
        (ord + 1).to_string()
    }
    fn club_text_signal(&self, club: Arc<Club>) -> impl Signal<Item = String> {
        APP.data
            .textlar_map
            .signal_map_cloned()
            .key_cloned(club.value)
            .map(|text| text.map(|text| text.value.clone()).unwrap_or_default())
    }
}
