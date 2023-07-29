use super::*;

// ===================================================================

lazy_static::lazy_static! {
    pub static ref HORIZONTAL_ALIGNMENT_RIGHT: Option<String> = Some("RIGHT".to_owned());
    pub static ref HORIZONTAL_ALIGNMENT_LEFT: Option<String>  = Some("LEFT".to_owned());
    pub static ref HORIZONTAL_ALIGNMENT_CENTER: Option<String>  = Some("CENTER".to_owned());
    pub static ref COLOR_GRAY: Option<Color> = Some(color_from_code("CCC"));
    pub static ref TEXT_FORMAT_BOLD: Option<TextFormat> = Some(TextFormat {
        bold: Some(true),
        ..TextFormat::default()
    });
    pub static ref CELL_FORMAT_LABEL: Option<CellFormat>  = Some(CellFormat {
        background_color: COLOR_GRAY.clone(),
        ..CellFormat::default()
    });
    pub static ref CELL_FORMAT_H1: Option<CellFormat>  = Some(CellFormat {
        text_format: Some(TextFormat {
            bold: Some(true),
            font_size: Some(24),
            ..TextFormat::default()
        }),
        ..CellFormat::default()
    });
    pub static ref CELL_FORMAT_H2: Option<CellFormat>  = Some(CellFormat {
        text_format: Some(TextFormat {
            bold: Some(true),
            font_size: Some(20),
            ..TextFormat::default()
        }),
        ..CellFormat::default()
    });
    pub static ref CELL_FORMAT_H3: Option<CellFormat>  = Some(CellFormat {
        text_format: Some(TextFormat {
            bold: Some(true),
            font_size: Some(16),
            ..TextFormat::default()
        }),
        ..CellFormat::default()
    });
    pub static ref CELL_FORMAT_TITLE: Option<CellFormat>  = Some(CellFormat {
        background_color: COLOR_GRAY.clone(),
        text_format: TEXT_FORMAT_BOLD.clone(),
        ..CellFormat::default()
    });
    pub static ref CELL_FORMAT_JUDGE_CODE: Option<CellFormat> = Some(CellFormat {
        horizontal_alignment: HORIZONTAL_ALIGNMENT_RIGHT.clone(),
        background_color: Some(color_from_code("FCF3CF")),
        ..CellFormat::default()
    });
    pub static ref CELL_FORMAT_CAPTION_RIGHT: Option<CellFormat> = Some(CellFormat {
        horizontal_alignment: HORIZONTAL_ALIGNMENT_RIGHT.clone(),
        ..CellFormat::default()
    });
    pub static ref CELL_FORMAT_CAPTION_CENTER: Option<CellFormat> = Some(CellFormat {
        horizontal_alignment: HORIZONTAL_ALIGNMENT_CENTER.clone(),
        ..CellFormat::default()
    });
    pub static ref CELL_FORMAT_CAPTION_LEFT: Option<CellFormat> = Some(CellFormat {
        horizontal_alignment: HORIZONTAL_ALIGNMENT_LEFT.clone(),
        ..CellFormat::default()
    });
    pub static ref CELL_FORMAT_CAPTION_SUMMARY_LEFT: Option<CellFormat> = Some(CellFormat {
        horizontal_alignment: HORIZONTAL_ALIGNMENT_LEFT.clone(),
        background_color: COLOR_GRAY.clone(),
        ..CellFormat::default()
    });
    pub static ref CELL_FORMAT_CAPTION_SUMMARY_RIGHT: Option<CellFormat> = Some(CellFormat {
        horizontal_alignment: HORIZONTAL_ALIGNMENT_RIGHT.clone(),
        background_color: COLOR_GRAY.clone(),
        ..CellFormat::default()
    });
    pub static ref CELL_FORMAT_CAPTION_SUMMARY_CENTER: Option<CellFormat> = Some(CellFormat {
        horizontal_alignment: HORIZONTAL_ALIGNMENT_CENTER.clone(),
        background_color: COLOR_GRAY.clone(),
        ..CellFormat::default()
    });

    pub static ref CELL_FORMAT_CAPTION_SUMMARY_TOTAL: Option<CellFormat> = Some(CellFormat {
        horizontal_alignment: HORIZONTAL_ALIGNMENT_CENTER.clone(),
        background_color: COLOR_GRAY.clone(),
        text_format: Some(TextFormat {
            bold: Some(true),
            ..TextFormat::default()
        }),
        ..CellFormat::default()
    });
    pub static ref CELL_FORMAT_TOTAL: Option<CellFormat> = Some(CellFormat {
        horizontal_alignment: HORIZONTAL_ALIGNMENT_CENTER.clone(),
        text_format: Some(TextFormat {
            bold: Some(true),
            ..TextFormat::default()
        }),
        ..CellFormat::default()
    });
}

#[derive(Debug, Clone, Copy)]
pub enum PointsKind {
    Ball,
    ScoreSt,
    ScoreStLa,
    ScoreLa,
}

#[derive(Debug, Clone, Copy, strum::Display)]
pub enum CompetKind {
    Кубок,
    Аттестация,
    Категория,
}

use std::hash::{Hash, Hasher};
#[derive(Clone, Debug, Eq)]
pub struct SummaryDancer {
    pub couple_number: i16,
    pub last_name: String,
    pub first_name: String,
    pub class: Option<String>,
    pub city: String,
    pub club: String,
}
impl Hash for SummaryDancer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.couple_number.hash(state);
        prepare_to_check_if_same_name(&self.last_name).hash(state);
        prepare_to_check_if_same_name(&self.first_name).hash(state);
    }
}
impl PartialEq for SummaryDancer {
    fn eq(&self, other: &Self) -> bool {
        self.couple_number == other.couple_number
            && prepare_to_check_if_same_name(&self.last_name)
                == prepare_to_check_if_same_name(&other.last_name)
            && prepare_to_check_if_same_name(&self.first_name)
                == prepare_to_check_if_same_name(&other.first_name)
    }
}
pub type SummaryDanceCategory = String;
pub type SummaryDancePoints = HashMap<SummaryDanceCategory, f64>;
pub type SummaryDancerlarPoints = HashMap<SummaryDancer, SummaryDancePoints>;

fn prepare_to_check_if_same_name(s: &str) -> String {
    s.to_lowercase().replace('ё', "е")
}

// ===================================================================

pub fn get_gradient_points(
    total_results: &[(CoupleNumber, Sax3RoundTotalRes)],
    dancer_class: &Option<String>,
    couples: &HashMap<i16, pasitos::sax3::Sax3Couple>,
    total_results_i: usize,
    current_round_number: i16,
    rounds: &[(Sax3RoundNumber, Sax3Round)],
) -> f64 {
    let prev_rounds = rounds
        .iter()
        .filter(|(round_number, _)| *round_number < current_round_number);
    let count = total_results
        .iter()
        .enumerate()
        .filter(|(j, _)| j > &total_results_i)
        .map(|(_, (couple_number, _))| *couple_number)
        .chain(
            prev_rounds
                .into_iter()
                .flat_map(|(_, Sax3Round { total_results, .. })| total_results.keys().cloned())
                .collect::<HashSet<_>>()
                .difference(
                    &total_results
                        .iter()
                        .map(|(couple_number, _)| *couple_number)
                        .collect::<HashSet<_>>(),
                )
                .cloned(),
        )
        .filter_map(|couple_number| {
            couples.get(&couple_number).and_then(
                |Sax3Couple {
                     class: couple_class_tst,
                     ..
                 }| {
                    couple_class_tst.as_ref().and_then(|couple_class_tst| {
                        dancer_class.as_ref().and_then(|dancer_class| {
                            (dancer_class > couple_class_tst).then_some(())
                        })
                    })
                },
            )
        })
        .count();
    (1 + count) as f64
        * if dancer_class
            .as_ref()
            .map(|dancer_class| dancer_class.starts_with('H'))
            .unwrap_or(false)
        {
            1f64
        } else {
            0.25f64
        }
}

// ===================================================================

#[derive(Debug, Clone)]
pub enum DanceCategory {
    Name(String),
    Gradient(String, String),
}

common_macros2::impl_display!(
    DanceCategory,
    self,
    f,
    match self {
        Self::Name(s) => write!(f, "{s}"),
        Self::Gradient(gradient, phase) => write!(f, "Градиент::{gradient}::{phase}"),
    }
);

#[derive(Debug, Clone, Copy)]
pub enum RoundKind {
    Gradient,
    BeginnersCup,
    Skating,
}

#[allow(clippy::too_many_arguments)]
pub fn process_rounds(
    ret_key: &pasitos::sax3::Sax3RetKey,
    compet_judges: Option<Sax3Judges>,
    rounds: HashMap<i16, pasitos::sax3::Sax3Round>,
    couples: HashMap<i16, pasitos::sax3::Sax3Couple>,
    kind: CompetKind,
    round_kind: RoundKind,
    dance_category: DanceCategory,
    ident: usize,
    sheet_rows: &mut Vec<Vec<spreadsheets::CellData>>,
    dancerlar_points: &mut SummaryDancerlarPoints,
    db_rows: &mut Vec<ImportEventRow>,
) {
    let mut rounds = rounds.into_iter().collect::<Vec<_>>();
    rounds.sort_by_key(|i| i.0);
    let mut rounds_peekable = rounds.iter().cloned().peekable();
    while let Some((
        round_number,
        Sax3Round {
            name,
            board_point: _,
            mode,
            judges,
            total_results,
            result_details,
        },
    )) = rounds_peekable.next()
    {
        if result_details.is_empty() {
            panic!("round_number: {round_number}; dance_category: {dance_category}, total_results: {total_results:#?}");
        }
        let points_kind = match mode {
            Sax3RoundMode::Ball => PointsKind::Ball,
            Sax3RoundMode::Skating | Sax3RoundMode::Sum => {
                if matches!(round_kind, RoundKind::BeginnersCup) {
                    PointsKind::Ball
                } else {
                    let mut ret = None;
                    for (Sax3DanceKey { name, .. }, _) in result_details.iter() {
                        ret = match (ret, name.as_str()) {
                            (None | Some(PointsKind::ScoreSt), "W" | "T" | "V" | "F" | "Q") => {
                                Some(PointsKind::ScoreSt)
                            }
                            (None | Some(PointsKind::ScoreLa), "S" | "Ch" | "R" | "P" | "J") => {
                                Some(PointsKind::ScoreLa)
                            }
                            (
                                Some(_),
                                "W" | "T" | "V" | "F" | "Q" | "S" | "Ch" | "R" | "P" | "J",
                            ) => Some(PointsKind::ScoreStLa),
                            (_, "Polka") => {
                                Some(PointsKind::Ball)
                            }
                            _ => unreachable!("{dance_category:?}: Dance name={name:?}, ret: {ret:?}, mode: {mode:?}, round_kind: {round_kind:?}"),
                        }
                    }
                    ret.unwrap()
                }
            }
        };

        {
            sheet_rows.push({
                let mut ret = (0..ident)
                    .map(|_| cell_data(None, None))
                    .chain(
                        vec![
                            cell_data(
                                Some(CompactExtendedValue::String("Тур:".to_owned())),
                                CELL_FORMAT_LABEL.clone(),
                            ),
                            cell_data(
                                Some(CompactExtendedValue::String(name.clone())),
                                CELL_FORMAT_TITLE.clone(),
                            ),
                        ]
                        .into_iter(),
                    )
                    .collect::<Vec<_>>();
                match points_kind {
                    PointsKind::Ball => {
                        ret.push(cell_data(
                            Some(CompactExtendedValue::String("баллы".to_owned())),
                            CELL_FORMAT_CAPTION_RIGHT.clone(),
                        ));
                    }
                    PointsKind::ScoreStLa => {
                        ret.push(cell_data(
                            Some(CompactExtendedValue::String("очки St+La".to_owned())),
                            CELL_FORMAT_CAPTION_RIGHT.clone(),
                        ));
                    }
                    PointsKind::ScoreSt => {
                        ret.push(cell_data(
                            Some(CompactExtendedValue::String("очки St".to_owned())),
                            CELL_FORMAT_CAPTION_RIGHT.clone(),
                        ));
                    }
                    PointsKind::ScoreLa => {
                        ret.push(cell_data(
                            Some(CompactExtendedValue::String("очки La".to_owned())),
                            CELL_FORMAT_CAPTION_RIGHT.clone(),
                        ));
                    }
                }
                if !matches!(kind, CompetKind::Аттестация) {
                    ret.push(cell_data(
                        Some(CompactExtendedValue::String("место".to_owned())),
                        CELL_FORMAT_CAPTION_RIGHT.clone(),
                    ));
                }
                ret.extend(vec![
                    cell_data(
                        Some(CompactExtendedValue::String("номер пары".to_owned())),
                        CELL_FORMAT_CAPTION_RIGHT.clone(),
                    ),
                    cell_data(
                        Some(CompactExtendedValue::String("фамилия".to_owned())),
                        CELL_FORMAT_CAPTION_LEFT.clone(),
                    ),
                    cell_data(
                        Some(CompactExtendedValue::String("имя".to_owned())),
                        CELL_FORMAT_CAPTION_LEFT.clone(),
                    ),
                    cell_data(
                        Some(CompactExtendedValue::String("класс".to_owned())),
                        CELL_FORMAT_CAPTION_CENTER.clone(),
                    ),
                    cell_data(
                        Some(CompactExtendedValue::String("город".to_owned())),
                        CELL_FORMAT_CAPTION_LEFT.clone(),
                    ),
                    cell_data(
                        Some(CompactExtendedValue::String("клуб".to_owned())),
                        CELL_FORMAT_CAPTION_LEFT.clone(),
                    ),
                ]);
                ret
            });
            let mut total_results = total_results.into_iter().collect::<Vec<_>>();
            total_results.sort_by_key(|i| {
                (i.1.place.or_else(|| i.1.sum.map(|v| -v)).unwrap_or(100f64) * 100f64) as i64
            });

            #[derive(Debug)]
            struct Dancer {
                points: Option<f64>,
                place: Option<f64>,
                couple_number: i16,
                first_name: String,
                last_name: String,
                book_number: Option<i32>,
                birth_day: Option<chrono::NaiveDate>,
                class: Option<String>,
                city: String,
                club: String,
            }

            let mut rows: Vec<Dancer> = vec![];
            for (i, (couple_number, Sax3RoundTotalRes { place, sum: _ })) in
                total_results.iter().cloned().enumerate()
            {
                let Some(Sax3Couple {
                    male,
                    female,
                    club,
                    ..
                }) = couples.get(&couple_number).cloned() else {
                    unreachable!("couple_number: {couple_number}, couples: {couples:#?}");
                };

                let Sax3Dancer {
                    points,
                    first_name,
                    last_name,
                    book_number,
                    birth_day,
                    class: dancer_class,
                    ..
                } = male;
                {
                    let Sax3Club {
                        city, name: club, ..
                    } = club.clone();
                    let points = match round_kind {
                        RoundKind::BeginnersCup => None,
                        RoundKind::Skating => points,
                        RoundKind::Gradient => Some(get_gradient_points(
                            &total_results,
                            &dancer_class,
                            &couples,
                            i,
                            round_number,
                            &rounds,
                        )),
                    };
                    let dancer = Dancer {
                        couple_number,
                        first_name,
                        last_name,
                        book_number,
                        birth_day,
                        class: dancer_class,
                        points,
                        place,
                        city,
                        club,
                    };
                    rows.push(dancer);
                }
                if let Some(Sax3Dancer {
                    first_name,
                    last_name,
                    book_number,
                    birth_day,
                    class: dancer_class,
                    points,
                    ..
                }) = female
                {
                    let Sax3Club {
                        city, name: club, ..
                    } = club;
                    let points = match round_kind {
                        RoundKind::BeginnersCup => None,
                        RoundKind::Skating => points,
                        RoundKind::Gradient => Some(get_gradient_points(
                            &total_results,
                            &dancer_class,
                            &couples,
                            i,
                            round_number,
                            &rounds,
                        )),
                    };
                    let dancer = Dancer {
                        couple_number,
                        place,
                        points,
                        first_name,
                        last_name,
                        book_number,
                        birth_day,
                        class: dancer_class,
                        city,
                        club,
                    };
                    rows.push(dancer);
                }
            }
            sheet_rows.extend(rows.into_iter().map(
                |Dancer {
                     couple_number,
                     place,
                     points,
                     first_name,
                     last_name,
                     book_number,
                     birth_day,
                     class,
                     city,
                     club,
                 }| {
                    let mut ret = (0..(ident + 2))
                        .map(|_| cell_data(None, None))
                        .collect::<Vec<_>>();

                    let need_show_points = if let Some((_, round)) = rounds_peekable.peek() {
                        let next_round_couples =
                            round.total_results.keys().copied().collect::<HashSet<_>>();
                        !next_round_couples.contains(&couple_number)
                    } else {
                        true
                    };
                    let points = match need_show_points.then_some(points_kind) {
                        None => None,
                        Some(PointsKind::Ball) => Some(points.unwrap_or(1f64)),
                        Some(PointsKind::ScoreStLa | PointsKind::ScoreSt | PointsKind::ScoreLa) => {
                            Some(points.unwrap_or(0f64))
                        }
                    };
                    if let Some(points) = points {
                        let summary_dancer = SummaryDancer {
                            couple_number,
                            last_name: last_name.clone(),
                            first_name: first_name.clone(),
                            class: class.clone(),
                            city: city.clone(),
                            club: club.clone(),
                        };
                        let category = if matches!(dance_category, DanceCategory::Name(_)) {
                            dance_category.to_string()
                        } else if result_details.len() == 1 {
                            result_details.keys().next().unwrap().name.clone()
                        } else {
                            use itertools::Itertools;
                            unreachable!(
                                "summary_dancer: {summary_dancer:#?}, result_details.keys: {:#?}",
                                result_details.keys().map(|key| key.name.clone()).join("+")
                            );
                        };

                        db_rows.push(ImportEventRow {
                            date: ret_key.date,
                            title: ret_key.title.clone(),
                            category: if matches!(dance_category, DanceCategory::Gradient { .. }) {
                                format!("{dance_category}::{category}")
                            } else {
                                category.clone()
                            },
                            couple_number,
                            st_score: matches!(points_kind, PointsKind::ScoreSt)
                                .then(|| (points * 4f64).round() as i16),
                            la_score: matches!(points_kind, PointsKind::ScoreLa)
                                .then(|| (points * 4f64).round() as i16),
                            st_la_score: matches!(points_kind, PointsKind::ScoreStLa)
                                .then(|| (points * 4f64).round() as i16),
                            points: matches!(points_kind, PointsKind::Ball)
                                .then(|| (points * 10f64).round() as i16),
                            external_id: book_number,
                            first_name: first_name.clone(),
                            last_name: last_name.clone(),
                            dancer_class: class.clone(),
                            birthdate: birth_day,
                            club: club.clone(),
                            city: city.clone(),
                        });
                        common_macros2::entry!(dancerlar_points, summary_dancer
                        =>
                            and_modify |e| {
                                common_macros2::entry!(e, category
                                =>
                                    and_modify |e| {
                                        *e += points;
                                    }
                                    or_insert points
                                );

                            }
                            or_insert vec![(category, points)].into_iter().collect()
                        );
                    }
                    ret.push(cell_data(points.map(CompactExtendedValue::Float), None));
                    if !matches!(kind, CompetKind::Аттестация) {
                        ret.push(cell_data(
                            place.map(|v| CompactExtendedValue::Int(v as i64)),
                            None,
                        ));
                    }
                    ret.extend(vec![
                        cell_data(Some(CompactExtendedValue::Int(couple_number as i64)), None),
                        cell_data(Some(CompactExtendedValue::String(last_name)), None),
                        cell_data(Some(CompactExtendedValue::String(first_name)), None),
                        cell_data(
                            class
                                .as_ref()
                                .map(|v| CompactExtendedValue::String(v.to_owned())),
                            CELL_FORMAT_CAPTION_CENTER.clone(),
                        ),
                        cell_data(Some(CompactExtendedValue::String(city)), None),
                        cell_data(Some(CompactExtendedValue::String(club)), None),
                    ]);
                    ret
                },
            ));
            sheet_rows.push(vec![cell_data(None, None)]);
        }
        {
            sheet_rows.push(
                (0..(ident + 1))
                    .map(|_| cell_data(None, None))
                    .chain(
                        vec![
                            cell_data(
                                Some(CompactExtendedValue::String("Судьи:".to_owned())),
                                CELL_FORMAT_LABEL.clone(),
                            ),
                            cell_data(
                                Some(CompactExtendedValue::String("фамилия".to_owned())),
                                CELL_FORMAT_CAPTION_LEFT.clone(),
                            ),
                            cell_data(
                                Some(CompactExtendedValue::String("имя".to_owned())),
                                CELL_FORMAT_CAPTION_LEFT.clone(),
                            ),
                            cell_data(
                                Some(CompactExtendedValue::String("клуб".to_owned())),
                                CELL_FORMAT_CAPTION_LEFT.clone(),
                            ),
                            cell_data(
                                Some(CompactExtendedValue::String("город".to_owned())),
                                CELL_FORMAT_CAPTION_LEFT.clone(),
                            ),
                        ]
                        .into_iter(),
                    )
                    .collect::<Vec<_>>(),
            );
            sheet_rows.extend(
                judges
                    .as_ref()
                    .or(compet_judges.as_ref())
                    .unwrap()
                    .all()
                    .into_iter()
                    .map(
                        |(
                            judge_id,
                            Sax3Judge {
                                first_name,
                                last_name,
                                club,
                                city,
                                // book_number,
                                ..
                            },
                        )| {
                            (0..(ident + 1))
                                .map(|_| cell_data(None, None))
                                .chain(
                                    vec![
                                        cell_data(
                                            Some(CompactExtendedValue::String(
                                                judge_id.to_string(),
                                            )),
                                            match judge_id {
                                                Sax3JudgeId::Private { .. } => {
                                                    CELL_FORMAT_JUDGE_CODE.clone()
                                                }
                                                _ => CELL_FORMAT_CAPTION_RIGHT.clone(),
                                            },
                                        ),
                                        cell_data(
                                            Some(CompactExtendedValue::String(last_name)),
                                            None,
                                        ),
                                        cell_data(
                                            Some(CompactExtendedValue::String(first_name)),
                                            None,
                                        ),
                                        cell_data(
                                            club.as_ref()
                                                .map(|v| CompactExtendedValue::String(v.clone())),
                                            None,
                                        ),
                                        cell_data(
                                            city.as_ref()
                                                .map(|v| CompactExtendedValue::String(v.clone())),
                                            None,
                                        ),
                                    ]
                                    .into_iter(),
                                )
                                .collect::<Vec<_>>()
                        },
                    ),
            );
            sheet_rows.push(vec![cell_data(None, None)]);
        }
        {
            let mut result_details = result_details.into_iter().collect::<Vec<_>>();
            result_details.sort_by_key(|i| i.0.number);

            for (sax3::Sax3DanceKey { name, .. }, dance_res_by_couple_number) in result_details {
                sheet_rows.push({
                    let mut ret = (0..ident + 1)
                        .map(|_| cell_data(None, None))
                        .chain(
                            vec![
                                cell_data(
                                    Some(CompactExtendedValue::String(name.clone())),
                                    CELL_FORMAT_TITLE.clone(),
                                ),
                                cell_data(
                                    Some(CompactExtendedValue::String("номер пары".to_owned())),
                                    CELL_FORMAT_CAPTION_RIGHT.clone(),
                                ),
                                cell_data(
                                    Some(CompactExtendedValue::String("класс".to_owned())),
                                    CELL_FORMAT_CAPTION_CENTER.clone(),
                                ),
                                cell_data(
                                    Some(CompactExtendedValue::String(
                                        match mode {
                                            Sax3RoundMode::Ball => "сумма",
                                            Sax3RoundMode::Skating => "место",
                                            Sax3RoundMode::Sum => "крестов",
                                        }
                                        .to_owned(),
                                    )),
                                    CELL_FORMAT_CAPTION_RIGHT.clone(),
                                ),
                            ]
                            .into_iter(),
                        )
                        .collect::<Vec<_>>();
                    ret.extend(
                        judges
                            .as_ref()
                            .or(compet_judges.as_ref())
                            .unwrap()
                            .private()
                            .into_iter()
                            .map(|(judge_id, _)| {
                                cell_data(
                                    Some(CompactExtendedValue::String(judge_id.to_string())),
                                    CELL_FORMAT_JUDGE_CODE.clone(),
                                )
                            })
                            .collect::<Vec<_>>(),
                    );

                    ret
                });
                let mut dance_reslar = dance_res_by_couple_number.into_iter().collect::<Vec<_>>();
                dance_reslar.sort_by_key(|i| {
                    (i.1.place.or_else(|| i.1.sum.map(|v| -v)).unwrap_or(100f64) * 100f64) as i64
                });
                struct Couple {
                    place: Option<f64>,
                    sum: Option<f64>,
                    couple_number: i16,
                    class: Option<String>,
                    details: Sax3RoundDanceResDetails,
                }
                let mut rows: Vec<Couple> = vec![];
                for (
                    couple_number,
                    Sax3RoundDanceRes {
                        sum,
                        place,
                        details,
                    },
                ) in dance_reslar
                {
                    let Some(sax3::Sax3Couple {
                        class,
                        ..
                    }) = couples.get(&couple_number).cloned() else {
                        unreachable!("couple_number: {couple_number}, couples: {couples:#?}");
                    };
                    rows.push(Couple {
                        couple_number,
                        place,
                        sum,
                        details,
                        class,
                    })
                }
                sheet_rows.extend(rows.into_iter().map(
                    |Couple {
                         couple_number,
                         place,
                         sum,
                         class,
                         details,
                     }| {
                        let mut ret = (0..ident + 2)
                            .map(|_| cell_data(None, None))
                            .chain(
                                vec![
                                    cell_data(
                                        Some(CompactExtendedValue::Int(couple_number as i64)),
                                        None,
                                    ),
                                    cell_data(
                                        class
                                            .as_ref()
                                            .map(|s| CompactExtendedValue::String(s.clone())),
                                        CELL_FORMAT_CAPTION_CENTER.clone(),
                                    ),
                                    cell_data(
                                        match mode {
                                            Sax3RoundMode::Ball => sum,
                                            Sax3RoundMode::Skating => place,
                                            Sax3RoundMode::Sum => sum,
                                        }
                                        .map(|v| CompactExtendedValue::Int(v as i64)),
                                        None,
                                    ),
                                ]
                                .into_iter(),
                            )
                            .collect::<Vec<_>>();
                        ret.extend(
                            judges
                                .as_ref()
                                .or(compet_judges.as_ref())
                                .unwrap()
                                .private()
                                .into_iter()
                                .map(|(judge_id, _)| match &details {
                                    Sax3RoundDanceResDetails::Crosses(crosses) => cell_data(
                                        crosses.contains(&judge_id.get_i()).then_some(
                                            CompactExtendedValue::String("X".to_owned()),
                                        ),
                                        CELL_FORMAT_CAPTION_RIGHT.clone(),
                                    ),
                                    Sax3RoundDanceResDetails::Places(places) => cell_data(
                                        places.get(&judge_id.get_i()).map(|place| {
                                            CompactExtendedValue::Int(*place as i64 + 1)
                                        }),
                                        None,
                                    ),
                                }),
                        );
                        ret
                    },
                ));
                sheet_rows.push(vec![cell_data(None, None)]);
            }
        }
    }
}

// ===================================================================

pub fn categorilar_summary(
    categorilar: Vec<String>,
    dancerlar_points: SummaryDancerlarPoints,
    total_caption: String,
    sheet_rows: &mut Vec<Vec<spreadsheets::CellData>>,
) {
    let mut dancerlar_points = dancerlar_points.into_iter().collect::<Vec<_>>();
    dancerlar_points.sort_by_key(|i| i.0.couple_number);

    sheet_rows.extend(
        categorilar
            .iter()
            .enumerate()
            .map(|(i, categori)| {
                vec![
                    cell_data(
                        Some(CompactExtendedValue::Int((i + 1) as i64)),
                        CELL_FORMAT_JUDGE_CODE.clone(),
                    ),
                    cell_data(Some(CompactExtendedValue::String(categori.clone())), None),
                ]
            })
            .collect::<Vec<_>>(),
    );

    sheet_rows.push(vec![cell_data(None, None)]);

    sheet_rows.push({
        let mut ret = vec![
            cell_data(
                Some(CompactExtendedValue::String("номер пары".to_owned())),
                CELL_FORMAT_CAPTION_SUMMARY_LEFT.clone(),
            ),
            cell_data(
                Some(CompactExtendedValue::String("фамилия".to_owned())),
                CELL_FORMAT_CAPTION_SUMMARY_LEFT.clone(),
            ),
            cell_data(
                Some(CompactExtendedValue::String("имя".to_owned())),
                CELL_FORMAT_CAPTION_SUMMARY_LEFT.clone(),
            ),
            cell_data(
                Some(CompactExtendedValue::String("класс".to_owned())),
                CELL_FORMAT_CAPTION_SUMMARY_CENTER.clone(),
            ),
            cell_data(
                Some(CompactExtendedValue::String("город".to_owned())),
                CELL_FORMAT_CAPTION_SUMMARY_LEFT.clone(),
            ),
            cell_data(
                Some(CompactExtendedValue::String("клуб".to_owned())),
                CELL_FORMAT_CAPTION_SUMMARY_LEFT.clone(),
            ),
            cell_data(
                Some(CompactExtendedValue::String(total_caption)),
                CELL_FORMAT_CAPTION_SUMMARY_TOTAL.clone(),
            ),
        ];
        for i in 0..categorilar.len() {
            ret.push(cell_data(
                Some(CompactExtendedValue::Int((i + 1) as i64)),
                CELL_FORMAT_JUDGE_CODE.clone(),
            ));
        }
        ret
    });
    sheet_rows.extend(
        dancerlar_points
            .into_iter()
            .map(
                |(
                    SummaryDancer {
                        couple_number,
                        last_name,
                        first_name,
                        class,
                        city,
                        club,
                        ..
                    },
                    points,
                )| {
                    let mut ret = vec![
                        cell_data(Some(CompactExtendedValue::Int(couple_number.into())), None),
                        cell_data(Some(CompactExtendedValue::String(last_name)), None),
                        cell_data(Some(CompactExtendedValue::String(first_name)), None),
                        cell_data(
                            class.map(CompactExtendedValue::String),
                            CELL_FORMAT_CAPTION_CENTER.clone(),
                        ),
                        cell_data(Some(CompactExtendedValue::String(city)), None),
                        cell_data(Some(CompactExtendedValue::String(club)), None),
                    ];
                    ret.push(cell_data(
                        Some(CompactExtendedValue::Float(points.values().sum::<f64>())),
                        CELL_FORMAT_TOTAL.clone(),
                    ));
                    for key in categorilar.iter() {
                        ret.push(cell_data(
                            points.get(key).map(|v| CompactExtendedValue::Float(*v)),
                            None,
                        ));
                    }
                    ret
                },
            )
            .collect::<Vec<_>>(),
    );

    sheet_rows.push(vec![cell_data(None, None)]);
}

// ===================================================================
//
use strum::IntoEnumIterator;
#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, strum::Display, strum::EnumIter,
)]
pub enum BeginnerKind {
    #[strum(serialize = "Н-2")]
    N2,
    #[strum(serialize = "Н-3")]
    N3,
    #[strum(serialize = "Н-4")]
    N4,
    #[strum(serialize = "Н-5")]
    N5,
    #[strum(serialize = "Прочее")]
    Other,
}

pub fn beginnerlar_summary(
    dancerlar_points: HashMap<SummaryDancer, HashMap<BeginnerKind, f64>>,
    total_caption: String,
    sheet_rows: &mut Vec<Vec<spreadsheets::CellData>>,
) {
    let mut dancerlar_points = dancerlar_points.into_iter().collect::<Vec<_>>();
    dancerlar_points.sort_by_key(|i| i.0.couple_number);

    let mut beginner_kind_set = HashSet::new();
    sheet_rows.push({
        let mut ret = vec![
            cell_data(
                Some(CompactExtendedValue::String("номер пары".to_owned())),
                CELL_FORMAT_CAPTION_SUMMARY_LEFT.clone(),
            ),
            cell_data(
                Some(CompactExtendedValue::String("фамилия".to_owned())),
                CELL_FORMAT_CAPTION_SUMMARY_LEFT.clone(),
            ),
            cell_data(
                Some(CompactExtendedValue::String("имя".to_owned())),
                CELL_FORMAT_CAPTION_SUMMARY_LEFT.clone(),
            ),
            cell_data(
                Some(CompactExtendedValue::String("класс".to_owned())),
                CELL_FORMAT_CAPTION_SUMMARY_CENTER.clone(),
            ),
            cell_data(
                Some(CompactExtendedValue::String("город".to_owned())),
                CELL_FORMAT_CAPTION_SUMMARY_LEFT.clone(),
            ),
            cell_data(
                Some(CompactExtendedValue::String("клуб".to_owned())),
                CELL_FORMAT_CAPTION_SUMMARY_LEFT.clone(),
            ),
            cell_data(
                Some(CompactExtendedValue::String(total_caption)),
                CELL_FORMAT_CAPTION_SUMMARY_TOTAL.clone(),
            ),
        ];
        for beginner_kind in BeginnerKind::iter() {
            if dancerlar_points
                .iter()
                .any(|(_, points)| points.get(&beginner_kind).is_some())
            {
                beginner_kind_set.insert(beginner_kind);
                ret.push(cell_data(
                    Some(CompactExtendedValue::String(beginner_kind.to_string())),
                    CELL_FORMAT_CAPTION_SUMMARY_RIGHT.clone(),
                ));
            }
        }
        ret
    });
    sheet_rows.extend(
        dancerlar_points
            .into_iter()
            .map(
                |(
                    SummaryDancer {
                        couple_number,
                        last_name,
                        first_name,
                        class,
                        city,
                        club,
                        ..
                    },
                    points,
                )| {
                    let mut ret = vec![
                        cell_data(Some(CompactExtendedValue::Int(couple_number.into())), None),
                        cell_data(Some(CompactExtendedValue::String(last_name)), None),
                        cell_data(Some(CompactExtendedValue::String(first_name)), None),
                        cell_data(
                            class.map(CompactExtendedValue::String),
                            CELL_FORMAT_CAPTION_CENTER.clone(),
                        ),
                        cell_data(Some(CompactExtendedValue::String(city)), None),
                        cell_data(Some(CompactExtendedValue::String(club)), None),
                    ];
                    ret.push(cell_data(
                        Some(CompactExtendedValue::Float(points.values().sum::<f64>())),
                        CELL_FORMAT_TOTAL.clone(),
                    ));
                    for beginner_kind in BeginnerKind::iter() {
                        if beginner_kind_set.contains(&beginner_kind) {
                            ret.push(cell_data(
                                points
                                    .get(&beginner_kind)
                                    .map(|v| CompactExtendedValue::Float(*v)),
                                None,
                            ));
                        }
                    }
                    ret
                },
            )
            .collect::<Vec<_>>(),
    );

    sheet_rows.push(vec![cell_data(None, None)]);
}

// ===================================================================
