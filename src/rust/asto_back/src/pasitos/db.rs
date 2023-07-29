use super::*;
use crate::server::{
    common::{send_response_message, TxHandle},
    ResponseMessage,
};
use pool_db::*;

pub type GetInitDataResult = Result<InitData>;
pub async fn get_init_data(key: InitDataKey) -> GetInitDataResult {
    let settings = settings!(db).clone();
    let op_mode = *OP_MODE.read().unwrap();
    let pool = get(PoolDb::Pg, settings, op_mode).await?;
    asto_db::get_init_data(key, as_ref!(Pg pool)).await
}

pub fn get_init_data_sync(res: GetInitDataResult, tx: TxHandle) -> Result<()> {
    send_response_message(ResponseMessage::InitData(Box::new(res)), tx);
    Ok(())
}

// ============================================================

pub type ImportEventResult = Result<()>;
pub async fn import_event(
    data: ImportEventRet,
    opts: pasitos::sax3::Sax3Opts,
) -> ImportEventResult {
    let mut settings = settings!(db).clone();
    let dry_run = opts.dry_run;
    if let Some(db_url) = opts.db_url {
        settings.url = db_url;
    }
    let op_mode = *OP_MODE.read().unwrap();
    let pool = get(PoolDb::Pg, settings, op_mode).await?;
    asto_db::import_event(data, dry_run, as_ref!(Pg pool)).await?;
    Ok(())
}

pub fn import_event_sync(res: ImportEventResult) -> Result<()> {
    if let Err(err) = res {
        error!("{}:{}: {err}", file!(), line!());
    }
    Ok(())
}

// ============================================================

pub type ImportResult = Result<()>;
pub async fn import(data: ImportRet) -> ImportResult {
    let settings = settings!(db).clone();
    let op_mode = *OP_MODE.read().unwrap();
    let pool = get(PoolDb::Pg, settings, op_mode).await?;
    asto_db::import(data, as_ref!(Pg pool)).await?;
    Ok(())
}

pub fn import_sync(res: ImportResult) -> Result<()> {
    if let Err(err) = res {
        error!("{}:{}: {err}", file!(), line!());
    }
    Ok(())
}

// ============================================================

pub type ExportResult = Result<asto_db::ExportData>;
pub async fn export() -> ExportResult {
    let settings = settings!(db).clone();
    let op_mode = *OP_MODE.read().unwrap();
    let pool = get(PoolDb::Pg, settings, op_mode).await?;
    asto_db::export(op_mode::OpMode::Prod, as_ref!(Pg pool)).await
}

pub fn export_sync(res: ExportResult, judges: bool, dancers: bool, clubs: bool) -> Result<()> {
    match res {
        Err(err) => {
            error!("{}:{}: {err}", file!(), line!());
        }
        Ok(data) => {
            pasitos!(spreadsheet push_back Export { data, judges, dancers, clubs });
        }
    }
    Ok(())
}

// ============================================================

pub type ExportDancersForAntonResult = Result<Vec<ForAntonDancerRow>>;
pub async fn export_dancers_for_anton() -> ExportDancersForAntonResult {
    let settings = settings!(db).clone();
    let op_mode = *OP_MODE.read().unwrap();
    let pool = get(PoolDb::Pg, settings, op_mode).await?;
    asto_db::export_dancers_for_anton(op_mode::OpMode::Prod, as_ref!(Pg pool)).await
}

pub fn export_dancers_for_anton_sync(res: ExportDancersForAntonResult) -> Result<()> {
    match res {
        Err(err) => {
            error!("{}:{}: {err}", file!(), line!());
        }
        Ok(mut rows) => {
            for row in rows.iter_mut() {
                if let Some((_, st_class_upgraded, la_class_upgraded)) =
                    class_upgrade(row.external_id)
                {
                    row.st_class = class_as_string(st_class_upgraded).map(|s| s.to_owned());
                    row.la_class = class_as_string(la_class_upgraded).map(|s| s.to_owned());
                }
            }
            pasitos!(spreadsheet push_back ExportDancersForAnton { rows });
        }
    }
    Ok(())
}

// ============================================================

pub type CommitResult = Result<Modal>;
pub async fn commit(modal: Modal) -> CommitResult {
    let settings = settings!(db).clone();
    let op_mode = *OP_MODE.read().unwrap();
    let pool = get(PoolDb::Pg, settings, op_mode).await?;
    asto_db::commit(modal, op_mode, as_ref!(Pg pool)).await
}

pub fn commit_sync(res: CommitResult, tx: TxHandle) -> Result<()> {
    send_response_message(ResponseMessage::Commit(res), tx);
    Ok(())
}
