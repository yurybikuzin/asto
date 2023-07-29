#[allow(unused_imports)]
use anyhow::{anyhow, bail, Context, Error, Result};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub use asto_common::*;
pub use pool_db::*;

pub async fn get_init_data(key: InitDataKey, pool: &PoolPg) -> Result<InitData> {
    #[derive(sqlx::FromRow)]
    struct Ret {
        init_data: serde_json::Value,
    }
    let ret = sqlx::query_as!(
        Ret,
        r#" select get_init_data($1::smallint, null::timestamptz) "init_data!" "#,
        key.op_mode as i16,
    )
    .fetch_one(pool)
    .await;
    ret.map_err(|err| anyhow!("{err}"))
        .and_then(|ret| {
            serde_json::from_value::<InitData>(ret.init_data).map_err(|err| {
                error!("{}:{}: {err}", file!(), line!());
                anyhow!("{err}")
            })
        })
        .map(|mut ret| {
            let dt = chrono::Utc::now().naive_local();
            ret.today = Some(dt.date());
            ret
        })
}

pub struct ExportData {
    pub clublar: Vec<ClubSpreadsheetRow>,
    pub judgelar: Vec<JudgeSpreadsheetRow>,
    pub dancerlar: Vec<DancerSpreadsheetRow>,
}
pub async fn export(op_mode: op_mode::OpMode, pool: &PoolPg) -> Result<ExportData> {
    let judgelar = {
        #[derive(sqlx::FromRow)]
        struct Ret {
            id: Option<i32>,
            external_id: Option<i32>,
            last_name: Option<String>,
            first_name: Option<String>,
            second_name: Option<String>,
            nick_name: Option<String>,
            categori: Option<String>,
            assignment_date: Option<chrono::NaiveDate>,
            club: Option<String>,
            citi: Option<String>,
            number_of_participation_in_festivals: Option<i32>,
            is_archive: Option<bool>,
        }
        let ret = sqlx::query_as!(
            Ret,
            r#" select * from export_judgelar($1::smallint) "#,
            op_mode as i16,
        )
        .fetch_all(pool)
        .await;
        ret.map_err(|err| anyhow!("{err}")).map(|ret| {
            ret.into_iter()
                .map(
                    |Ret {
                         id,
                         external_id,
                         last_name,
                         first_name,
                         second_name,
                         nick_name,
                         categori,
                         assignment_date,
                         club,
                         citi,
                         number_of_participation_in_festivals,
                         is_archive,
                     }| JudgeSpreadsheetRow {
                        id,
                        external_id: external_id.and_then(|i| (i >= 1).then_some(i)),
                        last_name: last_name
                            .and_then(|s| (!s.is_empty()).then_some(s))
                            .unwrap(),
                        first_name: first_name
                            .and_then(|s| (!s.is_empty()).then_some(s))
                            .unwrap(),
                        second_name: second_name.and_then(|s| (!s.is_empty()).then_some(s)),
                        nick_name: nick_name.and_then(|s| (!s.is_empty()).then_some(s)),
                        categori: categori.and_then(|s| (!s.is_empty()).then_some(s)),
                        assignment_date: assignment_date.and_then(|_date| {
                            (chrono::NaiveDate::from_ymd_opt(1900, 1, 1).unwrap() != _date)
                                .then_some(_date)
                        }),
                        club: club.and_then(|club| (!club.is_empty()).then_some(club)),
                        citi: citi.and_then(|citi| (!citi.is_empty()).then_some(citi)),
                        number_of_participation_in_festivals: number_of_participation_in_festivals
                            .and_then(|i| (i >= 0).then_some(i)),
                        is_archive,
                    },
                )
                .collect::<Vec<_>>()
        })
    }?;
    let clublar = {
        #[derive(sqlx::FromRow)]
        struct Ret {
            id: Option<i32>,
            club: Option<String>,
            citi: Option<String>,
            chief_last_name: Option<String>,
            chief_first_name: Option<String>,
            chief_second_name: Option<String>,
            chief_nick_name: Option<String>,
        }
        let ret = sqlx::query_as!(
            Ret,
            r#" select * from export_clublar($1::smallint) "#,
            op_mode as i16,
        )
        .fetch_all(pool)
        .await;
        ret.map_err(|err| anyhow!("{err}")).map(|ret| {
            ret.into_iter()
                .map(
                    |Ret {
                         id,
                         club,
                         citi,
                         chief_last_name,
                         chief_first_name,
                         chief_second_name,
                         chief_nick_name,
                     }| ClubSpreadsheetRow {
                        id,
                        club: club
                            .and_then(|club| (!club.is_empty()).then_some(club))
                            .unwrap(),
                        citi: citi
                            .and_then(|citi| (!citi.is_empty()).then_some(citi))
                            .unwrap(),
                        chief_last_name: chief_last_name.and_then(|s| (!s.is_empty()).then_some(s)),
                        chief_first_name: chief_first_name
                            .and_then(|s| (!s.is_empty()).then_some(s)),
                        chief_second_name: chief_second_name
                            .and_then(|s| (!s.is_empty()).then_some(s)),
                        chief_nick_name: chief_nick_name.and_then(|s| (!s.is_empty()).then_some(s)),
                    },
                )
                .collect::<Vec<_>>()
        })
    }?;

    let dancerlar = {
        #[derive(sqlx::FromRow)]
        struct Ret {
            id: Option<i32>,
            external_id: Option<i32>,
            last_name: Option<String>,
            first_name: Option<String>,
            second_name: Option<String>,
            nick_name: Option<String>,
            birth_date: Option<chrono::NaiveDate>,
            trainer_last_name: Option<String>,
            trainer_first_name: Option<String>,
            trainer_second_name: Option<String>,
            trainer_nick_name: Option<String>,
            trainer2_last_name: Option<String>,
            trainer2_first_name: Option<String>,
            trainer2_second_name: Option<String>,
            trainer2_nick_name: Option<String>,
            club: Option<String>,
            citi: Option<String>,
            st_class: Option<String>,
            la_class: Option<String>,
            st_score: Option<f64>,
            la_score: Option<f64>,
            st_la_score: Option<f64>,
            points: Option<f64>,
            is_archive: Option<bool>,
        }
        let ret = sqlx::query_as!(
            Ret,
            r#" select * from export_dancerlar($1::smallint) "#,
            op_mode as i16,
        )
        .fetch_all(pool)
        .await;
        ret.map_err(|err| anyhow!("{err}")).map(|ret| {
            ret.into_iter()
                .map(
                    |Ret {
                         id,
                         external_id,
                         last_name,
                         first_name,
                         second_name,
                         nick_name,
                         birth_date,
                         trainer_last_name,
                         trainer_first_name,
                         trainer_second_name,
                         trainer_nick_name,
                         trainer2_last_name,
                         trainer2_first_name,
                         trainer2_second_name,
                         trainer2_nick_name,
                         club,
                         citi,
                         st_class,
                         la_class,
                         st_score,
                         la_score,
                         st_la_score,
                         points,
                         is_archive,
                     }| DancerSpreadsheetRow {
                        id,
                        external_id: external_id.and_then(|i| (i >= 1).then_some(i)),
                        last_name: last_name
                            .and_then(|s| (!s.is_empty()).then_some(s))
                            .unwrap(),
                        first_name: first_name
                            .and_then(|s| (!s.is_empty()).then_some(s))
                            .unwrap(),
                        second_name: second_name.and_then(|s| (!s.is_empty()).then_some(s)),
                        nick_name: nick_name.and_then(|s| (!s.is_empty()).then_some(s)),
                        birth_date: birth_date.and_then(|date| {
                            (chrono::NaiveDate::from_ymd_opt(1900, 1, 1).unwrap() != date)
                                .then_some(date)
                        }),
                        club: club.and_then(|s| (!s.is_empty()).then_some(s)),
                        trainer_last_name: trainer_last_name
                            .and_then(|s| (!s.is_empty()).then_some(s)),
                        trainer_first_name: trainer_first_name
                            .and_then(|s| (!s.is_empty()).then_some(s)),
                        trainer_second_name: trainer_second_name
                            .and_then(|s| (!s.is_empty()).then_some(s)),
                        trainer_nick_name: trainer_nick_name
                            .and_then(|s| (!s.is_empty()).then_some(s)),
                        trainer2_last_name: trainer2_last_name
                            .and_then(|s| (!s.is_empty()).then_some(s)),
                        trainer2_first_name: trainer2_first_name
                            .and_then(|s| (!s.is_empty()).then_some(s)),
                        trainer2_second_name: trainer2_second_name
                            .and_then(|s| (!s.is_empty()).then_some(s)),
                        trainer2_nick_name: trainer2_nick_name
                            .and_then(|s| (!s.is_empty()).then_some(s)),
                        citi: citi.and_then(|s| (!s.is_empty()).then_some(s)),

                        st_class: st_class.and_then(|s| (!s.is_empty()).then_some(s)),
                        la_class: la_class.and_then(|s| (!s.is_empty()).then_some(s)),
                        st_score: st_score.unwrap_or(0f64),
                        la_score: la_score.unwrap_or(0f64),
                        st_la_score: st_la_score.unwrap_or(0f64),
                        points: points.unwrap_or(0f64),
                        is_archive,
                    },
                )
                .collect::<Vec<_>>()
        })
    }?;
    Ok(ExportData {
        judgelar,
        clublar,
        dancerlar,
    })
}

pub async fn export_dancers_for_anton(
    op_mode: op_mode::OpMode,
    pool: &PoolPg,
) -> Result<Vec<ForAntonDancerRow>> {
    type Ret = ForAntonDancerRow;
    sqlx::query_as!(
        Ret,
        r#" select * from export_dancerlar_for_anton($1::smallint) "#,
        op_mode as i16,
    )
    .fetch_all(pool)
    .await
    .map_err(|err| anyhow!("{err}"))
}

use chrono::NaiveDate;
use std::collections::HashSet;

pub async fn import_event(data: ImportEventRet, dry_run: bool, pool: &PoolPg) -> Result<()> {
    let op_mode: i16 = -1;
    #[derive(Hash, PartialEq, Eq)]
    struct Event<'a> {
        date: &'a NaiveDate,
        title: &'a String,
    }
    let eventlar = data
        .iter()
        .map(|ImportEventRow { date, title, .. }| Event { date, title })
        .collect::<HashSet<_>>();
    if !dry_run {
        for Event { date, title } in eventlar {
            sqlx::query!(
                r#"
                delete
                from event_resultlar
                where op_mode = $1
                    and event = add_event($1::smallint
                        , null::int
                        , $2::date
                        , $3::text
                    )
            "#,
                op_mode,
                date,
                title
            )
            .execute(pool)
            .await
            .map_err(|err| anyhow!("add_event: {err}"))?;
        }
    }
    #[derive(Hash, PartialEq, Eq, Clone)]
    struct RestoredFor {
        last_name: String,
        first_name: String,
        club: String,
        city: String,
    }
    use std::collections::HashMap;
    let mut restored: HashMap<RestoredFor, i32> = HashMap::new();
    let mut non_registered: HashSet<RestoredFor> = HashSet::new();
    let mut participantlar: HashSet<RestoredFor> = HashSet::new();
    let mut book_numberlar: std::collections::BTreeSet<i32> = std::collections::BTreeSet::new();
    for ImportEventRow {
        date,
        title,
        category,
        couple_number,
        st_score,
        la_score,
        st_la_score,
        points,
        external_id,
        first_name,
        last_name,
        club,
        city,
        ..
    } in data
    {
        let mut external_id = if external_id
            .map(|external_id| (5500001..=5599999).contains(&external_id))
            .unwrap_or(false)
        {
            external_id
        } else {
            None
        };
        let i = RestoredFor {
            last_name: last_name.clone(),
            first_name: first_name.clone(),
            club: club.clone(),
            city: city.clone(),
        };
        participantlar.insert(i.clone());
        let i_opt = if external_id.is_none() {
            if let Some(value) = restored.get(&i) {
                book_numberlar.insert(*value);
                external_id = Some(*value);
                None
            } else {
                Some(i)
            }
        } else {
            book_numberlar.insert(external_id.unwrap());
            None
        };
        #[derive(sqlx::FromRow)]
        pub struct Ret {
            pub id: i32,
        }
        let Ret { id } = sqlx::query_as!(
            Ret,
            r#"
                select add_event_result($1::smallint 
                    , add_event($1::smallint
                        , null::int
                        , $2::date
                        , $3::text
                    )
                    , add_text($4::text)
                    , $5::int
                    , $6::smallint
                    , $7::smallint
                    , $8::smallint
                    , $9::smallint
                    , $10::smallint
                    , $11::text
                    , $12::text
                    , $13::text
                    , $14::text
                    , $15::boolean
                ) "id!"
            "#,
            op_mode,
            date,
            title,
            category,
            external_id,
            couple_number,
            st_score,
            la_score,
            st_la_score,
            points,
            first_name,
            last_name,
            club,
            city,
            dry_run,
        )
        .fetch_one(pool)
        .await?;
        if external_id.is_none() {
            if id != 0 {
                book_numberlar.insert(id);
                restored.insert(i_opt.unwrap(), id);
                debug!(
                    "restored external_id {id} for {last_name:?} {first_name:?}, {club:?}, {city:?}"
                );
            } else {
                non_registered.insert(i_opt.unwrap());
            }
        }
    }
    debug!(
        "book_numberlar: {}",
        book_numberlar
            .into_iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
    debug!(
        "non_registered: {}/{}",
        non_registered.len(),
        participantlar.len()
    );
    Ok(())
}

pub async fn import(data: ImportRet, pool: &PoolPg) -> Result<()> {
    for ClubSpreadsheetRow {
        id,
        club,
        citi,
        chief_last_name,
        chief_first_name,
        chief_second_name,
        chief_nick_name,
    } in data.clubs.into_iter()
    {
        #[derive(sqlx::FromRow)]
        pub struct Ret {
            pub id: i32,
        }
        let Ret { id: _ } = sqlx::query_as!(
                Ret,
                r#"
                    select add_club($1::smallint
                        , $2::int
                        , $3::text
                        , add_citi($1::smallint
                            , null::int
                            , $4::text
                        )
                        , add_person($1::smallint
                            , null::int
                            , $5::text
                            , $6::text
                            , $7::text
                            , $8::text
                            , null::date
                        )
                    ) "id!"
                "#,
                -1i16,
                id,
                club, 
                citi, 
                chief_last_name.as_deref().unwrap_or_default(),
                chief_first_name.as_deref().unwrap_or_default(),
                chief_second_name.as_deref().unwrap_or_default(),
                chief_nick_name.as_deref().unwrap_or_default(),
            )
            .fetch_one(pool)
            .await
            .map_err(|err| anyhow!("id={id:?}, club={club:?}, citi={citi:?}, chief_last_name={chief_last_name:?}, chief_first_name={chief_first_name:?}, chief_second_name={chief_second_name:?}, chief_nick_name={chief_nick_name:?}: {err}"))?;
    }
    for JudgeSpreadsheetRow {
        id,
        external_id,
        last_name,
        first_name,
        second_name,
        nick_name,
        categori,
        assignment_date,
        club,
        citi,
        number_of_participation_in_festivals,
        is_archive,
    } in data.judges.into_iter()
    {
        #[derive(sqlx::FromRow)]
        pub struct Ret {
            pub trainer_id: i32,
        }
        let Ret { trainer_id: _ } = sqlx::query_as!(
                Ret,
                r#"
                    select add_judge( $1
                        , $2::int
                        , add_person($1
                            , null::int
                            , $3::text
                            , $4::text
                            , $5::text
                            , $6::text
                            , null::date
                        )::int
                        , $7::int
                        , (select id from categorilar where value = $8)::smallint
                        , $9::date
                        , add_club($1, null::int
                            , $10
                            , add_citi($1::smallint
                                , null::int
                                , $11::text
                            )
                            , null::int
                        )::int
                        , $12::int
                        , $13::bool
                    ) "trainer_id!"
                "#,
                -1i16,
                id,
                last_name, 
                first_name,
                second_name.as_deref().unwrap_or_default(),
                nick_name.as_deref().unwrap_or_default(),
                external_id.unwrap_or_default(),
                categori.as_deref().unwrap_or_default(),
                assignment_date.unwrap_or_else(|| chrono::NaiveDate::from_ymd_opt(1900, 1, 1).unwrap()),
                club.as_deref().unwrap_or_default(),
                citi.as_deref().unwrap_or_default(),
                number_of_participation_in_festivals.unwrap_or(-1),
                is_archive
            )
            .fetch_one(pool)
            .await
            .map_err(|err| anyhow!("id={id:?}, last_name={last_name:?}, first_name={first_name:?}, second_name={second_name:?}, nick_name={nick_name:?}, external_id={external_id:?}, categori={categori:?}, assignment_date={assignment_date:?}, club={club:?}, citi={citi:?}, number_of_participation_in_festivals={number_of_participation_in_festivals:?}: {err}"))?;
    }
    for DancerSpreadsheetRow {
        id,
        external_id,
        last_name,
        first_name,
        second_name,
        nick_name,
        birth_date,
        //
        trainer_last_name,
        trainer_first_name,
        trainer_second_name,
        trainer_nick_name,
        //
        trainer2_last_name,
        trainer2_first_name,
        trainer2_second_name,
        trainer2_nick_name,
        //
        club,
        citi,
        st_class,
        la_class,
        st_score,
        la_score,
        st_la_score,
        points,
        is_archive,
    } in data.dancers.into_iter()
    {
        #[derive(sqlx::FromRow)]
        pub struct Ret {
            pub id: i32,
        }
        let Ret { id: _ } = sqlx::query_as!(
                Ret,
                r#"
                    select add_dancer( $1, $25::int
                        , add_person($1, null::int
                            , $2::text
                            , $3::text
                            , $4::text
                            , $5::text
                            , $6::date
                        )::int
                        , $7::int
                        , add_club($1, null::int
                            , $8
                            , add_citi($1::smallint
                                , null::int
                                , $9::text
                            )::int
                            , null::int
                        )::int
                        , add_trainer($1, null::int
                            , $10
                            , $11
                            , $12
                            , $13
                            , add_club($1, null::int
                                , $8::text
                                , add_citi($1::smallint
                                    , null::int
                                    , $9::text
                                )::int
                                , null::int
                            )::int
                        )
                        , add_trainer($1, null::int
                            , $14
                            , $15
                            , $16
                            , $17
                            , add_club($1, null::int
                                , $8::text
                                , add_citi($1::smallint
                                    , null::int
                                    , $9::text
                                )::int
                                , null::int
                            )::int
                        )
                        , (select id from classlar where value = $18)
                        , (select id from classlar where value = $19)
                        , $20::int
                        , $21::int
                        , $22::int
                        , $23::int
                        , $24::bool
                    ) "id!"
                "#,
                -1i16,
                last_name, 
                first_name, 
                second_name.as_deref().unwrap_or_default(),
                nick_name.as_deref().unwrap_or_default(),
                birth_date.unwrap_or_else(|| chrono::NaiveDate::from_ymd_opt(1900, 1, 1).unwrap()),
                external_id.unwrap_or_default(),
                club.as_deref().unwrap_or_default(),
                citi.as_deref().unwrap_or_default(),
                trainer_last_name.as_deref().unwrap_or_default(),
                trainer_first_name.as_deref().unwrap_or_default(),
                trainer_second_name.as_deref().unwrap_or_default(),
                trainer_nick_name.as_deref().unwrap_or_default(),
                trainer2_last_name.as_deref().unwrap_or_default(),
                trainer2_first_name.as_deref().unwrap_or_default(),
                trainer2_second_name.as_deref().unwrap_or_default(),
                trainer2_nick_name.as_deref().unwrap_or_default(),
                st_class.as_deref().unwrap_or_default(),
                la_class.as_deref().unwrap_or_default(),
                (st_score * 4f64).round() as i32,
                (la_score * 4f64).round() as i32,
                (st_la_score * 4f64).round() as i32,
                (points * 10f64).round() as i32,
                is_archive,
                id,
            )
            .fetch_one(pool)
            .await
            .map_err(|err| anyhow!("last_name={last_name:?}, first_name={first_name:?}, second_name={second_name:?}, nick_name={nick_name:?}, external_id={external_id:?}, birth_date={birth_date:?}, club={club:?}, citi={citi:?}, trainer_last_name={trainer_last_name:?}, trainer_first_name={trainer_first_name:?}, trainer_second_name={trainer_second_name:?}, trainer_nick_name={trainer_nick_name:?}, trainer2_last_name={trainer2_last_name:?}, trainer2_first_name={trainer2_first_name:?}, trainer2_second_name={trainer2_second_name:?}, trainer2_nick_name={trainer2_nick_name:?}, st_class={st_class:?}, la_class={la_class:?}, st_score={st_score}, la_score={la_score}, st_la_score={st_la_score}, points={points}, is_archive: {is_archive:?}: {err}"))?;
    }
    sqlx::query!(r#" call determine_gender() "#,)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn import_dancers(_rows: Vec<DancerRow>, _pool: &PoolPg) -> Result<()> {
    todo!();
}

pub async fn commit(_modal: Modal, _op_mode: op_mode::OpMode, _pool: &PoolPg) -> Result<Modal> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use common_macros2::*;

    declare_env_settings! {
        database_url: String,
        database_connection_max_count: u32,
        database_connection_live_for_secs: u64,
    }

    #[tokio::test]
    async fn get_init_data() -> Result<()> {
        dotenv::dotenv().context("file .env")?;
        let _ = pretty_env_logger::try_init_timed();
        let op_mode = op_mode::OpMode::Local;

        let settings = PoolDbSettings {
            url: env_settings!(database_url).clone(),
            local_url: None,
            connection_max_count: env_settings!(database_connection_max_count),
            live_for_secs: Some(env_settings!(database_connection_live_for_secs)),
        };
        let pool = will_did!(trace => "get_pool", pool_db::get(
            PoolDb::Pg, settings, op_mode
        )
        .await)?;

        let init_data = common_macros2::will_did!(trace => "get_init_data", super::get_init_data( InitDataKey { op_mode }, pool_db::as_ref!(Pg pool)).await?);

        Ok(())
    }
}
