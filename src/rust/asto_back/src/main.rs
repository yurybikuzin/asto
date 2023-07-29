#[allow(unused_imports)]
use anyhow::{anyhow, bail, Context, Error, Result};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

// https://docs.rs/built/0.5.1/built/index.html
pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
use built_info::*;
use common_macros2::*;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

mod common;
mod pasitos;
mod server;
use common::*;

#[tokio::main]
async fn main() -> Result<()> {
    main_helper().await
}

// ==================================================
// ==================================================
// You have to customize:
// - here
// - pasitos/mod.rs
// - server/mod.rs
// - server/ws/mod.rs

use asto_common::*;
// use core::marker::Tuple;

declare_env_settings_for_server! {
    settings_toml_path: std::path::PathBuf,
}

declare_settings! {
    ping_interval_secs: u64,
    db: pool_db::PoolDbSettings,
    spreadsheet: SettingsSpreadsheet,
    for_anton: SettingsForAnton,
    data: SettingsData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsData {
    pub max_at_once: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsSpreadsheet {
    pub event_result_spreadsheet_id: String,
    pub event_result_import_range: String,
    pub id: String,
    pub service_account_secret_file: String,
    //
    pub export_judges_sheet_name: String,
    pub export_clubs_sheet_name: String,
    pub export_dancers_sheet_name: String,
    //
    pub import_clubs_data_range: String,
    pub import_judges_data_range: String,
    pub import_dancers_data_range: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsForAnton {
    pub id: String,
    pub service_account_secret_file: String,
    pub export_dancers_sheet_name: String,
}

use std::path::PathBuf;

#[derive(Debug, Clone, StructOpt)]
pub enum Command {
    Server {
        #[structopt(short, long)]
        port: Option<u16>,
        #[structopt(long)]
        op_mode: Option<op_mode::OpMode>,
    },
    Export {
        #[structopt(short = "j", long)]
        judges: bool,
        #[structopt(short = "d", long)]
        dancers: bool,
        #[structopt(short = "c", long)]
        clubs: bool,
    },
    Import {
        #[structopt(short = "j", long)]
        judges: bool,
        #[structopt(short = "d", long)]
        dancers: bool,
        #[structopt(short = "c", long)]
        clubs: bool,
    },
    Sax {
        file_pathlar: Vec<PathBuf>,
    },
    Sax2 {
        file_pathlar: Vec<PathBuf>,
    },
    Sax3 {
        #[structopt(short = "p", long)]
        proto: bool,
        #[structopt(short = "s", long)]
        summary: bool,
        #[structopt(short = "d", long)]
        database: bool,
        #[structopt(long)]
        dry_run: bool,
        #[structopt(short = "j", long)]
        judges: bool,

        #[structopt(long)]
        db_url: Option<String>,

        file_pathlar: Vec<PathBuf>,
    },
    ExportDancersForAnton {},
}

// ==================================================
// ==================================================
