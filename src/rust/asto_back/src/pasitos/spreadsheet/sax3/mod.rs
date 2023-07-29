use super::*;

use pasitos::sax3::*;
use std::collections::{HashMap, HashSet};
pub mod utils;
use utils::*;

pub type ExportSax3Ret = Vec<ImportEventRow>;
pub type ExportSax3Result = Result<ExportSax3Ret>;
use pasitos::sax3::Sax3Dancer;
pub async fn export_sax3(
    data: pasitos::sax3::Sax3Ret,
    opts: pasitos::sax3::Sax3Opts,
) -> ExportSax3Result {
    let mut ret = vec![];
    let service_account_secret_file = settings!(spreadsheet.service_account_secret_file).clone();
    let spreadsheet_id = settings!(spreadsheet.event_result_spreadsheet_id).clone();
    let key = yup_oauth2::read_service_account_key(&service_account_secret_file)
        .await
        .map_err(|err| anyhow!("read_service_account_key: {err}"))?;

    type Gradient = String;
    type Dance = String;
    type Dances = Vec<Dance>;
    type Phase = String;
    type PhasesByDance = HashMap<Dance, Vec<(Phase, Sax3Compet)>>;

    let gradient_category_prefixlar: HashMap<&str, Vec<&str>> = vec![
        (
            "2022-11-06",
            vec![
                "Соло, Юниоры-1, Юниоры-2, Молодежь",
                "Молодежь, Взрослые, Сеньоры",
            ],
        ),
        ("2023-03-19", vec!["Градиент"]),
    ]
    .into_iter()
    .collect();
    let skip_category_prefixlar: HashMap<&str, Vec<&str>> = vec![(
        "2023-03-19",
        vec![
            "Шербургские зонтики",
            "Ветер перемен",
            "Зажигалки",
            "Move",
            "Загадка",
            "Соло, Сеньоры",
        ],
    )]
    .into_iter()
    .collect();

    if opts.judges {
        #[derive(Debug, Clone, Serialize)]
        pub struct YamlRound {
            pub name: String,
            pub judges: Option<Sax3Judges>,
        }
        #[derive(Debug, Serialize)]
        pub struct YamlCompet {
            pub category: String,
            pub rounds: HashMap<Sax3RoundNumber, YamlRound>,
            pub judges: Option<Sax3Judges>,
        }
        type YamlData = HashMap<Sax3RetKey, Vec<YamlCompet>>;
        let mut yaml_data = YamlData::default();
        for (ret_key, compets) in data {
            let mut yaml_compets = Vec::new();
            'COMPETS: for compet in compets
                .into_iter()
                .filter(|compet| !compet.couples.is_empty())
            {
                if let Some(skip_category_prefixlar) =
                    skip_category_prefixlar.get(ret_key.date.to_string().as_str())
                {
                    for skip_category_prefix in skip_category_prefixlar.iter() {
                        if let Some(_category) = compet.category.strip_prefix(skip_category_prefix)
                        {
                            continue 'COMPETS;
                        }
                    }
                }
                let Sax3Compet {
                    category,
                    rounds,
                    judges,
                    ..
                } = compet;
                let mut yaml_rounds = HashMap::new();
                for (round_number, Sax3Round { name, judges, .. }) in rounds {
                    yaml_rounds.insert(round_number, YamlRound { name, judges });
                }
                let yaml_compet = YamlCompet {
                    category,
                    rounds: yaml_rounds,
                    judges,
                };
                yaml_compets.push(yaml_compet);
            }
            yaml_data.insert(ret_key, yaml_compets);
        }

        println!("{}", serde_yaml::to_string(&yaml_data).unwrap());
        std::process::exit(0);
    }

    let mut requests = vec![];
    for (ret_key, compets) in data {
        let mut gradient_compets: HashMap<Gradient, (Dances, PhasesByDance)> = HashMap::new();
        let mut non_gradient_compets = vec![];

        'COMPETS: for compet in compets
            .into_iter()
            .filter(|compet| !compet.couples.is_empty())
        {
            if let Some(skip_category_prefixlar) =
                skip_category_prefixlar.get(ret_key.date.to_string().as_str())
            {
                for skip_category_prefix in skip_category_prefixlar.iter() {
                    if let Some(_category) = compet.category.strip_prefix(skip_category_prefix) {
                        continue 'COMPETS;
                    }
                }
            }

            let non_gradient_compet = {
                let mut ret = Some(compet);
                if let Some(gradient_category_prefixlar) =
                    gradient_category_prefixlar.get(ret_key.date.to_string().as_str())
                {
                    for gradient in gradient_category_prefixlar.iter() {
                        if let Some(category) =
                            ret.as_ref().unwrap().category.strip_prefix(gradient)
                        {
                            let category = category.trim_matches(|ch| matches!(ch, ' ' | ','));
                            let splitted = category.split('(').collect::<Vec<_>>();
                            let dance = splitted[0].trim().to_owned();
                            let phase = splitted[1].trim_end_matches(')').to_owned();
                            let gradient = gradient.to_string();
                            let compet = ret.take().unwrap();
                            common_macros2::entry!(gradient_compets, gradient
                            =>
                                and_modify |e| {
                                    e.0.push(dance.clone());
                                    common_macros2::entry!(e.1, dance
                                    =>
                                        and_modify |e| {
                                            e.push((phase, compet));
                                        }
                                        or_insert vec![(phase, compet)]
                                    );
                                }
                                or_insert (vec![dance.clone()], vec![(dance, vec![(phase, compet)])].into_iter().collect())
                            );
                            break;
                        }
                    }
                }
                ret
            };
            if let Some(compet) = non_gradient_compet {
                non_gradient_compets.push(compet);
            }
        }

        let mut sheet_rows_proto = vec![
            vec![cell_data(
                Some(CompactExtendedValue::String(ret_key.to_string())),
                CELL_FORMAT_H1.clone(),
            )],
            vec![cell_data(None, None)],
        ];

        let mut sheet_rows_summary = vec![
            vec![cell_data(
                Some(CompactExtendedValue::String(ret_key.to_string())),
                CELL_FORMAT_H1.clone(),
            )],
            vec![cell_data(None, None)],
        ];

        {
            let mut beginner_compets = vec![];
            let mut non_beginner_compets = vec![];

            for compet in non_gradient_compets {
                let kind = get_kind_of_category(&compet.category);
                match kind {
                    CompetKind::Кубок | CompetKind::Аттестация => {
                        beginner_compets.push((kind, compet));
                    }
                    CompetKind::Категория => {
                        non_beginner_compets.push((kind, compet));
                    }
                }
            }

            if !beginner_compets.is_empty() {
                sheet_rows_proto.extend(vec![
                    vec![cell_data(
                        Some(CompactExtendedValue::String(
                            "Начинающие танцоры".to_string(),
                        )),
                        CELL_FORMAT_H2.clone(),
                    )],
                    vec![cell_data(None, None)],
                ]);

                sheet_rows_summary.extend(vec![
                    vec![cell_data(
                        Some(CompactExtendedValue::String(
                            "Начинающие танцоры".to_string(),
                        )),
                        CELL_FORMAT_H2.clone(),
                    )],
                    vec![cell_data(None, None)],
                ]);

                let mut sheet_rows_summary_temp = vec![];

                let beginner_compets = {
                    let mut ret = HashMap::<BeginnerKind, Vec<(CompetKind, Sax3Compet)>>::new();
                    for (kind, compet) in beginner_compets {
                        let category = compet.category.to_lowercase();
                        let key = if category.contains(" н-2")
                            || category.contains(" н2")
                            || category.starts_with("н2")
                        {
                            BeginnerKind::N2
                        } else if category.contains(" н-3")
                            || category.contains(" н3")
                            || category.starts_with("н3")
                        {
                            BeginnerKind::N3
                        } else if category.contains(" н-4")
                            || category.contains(" н4")
                            || category.starts_with("н4")
                        {
                            BeginnerKind::N4
                        } else if category.contains(" н-5")
                            || category.contains(" н5")
                            || category.starts_with("н5")
                        {
                            BeginnerKind::N5
                        } else {
                            BeginnerKind::Other
                        };
                        common_macros2::entry!(ret, key
                        =>
                            and_modify |e| {
                                e.push((kind, compet));
                            }
                            or_insert vec![(kind, compet)]
                        );
                    }
                    let mut ret = ret.into_iter().collect::<Vec<_>>();
                    ret.sort_by_key(|i| i.0);
                    ret
                };

                let mut dancerlar_points_summary: HashMap<
                    SummaryDancer,
                    HashMap<BeginnerKind, f64>,
                > = HashMap::new();
                for (beginner_kind, compets) in beginner_compets {
                    sheet_rows_proto.extend(vec![
                        vec![cell_data(
                            Some(CompactExtendedValue::String(beginner_kind.to_string())),
                            CELL_FORMAT_H3.clone(),
                        )],
                        vec![cell_data(None, None)],
                    ]);

                    sheet_rows_summary_temp.extend(vec![
                        vec![cell_data(
                            Some(CompactExtendedValue::String(beginner_kind.to_string())),
                            CELL_FORMAT_H3.clone(),
                        )],
                        vec![cell_data(None, None)],
                    ]);

                    let mut dancerlar_points: SummaryDancerlarPoints = HashMap::new();
                    let mut categorilar = vec![];
                    for (
                        kind,
                        Sax3Compet {
                            category,
                            couples,
                            rounds,
                            judges,
                        },
                    ) in compets
                    {
                        sheet_rows_proto.extend(vec![
                            vec![
                                cell_data(
                                    Some(CompactExtendedValue::String(format!("{kind}:"))),
                                    CELL_FORMAT_LABEL.clone(),
                                ),
                                cell_data(
                                    Some(CompactExtendedValue::String(category.clone())),
                                    CELL_FORMAT_TITLE.clone(),
                                ),
                            ]
                            .into_iter()
                            .chain((0..13).map(|_| cell_data(None, CELL_FORMAT_TITLE.clone())))
                            .collect::<Vec<_>>(),
                            vec![cell_data(None, None)],
                        ]);
                        let ident = 1;
                        categorilar.push(category.clone());
                        process_rounds(
                            &ret_key,
                            judges,
                            rounds,
                            couples,
                            kind,
                            if matches!(kind, CompetKind::Кубок) {
                                RoundKind::BeginnersCup
                            } else {
                                RoundKind::Skating
                            },
                            DanceCategory::Name(category),
                            ident,
                            &mut sheet_rows_proto,
                            &mut dancerlar_points,
                            &mut ret,
                        );
                    }
                    for (dancer, points) in dancerlar_points.iter() {
                        let points = points.iter().map(|(_, v)| v).sum::<f64>();
                        common_macros2::entry!(dancerlar_points_summary, (*dancer).clone()
                        =>
                            and_modify |e| {
                                e.insert(beginner_kind, points)
                            }
                            or_insert vec![(beginner_kind, points)].into_iter().collect::<HashMap<_, _>>()
                        )
                    }
                    categorilar_summary(
                        categorilar,
                        dancerlar_points,
                        "всего".to_owned(),
                        &mut sheet_rows_summary_temp,
                    );
                }

                beginnerlar_summary(
                    dancerlar_points_summary,
                    "итого".to_owned(),
                    &mut sheet_rows_summary,
                );

                sheet_rows_summary.extend(sheet_rows_summary_temp);
            }

            if !non_beginner_compets.is_empty() {
                sheet_rows_proto.extend(vec![
                    vec![cell_data(
                        Some(CompactExtendedValue::String(
                            "Классовые танцоры".to_string(),
                        )),
                        CELL_FORMAT_H2.clone(),
                    )],
                    vec![cell_data(None, None)],
                ]);

                sheet_rows_summary.extend(vec![
                    vec![cell_data(
                        Some(CompactExtendedValue::String(
                            "Классовые танцоры".to_string(),
                        )),
                        CELL_FORMAT_H2.clone(),
                    )],
                    vec![cell_data(None, None)],
                ]);

                let mut dancerlar_points: SummaryDancerlarPoints = HashMap::new();
                let mut categorilar = vec![];
                for (
                    kind,
                    Sax3Compet {
                        category,
                        couples,
                        rounds,
                        judges,
                    },
                ) in non_beginner_compets
                {
                    sheet_rows_proto.extend(vec![
                        vec![
                            cell_data(
                                Some(CompactExtendedValue::String(format!("{kind}:"))),
                                CELL_FORMAT_LABEL.clone(),
                            ),
                            cell_data(
                                Some(CompactExtendedValue::String(category.clone())),
                                CELL_FORMAT_TITLE.clone(),
                            ),
                        ]
                        .into_iter()
                        .chain((0..13).map(|_| cell_data(None, CELL_FORMAT_TITLE.clone())))
                        .collect::<Vec<_>>(),
                        vec![cell_data(None, None)],
                    ]);
                    let ident = 1;
                    categorilar.push(category.clone());
                    process_rounds(
                        &ret_key,
                        judges,
                        rounds,
                        couples,
                        kind,
                        if matches!(kind, CompetKind::Кубок) {
                            RoundKind::BeginnersCup
                        } else {
                            RoundKind::Skating
                        },
                        DanceCategory::Name(category),
                        ident,
                        &mut sheet_rows_proto,
                        &mut dancerlar_points,
                        &mut ret,
                    );
                }
                categorilar_summary(
                    categorilar,
                    dancerlar_points,
                    "итого".to_owned(),
                    &mut sheet_rows_summary,
                );
            }
        }

        let gradient_compets = {
            let mut ret = vec![];
            if let Some(gradient_category_prefixlar) =
                gradient_category_prefixlar.get(ret_key.date.to_string().as_str())
            {
                for gradient in gradient_category_prefixlar {
                    if let Some((dancelar, mut phases_by_dance)) =
                        gradient_compets.remove(*gradient)
                    {
                        let mut dances = vec![];
                        for dance in dancelar {
                            if let Some(phases) = phases_by_dance.remove(&dance) {
                                dances.push((dance, phases));
                            }
                        }
                        ret.push((gradient, dances));
                    }
                }
            }
            ret
        };

        if !gradient_compets.is_empty() {
            sheet_rows_proto.extend(vec![
                vec![cell_data(
                    Some(CompactExtendedValue::String("Градиенты".to_string())),
                    CELL_FORMAT_H2.clone(),
                )],
                vec![cell_data(None, None)],
            ]);

            sheet_rows_summary.extend(vec![
                vec![cell_data(
                    Some(CompactExtendedValue::String(
                        if gradient_compets.len() == 1 {
                            "Градиент"
                        } else {
                            "Градиенты"
                        }
                        .to_string(),
                    )),
                    CELL_FORMAT_H2.clone(),
                )],
                vec![cell_data(None, None)],
            ]);

            for (gradient, dances) in gradient_compets {
                let mut dancerlar_points: SummaryDancerlarPoints = HashMap::new();
                if opts.proto {
                    sheet_rows_proto.extend(vec![
                        vec![
                            cell_data(
                                Some(CompactExtendedValue::String("Градиент:".to_owned())),
                                CELL_FORMAT_LABEL.clone(),
                            ),
                            cell_data(
                                Some(CompactExtendedValue::String(gradient.to_string())),
                                CELL_FORMAT_TITLE.clone(),
                            ),
                        ]
                        .into_iter()
                        .chain((0..13).map(|_| cell_data(None, CELL_FORMAT_TITLE.clone())))
                        .collect::<Vec<_>>(),
                        vec![cell_data(None, None)],
                    ]);
                }
                for (dance, phases) in dances.into_iter() {
                    sheet_rows_proto.extend(vec![
                        (0..1)
                            .map(|_| cell_data(None, None))
                            .chain(
                                vec![
                                    cell_data(
                                        Some(CompactExtendedValue::String("Танец:".to_owned())),
                                        CELL_FORMAT_LABEL.clone(),
                                    ),
                                    cell_data(
                                        Some(CompactExtendedValue::String(dance.to_string())),
                                        CELL_FORMAT_TITLE.clone(),
                                    ),
                                    cell_data(None, CELL_FORMAT_TITLE.clone()),
                                ]
                                .into_iter(),
                            )
                            .collect::<Vec<_>>(),
                        vec![cell_data(None, None)],
                    ]);
                    for (
                        phase,
                        Sax3Compet {
                            category: _,
                            couples,
                            rounds,
                            judges,
                        },
                    ) in phases
                    {
                        let kind = CompetKind::Категория;
                        sheet_rows_proto.extend(vec![
                            (0..2)
                                .map(|_| cell_data(None, None))
                                .chain(
                                    vec![
                                        cell_data(
                                            Some(CompactExtendedValue::String("Этап:".to_owned())),
                                            CELL_FORMAT_LABEL.clone(),
                                        ),
                                        cell_data(
                                            Some(CompactExtendedValue::String(phase.to_string())),
                                            CELL_FORMAT_TITLE.clone(),
                                        ),
                                    ]
                                    .into_iter(),
                                )
                                .collect::<Vec<_>>(),
                            vec![cell_data(None, None)],
                        ]);
                        let ident = 3;
                        process_rounds(
                            &ret_key,
                            judges,
                            rounds,
                            couples,
                            kind,
                            RoundKind::Gradient,
                            DanceCategory::Gradient(gradient.to_string(), phase),
                            ident,
                            &mut sheet_rows_proto,
                            &mut dancerlar_points,
                            &mut ret,
                        );
                    }
                }

                sheet_rows_summary.extend(vec![vec![
                    cell_data(
                        Some(CompactExtendedValue::String("Градиент:".to_owned())),
                        CELL_FORMAT_LABEL.clone(),
                    ),
                    cell_data(
                        Some(CompactExtendedValue::String(gradient.to_string())),
                        CELL_FORMAT_TITLE.clone(),
                    ),
                ]
                .into_iter()
                .chain((0..15).map(|_| cell_data(None, CELL_FORMAT_TITLE.clone())))
                .collect::<Vec<_>>()]);

                let mut dancerlar_points = dancerlar_points.into_iter().collect::<Vec<_>>();
                dancerlar_points.sort_by_key(|i| i.0.couple_number);

                let dances = vec!["W", "T", "V", "F", "Q", "S", "Ch", "R", "P", "J"];
                sheet_rows_summary.push({
                    let mut ret = vec![
                        cell_data(
                            Some(CompactExtendedValue::String("номер пары".to_owned())),
                            CELL_FORMAT_CAPTION_LEFT.clone(),
                        ),
                        cell_data(
                            Some(CompactExtendedValue::String("фамилия".to_owned())),
                            None,
                        ),
                        cell_data(Some(CompactExtendedValue::String("имя".to_owned())), None),
                        cell_data(
                            Some(CompactExtendedValue::String("класс".to_owned())),
                            CELL_FORMAT_CAPTION_CENTER.clone(),
                        ),
                        cell_data(Some(CompactExtendedValue::String("город".to_owned())), None),
                        cell_data(Some(CompactExtendedValue::String("клуб".to_owned())), None),
                    ];
                    ret.push(cell_data(
                        Some(CompactExtendedValue::String("итого".to_owned())),
                        CELL_FORMAT_TOTAL.clone(),
                    ));
                    for dance in dances.iter() {
                        ret.push(cell_data(
                            Some(CompactExtendedValue::String(dance.to_string())),
                            CELL_FORMAT_CAPTION_RIGHT.clone(),
                        ));
                    }
                    ret
                });
                for (
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
                ) in dancerlar_points
                {
                    sheet_rows_summary.push({
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
                        for key in dances.iter() {
                            ret.push(cell_data(
                                points.get(*key).map(|v| CompactExtendedValue::Float(*v)),
                                None,
                            ));
                        }
                        ret
                    });
                }
                sheet_rows_summary.push(vec![cell_data(None, None)]);
            }
        }
        if opts.summary {
            let sheet_id_summary = get_sheet_props(
                &format!("{}::Результаты", ret_key.date),
                &spreadsheet_id,
                key.clone(),
            )
            .await?
            .map(|SheetProps { sheet_id, .. }| sheet_id)
            .unwrap();
            requests.push(append_cells_request(sheet_rows_summary, sheet_id_summary));
        }

        if opts.proto {
            let sheet_id_proto = get_sheet_props(
                &format!("{}::Протокол", ret_key.date),
                &spreadsheet_id,
                key.clone(),
            )
            .await?
            .map(|SheetProps { sheet_id, .. }| sheet_id)
            .unwrap();
            requests.push(append_cells_request(sheet_rows_proto, sheet_id_proto));
        }
    }
    if !requests.is_empty() {
        let _: BatchUpdateSpreadsheetResponse =
            spreadsheets_batch_update(requests, &spreadsheet_id, key).await?;
    }
    Ok(ret)
}

pub fn export_sax3_sync(res: ExportSax3Result, opts: pasitos::sax3::Sax3Opts) -> Result<()> {
    match res {
        Err(err) => {
            error!("{}:{}: {err}", file!(), line!())
        }
        Ok(data) => {
            if opts.database {
                pasitos!(db push_back ImportEvent { data, opts });
            } else {
                println!("use -d to import_event to database");
            }
        }
    }
    Ok(())
}

pub fn get_kind_of_category(category: &str) -> CompetKind {
    let category = category.to_lowercase();
    if category.contains(" н-")
        || (category.contains(" н2")
            || category.contains(" н3")
            || category.contains(" н4")
            || category.contains(" н5")
            || category.contains(" н6"))
        || category.starts_with("н2")
        || category.starts_with("н3")
        || category.starts_with("н4")
        || category.starts_with("н5")
        || category.starts_with("н6")
    {
        if category.contains("кубок") {
            CompetKind::Кубок
        } else {
            CompetKind::Аттестация
        }
    } else if category.contains("кубок") && !category.contains("класс") {
        CompetKind::Кубок
    } else if category.contains("аттестация") || category.starts_with("атт ") {
        CompetKind::Аттестация
    } else {
        CompetKind::Категория
    }
}

// ===================================================================
