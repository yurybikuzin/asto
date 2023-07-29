use super::*;

use futures::StreamExt;
use server::{
    common::{
        request_message, request_message_sync, start_receive, RequestMessageResult, RxPasitos,
        TxHandle,
    },
    ws::common::{ping_ws_connections, start_ping_ws_connections},
};

// ==================================================
// ==================================================
// You have to customize:
// - here
pub mod data;
pub mod db;
pub mod sax;
pub mod sax2;

pub mod sax3;
pub mod spreadsheet;
// ==================================================
// ==================================================

pasitos!(fut_queue, run_for;
    init {
        let start = std::time::Instant::now();
        let opt = (*OPT.read().unwrap()).clone().unwrap();
        match opt.cmd.as_ref().unwrap() {
            Command::Server {..} => {
                start_receive();
                start_ping_ws_connections();
            },
            // ==================================================
            // ==================================================
            // You have to customize:
            // - here
            Command::ExportDancersForAnton {} => {
                pasitos!(db push_back ExportDancersForAnton { });
            }
            Command::Export {
                judges,
                dancers,
                clubs,
            } => {
                let need_all = !(*judges || *dancers || *clubs);
                if need_all {
                    warn!("no -j, -d, -c specified, all of them will be meant");
                }
                pasitos!(db push_back Export {
                    judges: *judges || need_all,
                    dancers: *dancers || need_all,
                    clubs: *clubs || need_all,
                });
            }
            Command::Import {
                judges,
                dancers,
                clubs,
            } => {
                let need_all = !(*judges || *dancers || *clubs);
                if need_all {
                    warn!("no -j, -d, -c specified, all of them will be meant");
                }
                pasitos!(spreadsheet push_back Import {
                    judges: *judges || need_all,
                    dancers: *dancers || need_all,
                    clubs: *clubs || need_all,
                });
            }
            Command::Sax {
                file_pathlar,
            } => {
                let data = pasitos::sax::sax(file_pathlar)?;
                pasitos!(spreadsheet push_back ExportSax {
                    data
                });
            }
            Command::Sax2 {
                file_pathlar,
            } => {
                let data = pasitos::sax2::sax2(file_pathlar)?;
                pasitos!(spreadsheet push_back ExportSax2 {
                    data
                });
            }
            Command::Sax3 {
                file_pathlar,
                proto,
                summary,
                database,
                dry_run,
                judges,
                db_url,
            } => {
                let data = pasitos::sax3::sax3(file_pathlar)?;
                pasitos!(spreadsheet push_back ExportSax3 {
                    data,
                    opts: pasitos::sax3::Sax3Opts {
                        proto: *proto,
                        summary: *summary,
                        database: *database,
                        dry_run: *dry_run,
                        judges: *judges,
                        db_url: db_url.clone(),
                    },
                });
            }
            // ==================================================
            // ==================================================
        }
    }
    on_complete {
        info!(
            "{}, complete",
            arrange_millis::get(std::time::Instant::now().duration_since(start).as_millis()),
        );
        return Ok(());
    }
    on_next_end {
    }

    // >>> server::ws
    demoras {
        demora WsPing({
            duration: tokio::time::Duration,
        }) {
            pasitos!(ws push_back Ping { duration });
        }
    }

    pasos ws {
        max_at_once: 1;
        paso Ping({
            duration: tokio::time::Duration,
        }) -> ({
        }) {
            ping_ws_connections(duration).await;
        } => sync {
            let duration = tokio::time::Duration::from_secs(settings!(ping_interval_secs));
            pasitos!(delay WsPing {duration} for duration);
        }
    }
    // <<< server::ws

    // >>> server
    pasos receive {
        max_at_once: 1;
        paso RequestMessage({
            rx: RxPasitos,
        }) -> ({
            res: RequestMessageResult,
        }) {
            let res = request_message(rx).await;
        } => sync {
            request_message_sync(res)?;
        }
    }
    // <<< server

    // ==================================================
    // ==================================================
    // You have to customize:
    // - here

    pasos data {
        max_at_once: settings!(data.max_at_once);

        paso Fest({
            fest: String,
        }) -> ( {
            res: pasitos::data::FestResult,
            fest: String,
        }) {
            let res = pasitos::data::fest(&fest).await;
        } => sync {
            pasitos::data::fest_sync(res, fest)?;
        }

        paso FestIndex({
            tx: TxHandle,
        }) -> ( {
            res: pasitos::data::FestIndexResult,
            tx: TxHandle,
        }) {
            let res = pasitos::data::fest_index().await;
        } => sync {
            pasitos::data::fest_index_sync(res, tx)?;
        }

        paso FestJudges({
            tx: TxHandle,
        }) -> ( {
            res: pasitos::data::FestJudgesResult,
            tx: TxHandle,
        }) {
            let res = pasitos::data::fest_judges().await;
        } => sync {
            pasitos::data::fest_judges_sync(res, tx)?;
        }
    }

    pasos db {
        max_at_once: settings!(db.connection_max_count) as usize;

        paso ImportEvent({
            data: ImportEventRet,
            opts: pasitos::sax3::Sax3Opts,
        }) -> ( {
            res: pasitos::db::ImportEventResult,
        }) {
            let res = pasitos::db::import_event(data, opts).await;
        } => sync {
            pasitos::db::import_event_sync(res)?;
        }

        paso GetInitData({
            tx: TxHandle,
            key: InitDataKey,
        }) -> ( {
            res: pasitos::db::GetInitDataResult,
            tx: TxHandle,
        }) {
            let res = pasitos::db::get_init_data(key).await;
        } => sync {
            pasitos::db::get_init_data_sync(res, tx)?;
        }

        paso Import({
            data: ImportRet,
        }) -> ( {
            res: pasitos::db::ImportResult,
        }) {
            let res = pasitos::db::import(data).await;
        } => sync {
            pasitos::db::import_sync(res)?;
        }

        paso Export({
            judges: bool,
            dancers: bool,
            clubs: bool,
        }) -> ( {
            res: pasitos::db::ExportResult,
            judges: bool,
            dancers: bool,
            clubs: bool,
        }) {
            let res = pasitos::db::export().await;
        } => sync {
            pasitos::db::export_sync(res, judges, dancers, clubs)?;
        }

        paso ExportDancersForAnton({
        }) -> ( {
            res: pasitos::db::ExportDancersForAntonResult,
        }) {
            let res = pasitos::db::export_dancers_for_anton().await;
        } => sync {
            pasitos::db::export_dancers_for_anton_sync(res)?;
        }

        paso Commit({
            tx: TxHandle,
            modal: Modal,
        }) -> ( {
            res: pasitos::db::CommitResult,
            tx: TxHandle,
        }) {
            let res = pasitos::db::commit(modal).await;
        } => sync {
            pasitos::db::commit_sync(res, tx)?;
        }
    }

    pasos spreadsheet {
        max_at_once: 1;

        paso ExportSax({
            data: pasitos::sax::SaxRet,
        }) -> ( {
            res: pasitos::spreadsheet::ExportSaxResult,
        }) {
            let res = pasitos::spreadsheet::export_sax(data).await;
        } => sync {
            pasitos::spreadsheet::export_sax_sync(res)?;
        }

        paso ExportSax2({
            data: pasitos::sax2::Sax2Ret,
        }) -> ( {
            res: pasitos::spreadsheet::ExportSax2Result,
        }) {
            let res = pasitos::spreadsheet::export_sax2(data).await;
        } => sync {
            pasitos::spreadsheet::export_sax2_sync(res)?;
        }

        paso ExportSax3({
            data: pasitos::sax3::Sax3Ret,
            opts: pasitos::sax3::Sax3Opts,
        }) -> ( {
            res: pasitos::spreadsheet::ExportSax3Result,
            opts: pasitos::sax3::Sax3Opts,
        }) {
            let res = pasitos::spreadsheet::export_sax3(data, opts.clone()).await;
        } => sync {
            pasitos::spreadsheet::export_sax3_sync(res, opts)?;
        }

        paso Export({
            data: asto_db::ExportData,
            judges: bool,
            dancers: bool,
            clubs: bool,
        }) -> ( {
            res: pasitos::spreadsheet::ExportResult,
        }) {
            let res = pasitos::spreadsheet::export(data, judges, dancers, clubs).await;
        } => sync {
            pasitos::spreadsheet::export_sync(res)?;
        }

        paso Import({
            judges: bool,
            dancers: bool,
            clubs: bool,
        }) -> ( {
            res: pasitos::spreadsheet::ImportResult,
        }) {
            let res = pasitos::spreadsheet::import(judges, dancers, clubs).await;
        } => sync {
            pasitos::spreadsheet::import_sync(res)?;
        }

        paso ExportDancersForAnton({
            rows: Vec<ForAntonDancerRow>,
        }) -> ( {
            res: pasitos::spreadsheet::ExportDancersForAntonResult,
        }) {
            let res = pasitos::spreadsheet::export_dancers_for_anton(rows).await;
        } => sync {
            pasitos::spreadsheet::export_dancers_for_anton_sync(res)?;
        }
    }
    // ==================================================
    // ==================================================
);
