use super::*;

use spreadsheets::*;

pub mod sax3;
pub use sax3::*;

// ===================================================================

pub type ExportDancersForAntonResult = Result<()>;
pub async fn export_dancers_for_anton(
    items: Vec<ForAntonDancerRow>,
) -> ExportDancersForAntonResult {
    let service_account_secret_file = settings!(for_anton.service_account_secret_file).clone();
    let spreadsheet_id = settings!(for_anton.id).clone();
    let dancers_sheet_name = settings!(for_anton.export_dancers_sheet_name).clone();
    let key = yup_oauth2::read_service_account_key(&service_account_secret_file)
        .await
        .map_err(|err| anyhow!("read_service_account_key: {err}"))?;

    let sheet_id = get_sheet_props(&dancers_sheet_name, &spreadsheet_id, key.clone())
        .await?
        .map(|SheetProps { sheet_id, .. }| sheet_id)
        .unwrap();
    let columns = [
        "№ МФТС",
        "Фамилия Имя",
        "Отчество",
        "Дата рождения",
        "Класс ST",
        "Дата (ST)",
        "Класс LA",
        "Дата (LA)",
        "Разряд",
        "Дата (Разряд)",
        "Транслит",
        "Клуб",
        "Город",
        "Руководитель (ст. тренер)",
        "Тренер 1",
        "Тренер 2",
        "Регион",
        "Переход",
        "Пол",
        "Партнерша",
    ];
    let rows: Vec<Vec<CellData>> = vec![None]
        .into_iter()
        .map(|_: Option<()>| {
            columns
                .iter()
                .map(|col| cell_data(Some(CompactExtendedValue::String(col.to_string())), None))
                .collect::<Vec<_>>()
        })
        .chain(items.into_iter().map(
            |ForAntonDancerRow {
                 external_id,
                 name,
                 second_name,
                 birth_date,
                 st_class,
                 la_class,
                 club,
                 citi,
                 trainer,
                 trainer2,
                 chief,
                 region,
                 gender,
             }| {
                columns
                    .iter()
                    .map(|col| match *col {
                        "№ МФТС" => cell_data(
                            external_id
                                .map(|external_id| CompactExtendedValue::Int(external_id as i64)),
                            None,
                        ),
                        "Фамилия Имя" => cell_data(
                            name.as_ref()
                                .map(|s| CompactExtendedValue::String(s.clone())),
                            None,
                        ),
                        "Отчество" => cell_data(
                            second_name
                                .as_ref()
                                .map(|s| CompactExtendedValue::String(s.clone())),
                            None,
                        ),
                        "Дата рождения" => birth_date
                            .map(cell_data_from_naive_date)
                            .unwrap_or_else(|| cell_data(None, None)),
                        "Класс ST" => cell_data(
                            st_class
                                .as_ref()
                                .map(|s| CompactExtendedValue::String(s.clone())),
                            None,
                        ),
                        "Дата (ST)" | "Дата (LA)" => {
                            cell_data_from_naive_date(chrono::Utc::now().date_naive())
                        }
                        "Класс LA" => cell_data(
                            la_class
                                .as_ref()
                                .map(|s| CompactExtendedValue::String(s.clone())),
                            None,
                        ),
                        "Руководитель (ст. тренер)" => cell_data(
                            chief
                                .as_ref()
                                .or(trainer.as_ref())
                                // trainer
                                .as_ref()
                                .map(|s| CompactExtendedValue::String((*s).clone())),
                            None,
                        ),
                        "Тренер 1" => cell_data(
                            trainer
                                .as_ref()
                                .map(|s| CompactExtendedValue::String(s.clone())),
                            None,
                        ),
                        "Тренер 2" => cell_data(
                            trainer2
                                .as_ref()
                                .map(|s| CompactExtendedValue::String(s.clone())),
                            None,
                        ),
                        "Клуб" => cell_data(
                            club.as_ref()
                                .map(|s| CompactExtendedValue::String(s.clone())),
                            None,
                        ),
                        "Город" => cell_data(
                            citi.as_ref()
                                .map(|s| CompactExtendedValue::String(s.clone())),
                            None,
                        ),
                        "Регион" => {
                            cell_data(region.map(|i| CompactExtendedValue::Int(i as i64)), None)
                        }
                        "Пол" => cell_data(
                            gender
                                .as_ref()
                                .map(|s| CompactExtendedValue::String(s.clone())),
                            None,
                        ),
                        _ => cell_data(None, None),
                    })
                    .collect::<Vec<_>>()
            },
        ))
        .collect::<Vec<_>>();
    let requests = vec![append_cells_request(rows, sheet_id)];
    let _: BatchUpdateSpreadsheetResponse =
        spreadsheets_batch_update(requests, &spreadsheet_id, key).await?;
    Ok(())
}

pub fn export_dancers_for_anton_sync(res: ExportDancersForAntonResult) -> Result<()> {
    if let Err(err) = res {
        error!("{}:{}: {err}", file!(), line!())
    }
    Ok(())
}

// ===================================================================

pub type ExportSaxResult = Result<()>;
// use pasitos::sax::{SaxDancer, SaxRet};
use pasitos::sax::SaxDancer;
pub async fn export_sax(data: pasitos::sax::SaxRet) -> ExportSaxResult {
    let service_account_secret_file = settings!(spreadsheet.service_account_secret_file).clone();
    let spreadsheet_id = settings!(spreadsheet.event_result_spreadsheet_id).clone();

    let key = yup_oauth2::read_service_account_key(&service_account_secret_file)
        .await
        .map_err(|err| anyhow!("read_service_account_key: {err}"))?;

    let mut requests = vec![];
    for (sheet_name, dancers) in data {
        let sheet_id = get_sheet_props(&sheet_name, &spreadsheet_id, key.clone())
            .await?
            .map(|SheetProps { sheet_id, .. }| sheet_id)
            .unwrap();
        let rows = spreadsheets::spreadsheet_rows!(
            dancers => SaxDancer {
                date: cell_data_from_naive_date(date.unwrap()),
                title: cell_data(
                    title
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                category: cell_data(
                    category
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                n: cell_data(
                    n
                        .map(CompactExtendedValue::Int),
                    None,
                ),
                place: cell_data(
                    place
                        .map(CompactExtendedValue::Int),
                    None,
                ),
                st_la_score: cell_data(
                    st_la_score
                        .map(CompactExtendedValue::Float),
                    None,
                ),
                st_score: cell_data(
                    st_score
                        .map(CompactExtendedValue::Float),
                    None,
                ),
                la_score: cell_data(
                    la_score
                        .map(CompactExtendedValue::Float),
                    None,
                ),
                ball: cell_data(
                    ball
                        .map(CompactExtendedValue::Float),
                    None,
                ),
                book_number: cell_data(
                    book_number
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                last_name: cell_data(
                    last_name
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                class: cell_data(
                    class
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                first_name: cell_data(
                    first_name
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                birth_day: cell_data(
                    birth_day
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                city: cell_data(
                    city
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                club: cell_data(
                    club
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                chief1_last_name: cell_data(
                    chief1_last_name
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                chief1_first_name: cell_data(
                    chief1_first_name
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                chief2_last_name: cell_data(
                    chief2_last_name
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                chief2_first_name: cell_data(
                    chief2_first_name
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                trener1_last_name: cell_data(
                    trener1_last_name
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                trener1_first_name: cell_data(
                    trener1_first_name
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                trener2_last_name: cell_data(
                    trener2_last_name
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                trener2_first_name: cell_data(
                    trener2_first_name
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
            }
        );
        requests.push(append_cells_request(rows, sheet_id));
    }
    let _: BatchUpdateSpreadsheetResponse =
        spreadsheets_batch_update(requests, &spreadsheet_id, key).await?;
    Ok(())
}

pub fn export_sax_sync(res: ExportSaxResult) -> Result<()> {
    if let Err(err) = res {
        error!("{}:{}: {err}", file!(), line!())
    }
    Ok(())
}

// ===================================================================

pub type ExportSax2Result = Result<()>;
use pasitos::sax2::Sax2Dancer;
pub async fn export_sax2(data: pasitos::sax2::Sax2Ret) -> ExportSax2Result {
    let service_account_secret_file = settings!(spreadsheet.service_account_secret_file).clone();
    let spreadsheet_id = settings!(spreadsheet.event_result_spreadsheet_id).clone();

    let key = yup_oauth2::read_service_account_key(&service_account_secret_file)
        .await
        .map_err(|err| anyhow!("read_service_account_key: {err}"))?;

    let mut requests = vec![];
    for (ret_key, by_clubs) in data {
        let sheet_id = get_sheet_props(&ret_key.to_string(), &spreadsheet_id, key.clone())
            .await?
            .map(|SheetProps { sheet_id, .. }| sheet_id)
            .unwrap();
        struct Row {
            name: String,
            chief1_last_name: Option<String>,
            trener1_last_name: Option<String>,
            first_name: String,
            last_name: String,
            book_number: Option<String>,
            birth_day: String,
            class: Option<String>,
            caterogys: usize,
            one: usize,
        }
        let mut rows = Vec::<Row>::new();
        for (
            sax2::Sax2Club {
                city,
                name,
                chief1_last_name,
                trener1_last_name,
                ..
            },
            by_dancer,
        ) in by_clubs
        {
            for (
                Sax2Dancer {
                    first_name,
                    last_name,
                    book_number,
                    birth_day,
                    class,
                },
                caterogys,
            ) in by_dancer
            {
                let caterogys = caterogys.len();
                rows.push(Row {
                    name: format!("{}::{}", name, city),
                    chief1_last_name: chief1_last_name.clone(),
                    trener1_last_name: trener1_last_name.clone(),
                    first_name: first_name.clone(),
                    last_name: last_name.clone(),
                    book_number: book_number.clone(),
                    birth_day: birth_day.clone(),
                    class: class.clone(),
                    caterogys,
                    one: 1,
                })
            }
        }
        let rows = spreadsheets::spreadsheet_rows!(
            rows => Row {
                name: cell_data(
                    Some(CompactExtendedValue::String(name.clone())),
                    None,
                ),
                chief1_last_name: cell_data(
                    chief1_last_name
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                trener1_last_name: cell_data(
                    trener1_last_name
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                last_name: cell_data(
                    Some(CompactExtendedValue::String(last_name.clone())),
                    None,
                ),
                first_name: cell_data(
                    Some(CompactExtendedValue::String(first_name.clone())),
                    None,
                ),
                book_number: cell_data(
                    book_number
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                birth_day: cell_data(
                    Some(CompactExtendedValue::String(birth_day.clone())),
                    None,
                ),
                class: cell_data(
                    class
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                caterogys: cell_data(
                    Some(CompactExtendedValue::Int(caterogys as i64)),
                    None,
                ),
                one: cell_data(
                    Some(CompactExtendedValue::Int(one as i64)),
                    None,
                ),
            }
        );
        requests.push(append_cells_request(rows, sheet_id));
    }
    let _: BatchUpdateSpreadsheetResponse =
        spreadsheets_batch_update(requests, &spreadsheet_id, key).await?;
    Ok(())
}

pub fn export_sax2_sync(res: ExportSax2Result) -> Result<()> {
    if let Err(err) = res {
        error!("{}:{}: {err}", file!(), line!())
    }
    Ok(())
}

// ===================================================================

pub type ExportResult = Result<()>;
pub async fn export(
    data: asto_db::ExportData,
    judges: bool,
    dancers: bool,
    clubs: bool,
) -> ExportResult {
    let service_account_secret_file = settings!(spreadsheet.service_account_secret_file).clone();
    let spreadsheet_id = settings!(spreadsheet.id).clone();

    let key = yup_oauth2::read_service_account_key(&service_account_secret_file)
        .await
        .map_err(|err| anyhow!("read_service_account_key: {err}"))?;

    let mut requests = vec![];
    if dancers {
        let dancers_sheet_name = settings!(spreadsheet.export_dancers_sheet_name).clone();
        let sheet_id = get_sheet_props(&dancers_sheet_name, &spreadsheet_id, key.clone())
            .await?
            .map(|SheetProps { sheet_id, .. }| sheet_id)
            .unwrap();
        let rows = spreadsheets::spreadsheet_rows!(
            data.dancerlar => DancerSpreadsheetRow {
                id: cell_data(id.map(|id| CompactExtendedValue::Int(id as i64)), None),
                external_id: cell_data(
                    external_id.map(|external_id| {
                        CompactExtendedValue::Int(external_id as i64)
                    }),
                    None,
                ),
                last_name: cell_data(
                    Some(CompactExtendedValue::String(last_name.clone())),
                    None,
                ),
                first_name: cell_data(
                    Some(CompactExtendedValue::String(first_name.clone())),
                    None,
                ),
                second_name: cell_data(
                    second_name
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                nick_name: cell_data(
                    nick_name
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                birth_date: birth_date
                    .map(cell_data_from_naive_date)
                    .unwrap_or_else(|| cell_data(None, None)),
                trainer_last_name: cell_data(
                    trainer_last_name
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                trainer_first_name: cell_data(
                    trainer_first_name
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                trainer_second_name: cell_data(
                    trainer_second_name
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                trainer_nick_name: cell_data(
                    trainer_nick_name
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                trainer2_last_name: cell_data(
                    trainer2_last_name
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                trainer2_first_name: cell_data(
                    trainer2_first_name
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                trainer2_second_name: cell_data(
                    trainer2_second_name
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                trainer2_nick_name: cell_data(
                    trainer2_nick_name
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                club: cell_data(
                    club.as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                citi: cell_data(
                    citi.as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                st_class: cell_data(
                    st_class
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                la_class: cell_data(
                    la_class
                        .as_ref()
                        .map(|s| CompactExtendedValue::String(s.clone())),
                    None,
                ),
                st_score: cell_data(Some(CompactExtendedValue::Float(st_score)), None),
                la_score: cell_data(Some(CompactExtendedValue::Float(la_score)), None),
                st_la_score: cell_data(Some(CompactExtendedValue::Float(st_la_score)), None),
                points: cell_data(Some(CompactExtendedValue::Float(points)), None),
                is_archive: cell_data(
                    Some(CompactExtendedValue::Int(
                        i64::from(is_archive.unwrap_or(false))
                    )),
                    None,
                ),
            }

        );
        requests.push(append_cells_request(rows, sheet_id));
    }
    if judges {
        let judges_sheet_name = settings!(spreadsheet.export_judges_sheet_name).clone();
        let sheet_id = get_sheet_props(&judges_sheet_name, &spreadsheet_id, key.clone())
            .await?
            .map(|SheetProps { sheet_id, .. }| sheet_id)
            .unwrap();
        let rows = spreadsheets::spreadsheet_rows!(data.judgelar => JudgeSpreadsheetRow {
            id: cell_data(id.map(|id| CompactExtendedValue::Int(id as i64)), None),
            external_id: cell_data(
                external_id.map(|external_id| {
                    CompactExtendedValue::Int(external_id as i64)
                }),
                None,
            ),
            last_name: cell_data(
                Some(CompactExtendedValue::String(last_name.clone())),
                None,
            ),
            first_name: cell_data(
                Some(CompactExtendedValue::String(first_name.clone())),
                None,
            ),
            second_name: cell_data(
                second_name
                    .as_ref()
                    .map(|s| CompactExtendedValue::String(s.clone())),
                None,
            ),
            nick_name: cell_data(
                nick_name
                    .as_ref()
                    .map(|s| CompactExtendedValue::String(s.clone())),
                None,
            ),
            categori: cell_data(
                categori
                    .as_ref()
                    .map(|s| CompactExtendedValue::String(s.clone())),
                None,
            ),
            assignment_date: assignment_date
                .map(cell_data_from_naive_date)
                .unwrap_or_else(|| cell_data(None, None)),
            club: cell_data(
                club.as_ref()
                    .map(|s| CompactExtendedValue::String(s.clone())),
                None,
            ),
            citi: cell_data(
                citi.as_ref()
                    .map(|s| CompactExtendedValue::String(s.clone())),
                None,
            ),
            number_of_participation_in_festivals: cell_data(
                number_of_participation_in_festivals
                    .map(|val| CompactExtendedValue::Int(val as i64)),
                None,
            ),
            is_archive: cell_data(
                Some(CompactExtendedValue::Int(
                    // if is_archive.unwrap_or(false) { 1 } else { 0 },
                    i64::from(is_archive.unwrap_or(false))
                )),
                None,
            ),
        });
        requests.push(append_cells_request(rows, sheet_id));
    }
    if clubs {
        let clubs_sheet_name = settings!(spreadsheet.export_clubs_sheet_name).clone();
        let sheet_id = get_sheet_props(&clubs_sheet_name, &spreadsheet_id, key.clone())
            .await?
            .map(|SheetProps { sheet_id, .. }| sheet_id)
            .unwrap();
        let rows = spreadsheet_rows!(data.clublar => ClubSpreadsheetRow {
            id: cell_data(id.map(|id| CompactExtendedValue::Int(id as i64)), None),
            club: cell_data(
                Some(CompactExtendedValue::String(club.clone())),
                None,
            ),
            citi: cell_data(
                Some(CompactExtendedValue::String(citi.clone())),
                None,
            ),
            chief_last_name: cell_data(
                chief_last_name
                    .as_ref()
                    .map(|s| CompactExtendedValue::String(s.clone())),
                None,
            ),
            chief_first_name: cell_data(
                chief_first_name
                    .as_ref()
                    .map(|s| CompactExtendedValue::String(s.clone())),
                None,
            ),
            chief_second_name: cell_data(
                chief_second_name
                    .as_ref()
                    .map(|s| CompactExtendedValue::String(s.clone())),
                None,
            ),
            chief_nick_name: cell_data(
                chief_nick_name
                    .as_ref()
                    .map(|s| CompactExtendedValue::String(s.clone())),
                None,
            ),
        });

        requests.push(append_cells_request(rows, sheet_id));
    }
    let _: BatchUpdateSpreadsheetResponse =
        spreadsheets_batch_update(requests, &spreadsheet_id, key).await?;
    Ok(())
}

pub fn export_sync(res: ExportResult) -> Result<()> {
    if let Err(err) = res {
        error!("{}:{}: {err}", file!(), line!())
    }
    Ok(())
}

// ===================================================================

pub type ImportResult = Result<ImportRet>;
pub async fn import(judges: bool, dancers: bool, clubs: bool) -> ImportResult {
    let spreadsheet_id = settings!(spreadsheet.id).clone();
    let service_account_secret_file = settings!(spreadsheet.service_account_secret_file).clone();
    let key = yup_oauth2::read_service_account_key(&service_account_secret_file)
        .await
        .map_err(|err| anyhow!("read_service_account_key: {err}"))?;
    let dancers_data_range = settings!(spreadsheet.import_dancers_data_range).clone();
    let judges_data_range = settings!(spreadsheet.import_judges_data_range).clone();
    let clubs_data_range = settings!(spreadsheet.import_clubs_data_range).clone();
    let mut rangelar = vec![];
    if judges {
        rangelar.push(judges_data_range.clone());
    }
    if clubs {
        rangelar.push(clubs_data_range.clone());
    }
    if dancers {
        rangelar.push(dancers_data_range.clone());
    }
    let ret = will_did!(trace => "get_rangelar_data", get_rangelar_data(
        rangelar,
        &spreadsheet_id,
        key.clone(),
    )
    .await)?;
    let mut clubs = vec![];
    let mut judges = vec![];
    let mut dancers = vec![];
    let dancers_sheet_name = dancers_data_range.split('!').next().unwrap();
    let judges_sheet_name = judges_data_range.split('!').next().unwrap();
    let clubs_sheet_name = clubs_data_range.split('!').next().unwrap();
    for (sheet_name, data_ranges) in ret {
        for (_data_range, range_data) in data_ranges {
            if sheet_name == clubs_sheet_name {
                clubs = spreadsheets::from_range_data!(range_data => ClubSpreadsheetRow { | cev |
                    0 id Option: cev.as_int().map(|i| i as i32),
                    0 club: cev.as_string(),
                    0 citi: cev.as_string(),

                    0 chief_last_name Option: cev.as_string(),
                    0 chief_first_name Option : cev.as_string(),
                    0 chief_second_name Option : cev.as_string(),
                    0 chief_nick_name Option : cev.as_string(),
                });
            } else if sheet_name == judges_sheet_name {
                judges = spreadsheets::from_range_data!(range_data => JudgeSpreadsheetRow { | cev |
                    0 id Option: cev.as_int().map(|i| i as i32),
                    0 external_id Option: cev.as_int().map(|i| i as i32),
                    0 last_name: cev.as_string(),
                    0 first_name: cev.as_string(),
                    0 second_name Option : cev.as_string(),
                    0 nick_name Option : cev.as_string(),
                    0 categori Option: cev.as_string(),
                    0 assignment_date Option : cev.as_f64().map(spreahsheet_number_value_to_naive_date),
                    0 club Option: cev.as_string(),
                    0 citi Option: cev.as_string(),
                    0 number_of_participation_in_festivals Option: cev.as_int().and_then(|i| {
                        let ret: Option<i32> = i.try_into().ok();
                        ret
                    }),
                    0 is_archive Option: Some(cev.as_int().map(|i| i > 0).unwrap_or(false)),
                });
            } else if sheet_name == dancers_sheet_name {
                dancers = spreadsheets::from_range_data!(range_data => DancerSpreadsheetRow { | cev |
                    0 id Option: cev.as_int().map(|i| i as i32),
                    0 external_id Option: cev.as_int().map(|i| i as i32),
                    0 last_name: cev.as_string(),
                    0 first_name: cev.as_string(),
                    0 second_name Option : cev.as_string(),
                    0 nick_name Option : cev.as_string(),
                    0 birth_date Option : cev.as_f64().map(spreahsheet_number_value_to_naive_date),
                    0 trainer_last_name Option: cev.as_string(),
                    0 trainer_first_name Option: cev.as_string(),
                    0 trainer_second_name Option: cev.as_string(),
                    0 trainer_nick_name Option: cev.as_string(),
                    0 trainer2_last_name Option: cev.as_string(),
                    0 trainer2_first_name Option: cev.as_string(),
                    0 trainer2_second_name Option: cev.as_string(),
                    0 trainer2_nick_name Option: cev.as_string(),
                    0 club Option: cev.as_string(),
                    0 citi Option: cev.as_string(),
                    0 st_class Option: cev.as_string(),
                    0 la_class Option: cev.as_string(),
                    0 st_score Option { .unwrap_or_default() }: cev.as_f64(),
                    0 la_score Option { .unwrap_or_default() }: cev.as_f64(),
                    0 st_la_score Option { .unwrap_or_default() }: cev.as_f64(),
                    0 points Option { .unwrap_or_default() }: cev.as_f64(),
                    0 is_archive Option: Some(cev.as_int().map(|i| i > 0).unwrap_or(false)),
                });
            } else {
                unreachable!();
            }
        }
    }
    Ok(ImportRet {
        clubs,
        judges,
        dancers,
    })
}

pub fn import_sync(res: ImportResult) -> Result<()> {
    match res {
        Err(err) => {
            error!("{}:{}: {err}", file!(), line!())
        }
        Ok(data) => {
            pasitos!(db push_back Import { data });
        }
    }
    Ok(())
}

// ===================================================================
