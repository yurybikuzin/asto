#[allow(unused_imports)]
use anyhow::{anyhow, bail, Context, Error, Result};

use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use op_mode::OpMode;
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, strum::EnumDiscriminants)]
pub enum ClientMessage {
    Ping,
    Version(semver::Version),
    Init(Vec<u8>),
    NeedInitData { key: InitDataKey, refresh: bool },
    Commit(Modal),
}

pub type NeedRefresh = bool;
#[derive(Debug, Clone, Serialize, Deserialize, strum::EnumDiscriminants)]
pub enum ServerMessage {
    Pong,
    Version(std::result::Result<NeedRefresh, String>),
    InitData(std::result::Result<Vec<u8>, String>),
    Commit(std::result::Result<Modal, String>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, strum::EnumDiscriminants)]
#[strum_discriminants(derive(strum::EnumString, strum::Display))]
#[strum_discriminants(strum(serialize_all = "snake_case"))]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub enum Modal {
    Citi(ModalCiti),
    Club(ModalClub),
    Person(Option<u16>),
    Judge(Option<u16>),
    Trainer(Option<u16>),
    Dancer(Option<u16>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct ModalCiti {
    pub id: Option<u16>,
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct ModalClub {
    pub id: Option<u16>,
    pub citi: Option<u16>,
    pub value: Option<String>,
}

impl Modal {
    pub fn caption(&self) -> &'static str {
        match self {
            Self::Citi(ModalCiti { id: None, .. }) => "Новый город",
            Self::Citi(ModalCiti { id: Some(_), .. }) => "Город",
            Self::Club(ModalClub { id: None, .. }) => "Новый клуб",
            Self::Club(ModalClub { id: Some(_), .. }) => "Клуб",
            _ => unreachable!(),
        }
    }
}

impl ClientMessage {
    pub fn init_set(params: &ClientMessageInit) -> Self {
        ClientMessage::Init(compress_bincoded(&params).unwrap())
    }
    pub fn init_get(self) -> Option<ClientMessageInit> {
        if let ClientMessage::Init(compressed_bytes) = self {
            decompress_bincoded::<ClientMessageInit>(compressed_bytes).ok()
        } else {
            None
        }
    }
    pub fn encoded(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    pub fn from_encoded(value: &[u8]) -> Result<Self> {
        bincode::deserialize(value).map_err(|err| anyhow!("{}:{}: {err}", file!(), line!()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerMessageNewClub {
    citi_id: i16,
    club_id: i16,
}

impl ServerMessage {
    pub fn init_data_set(init_data: &InitData) -> Self {
        Self::InitData(compress_bincoded(&init_data))
    }
    pub fn init_data_get(self) -> Option<std::result::Result<InitData, String>> {
        if let Self::InitData(res) = self {
            Some(match res {
                Ok(compressed_bytes) => decompress_bincoded::<InitData>(compressed_bytes)
                    .map_err(|err| format!("{err}")),
                Err(err) => Err(err),
            })
        } else {
            None
        }
    }
    pub fn encoded(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    pub fn from_encoded(value: &[u8]) -> Result<Self> {
        bincode::deserialize(value).map_err(|err| anyhow!("{}:{}: {err}", file!(), line!()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientMessageInit {
    pub key: InitDataKey,
}
// https://developer.mozilla.org/en-US/docs/Web/API/Location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub protocol: String,
    pub host: String,
    pub port: Option<u16>,
    pub pathname: String,
    pub search: Option<String>,
    pub hash: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct InitDataKey {
    pub op_mode: OpMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientMessageLogin {
    pub location: Location,
    pub token: String,
    pub fingerprint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitData {
    pub today: Option<chrono::NaiveDate>,
    pub textlar: Vec<Text>,
    pub citilar: Vec<Citi>,
    pub clublar: Vec<Club>,
    pub genderlar: Vec<Gender>,
    pub first_namelar: Vec<FirstName>,
    pub second_namelar: Vec<SecondName>,
    pub last_namelar: Vec<LastName>,
    pub nick_namelar: Vec<NickName>,
    pub personlar: Vec<Person>,
    pub categorilar: Vec<Categori>,
    pub judgelar: Vec<Judge>,
    pub trainerlar: Vec<Trainer>,
    pub classlar: Vec<Class>,
    pub dancerlar: Vec<Dancer>,
    pub eventlar: Vec<Event>,
    pub event_resultlar: Vec<EventResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: i32,
    pub date: chrono::NaiveDate,
    pub title: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventResult {
    pub id: i32,
    pub event: i32,
    pub category: i32,
    pub external_id: i32,
    pub couple_num: i16,
    // pub place: Option<i16>,
    pub st_score: Option<i16>,
    pub la_score: Option<i16>,
    pub st_la_score: Option<i16>,
    pub points: Option<i16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text {
    pub id: i32,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dancer {
    pub id: i32,
    pub external_id: Option<i32>,
    pub person: i32,
    pub st_class: i8,
    pub la_class: i8,
    pub st_score: i16,
    pub la_score: i16,
    pub st_la_score: i16,
    pub points: i16,
    pub club: i16,
    pub trainer: i32,
    pub trainer2: i32,
    pub is_archive: bool,
}
impl Dancer {
    pub fn st_class(&self) -> i8 {
        if let Some((_, st_class, _)) = self.class_upgrade() {
            st_class
        } else {
            self.st_class
        }
    }
    pub fn la_class(&self) -> i8 {
        if let Some((_, _, la_class)) = self.class_upgrade() {
            la_class
        } else {
            self.la_class
        }
    }
    pub fn is_beginning(&self, at: &Option<chrono::NaiveDate>, less_than_n4: bool) -> bool {
        let (st_class, la_class) = if let Some((date, st_class, la_class)) = self.class_upgrade() {
            if let Some(at) = at.as_ref() {
                if at >= &date {
                    (st_class, la_class)
                } else {
                    (self.st_class, self.la_class)
                }
            } else {
                (st_class, la_class)
            }
        } else {
            (self.st_class, self.la_class)
        };
        let limit = if less_than_n4 {
            10 // Н3
        } else {
            8 // Н5
        };
        st_class >= limit && la_class >= limit
    }
    pub fn class_upgrade(
        &self,
    ) -> Option<(
        chrono::NaiveDate,
        i8, /* to St-класс */
        i8, /* to La-класс */
    )> {
        class_upgrade(self.external_id)
    }
    pub fn score_points_class(&self, at: &Option<chrono::NaiveDate>) -> &str {
        if self.is_beginning(at, false) {
            "point"
        } else {
            "score"
        }
    }
}

pub fn class_as_string(class: i8) -> Option<&'static str> {
    match class {
        1 => Some("M"),
        2 => Some("S"),
        3 => Some("A"),
        4 => Some("B"),
        5 => Some("C"),
        6 => Some("D"),
        7 => Some("E"),
        8 => Some("H5"),
        9 => Some("H4"),
        10 => Some("H3"),
        11 => Some("H2"),
        _ => None,
    }
}

pub fn class_upgrade(
    external_id: Option<i32>,
) -> Option<(
    chrono::NaiveDate,
    i8, /* to St-класс */
    i8, /* to La-класс */
)> {
    match external_id {
        Some(5590122 | 5590119) => {
            // Алексеев: Косьянова, Губанова
            Some((
                NaiveDate::from_ymd_opt(2023, 5, 28).unwrap(),
                6, /* to D-класс */
                6, /* to D-класс */
            ))
        }
        Some(5500095) => {
            // Станиславская: Кожухова Дарья
            Some((
                NaiveDate::from_ymd_opt(2023, 5, 23).unwrap(),
                6, /* to D-класс */
                6, /* to D-класс */
            ))
        }
        Some(5500360) => {
            // Ржевская: Осипов Владислав Александрович в Е-класс по заявлению педагога
            Some((
                NaiveDate::from_ymd_opt(2023, 5, 14).unwrap(),
                7, /* to E-класс */
                7, /* to E-класс */
            ))
        }
        Some(5500296) => {
            // Ржевская: Конюшко Анна Андреевна в Н4
            Some((
                NaiveDate::from_ymd_opt(2023, 5, 14).unwrap(),
                9, /* to Н4-класс */
                9, /* to Н4-класс */
            ))
        }
        Some(5500355) => {
            // Ржевская: Витенберг Алиса Антоновна в Н3
            Some((
                NaiveDate::from_ymd_opt(2023, 5, 14).unwrap(),
                10, /* to Н3-класс */
                10, /* to Н3-класс */
            ))
        }
        Some(5500017 | 5500018) => {
            // Ржевская: Рябенко Сергей, Осипова Полина
            Some((
                NaiveDate::from_ymd_opt(2023, 5, 14).unwrap(),
                6, /* to D-класс */
                6, /* to D-класс */
            ))
        }
        Some(5500030) => {
            // Ржевская: Терзинова, автомат
            Some((
                NaiveDate::from_ymd_opt(2023, 5, 11).unwrap(),
                5, /* to C-класс */
                5, /* to C-класс */
            ))
        }
        Some(5500305 | 5500306) => {
            // Смиронова: Спиридонова Ольга и Шеховцов Егор
            Some((
                NaiveDate::from_ymd_opt(2023, 5, 4).unwrap(),
                6, /* to D-класс */
                6, /* to D-класс */
            ))
        }
        Some(5500305 | 5500306) => {
            // Смиронова: Спиридонова Ольга и Шеховцов Егор
            Some((
                NaiveDate::from_ymd_opt(2023, 5, 4).unwrap(),
                6, /* to D-класс */
                6, /* to D-класс */
            ))
        }
        Some(5500203 | 5500202) => {
            // Кандудин: Зиновьев, Русанова
            Some((
                NaiveDate::from_ymd_opt(2023, 5, 1).unwrap(),
                6, /* to D-класс */
                6, /* to D-класс */
            ))
        }
        Some(5590124) => {
            // Алексеев: Панюшкин
            Some((
                NaiveDate::from_ymd_opt(2023, 4, 19).unwrap(),
                7, /* to E-класс */
                7, /* to E-класс */
            ))
        }
        Some(5500311 | 5500308 | 5500312) => {
            // Алексеев: Рязанцева, Малинина, Пословская
            Some((
                NaiveDate::from_ymd_opt(2023, 4, 19).unwrap(),
                10, /* to Н3-класс */
                10, /* to Н3-класс */
            ))
        }
        Some(5530108 | 5590106) => {
            // Муравьёва: Мартынова Полина / Мартынова Кристина
            Some((
                NaiveDate::from_ymd_opt(2023, 4, 12).unwrap(),
                7, /* to E-класс */
                7, /* to E-класс */
            ))
        }
        Some(5500382) => {
            // Муравьёва: Вишнякова
            Some((
                NaiveDate::from_ymd_opt(2023, 4, 12).unwrap(),
                8, /* to Н5-класс */
                8, /* to Н5-класс */
            ))
        }
        Some(5500211) => {
            // Буров: Макагонова
            Some((
                NaiveDate::from_ymd_opt(2023, 4, 9).unwrap(),
                6, /* to D-класс */
                6, /* to D-класс */
            ))
        }
        Some(5500303) => {
            // Могилко: Клепиков в Н3
            Some((
                NaiveDate::from_ymd_opt(2023, 3, 20).unwrap(),
                10, /* to Н3-класс*/
                10, /* to Н3-класс*/
            ))
        }
        Some(5590108) => {
            // Ржевская: Сулковская в Н5
            Some((
                NaiveDate::from_ymd_opt(2023, 3, 20).unwrap(),
                8, /* to Н5-класс*/
                8, /* to Н5-класс*/
            ))
        }
        Some(5530104) => {
            // Соколов: Ефимова в Н4
            Some((
                NaiveDate::from_ymd_opt(2023, 3, 15).unwrap(),
                9, /* to Н4-класс*/
                9, /* to Н4-класс*/
            ))
        }
        Some(5500263 | 5500264 | 5500265) => {
            // Могилко: Чечуй в Е-класс
            Some((
                NaiveDate::from_ymd_opt(2023, 3, 4).unwrap(),
                8, /* to Н5-класс*/
                8, /* to Н5-класс*/
            ))
        }
        Some(5500302) => {
            // Могилко: Чечуй в Е-класс
            Some((
                NaiveDate::from_ymd_opt(2023, 3, 1).unwrap(),
                7, /* to E-класс*/
                7, /* to E-класс*/
            ))
        }
        Some(5590121 | 5590119) => {
            // Гашилову и Губанову в Е-класс
            Some((
                NaiveDate::from_ymd_opt(2023, 2, 27).unwrap(),
                7, /* to E-класс*/
                7, /* to E-класс*/
            ))
        }
        Some(5500218 | 5500219) => {
            // Присвоить Дкл Гавриш Андрею ( 5500218 ) и Лебедь Ульяне ( 5500219 ) по возрасту
            Some((
                NaiveDate::from_ymd_opt(2023, 2, 27).unwrap(),
                6, /* to D-класс*/
                6, /* to D-класс*/
            ))
        }
        Some(5500289) => {
            // Буров: Иващенко в D-класс
            Some((
                NaiveDate::from_ymd_opt(2023, 2, 27).unwrap(),
                6, /* to D-класс*/
                6, /* to D-класс*/
            ))
        }
        Some(5500093) => {
            // Станиславская: Куницына в D-класс
            Some((
                NaiveDate::from_ymd_opt(2022, 12, 13).unwrap(),
                6, /* to D-класс*/
                6, /* to D-класс*/
            ))
        }
        Some(5530114) => {
            // Ржевская: Фомина Зоя в Н5-класс
            Some((
                NaiveDate::from_ymd_opt(2023, 2, 23).unwrap(),
                8, /* to Н5-класс*/
                8, /* to Н5-класс*/
            ))
        }
        Some(5500320) => {
            // Шляхов: Лукьянчук Каролина в E-класс
            Some((
                NaiveDate::from_ymd_opt(2023, 2, 14).unwrap(),
                7, /* to E-класс*/
                7, /* to E-класс*/
            ))
        }
        Some(5590122) => {
            // Косьянова в E-класс
            Some((
                NaiveDate::from_ymd_opt(2023, 2, 12).unwrap(),
                7, /* to E-класс*/
                7, /* to E-класс*/
            ))
        }
        Some(5500226 | 5500123) => {
            // Кварталова: Васин, Колпакова
            Some((
                NaiveDate::from_ymd_opt(2023, 2, 8).unwrap(),
                6, /* to D-класс*/
                6, /* to D-класс*/
            ))
        }
        // Some(5590124 | 5590119 | 5590122) => { // Косьянова в E-класс
        Some(5590124 | 5590119) => {
            // Алексеевские (Мечта, Тула)
            Some((
                NaiveDate::from_ymd_opt(2023, 2, 2).unwrap(),
                8, /* to Н5-класс*/
                8, /* to Н5-класс*/
            ))
        }
        Some(5500367) => {
            // Крутикова - была добавлена как Н4 и сразу переведена в Н5, чтобы
            // сбросить очки
            Some((
                NaiveDate::from_ymd_opt(2023, 2, 1).unwrap(),
                8, /* to H5-класс*/
                8, /* to H5-класс*/
            ))
        }
        Some(5500004 | 5500177 | 5500245 | 5500217 | 5500225) => {
            Some((
                NaiveDate::from_ymd_opt(2023, 1, 16).unwrap(),
                6, /* to D-класс*/
                6, /* to D-класс*/
            ))
        }
        Some(5530136) => {
            Some((
                NaiveDate::from_ymd_opt(2022, 12, 27).unwrap(),
                8, /* to H5-класс*/
                8, /* to H5-класс*/
            ))
        }
        Some(5500087) => {
            Some((
                NaiveDate::from_ymd_opt(2022, 12, 27).unwrap(),
                6, /* to D-класс*/
                6, /* to D-класс*/
            ))
        }
        Some(5500028 | 5500055) => {
            Some((
                NaiveDate::from_ymd_opt(2022, 12, 10).unwrap(),
                6, /* to D-класс*/
                6, /* to D-класс*/
            ))
        }
        Some(5500050 | 5500076 | 5500082 | 5500083 | 5500215 | 5500216) => {
            Some((
                NaiveDate::from_ymd_opt(2022, 12, 1).unwrap(),
                6, /* to D-класс*/
                6, /* to D-класс*/
            ))
        }
        Some(_) | None => None,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    pub id: i8,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trainer {
    pub id: i32,
    pub person: i32,
    pub club: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Judge {
    pub id: i32,
    pub external_id: Option<i32>,
    pub person: i32,
    pub categori: i8,
    pub assignment_date: chrono::NaiveDate,
    pub club: i16,
    pub number_of_participation_in_festivals: i16,
    pub is_archive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Categori {
    pub id: i8,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: i32,
    pub last_name: i16,
    pub first_name: i16,
    pub second_name: i16,
    pub nick_name: i16,
    pub birth_date: chrono::NaiveDate,
    pub gender: Option<i8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NickName {
    pub id: i16,
    pub value: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LastName {
    pub id: i16,
    pub value: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondName {
    pub id: i16,
    pub value: i32,
    pub default_gender: Option<i8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirstName {
    pub id: i16,
    pub value: i32,
    pub default_gender: Option<i8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Gender {
    pub id: i8,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Citi {
    pub id: i16,
    pub value: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Club {
    pub id: i16,
    pub value: i32,
    pub citi: i16,
    pub chief: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddonRow {
    pub id: i32,
    pub name: String,
    pub second_name: Option<String>,
    pub birth_date: Option<chrono::NaiveDate>,
    pub n_class: String,
    pub club: String,
    pub citi: String,
    pub trainer: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScannedDancer {
    pub id: i32,
    pub name: String,
    pub second_name: String,
    pub birth_date: Option<chrono::NaiveDate>,
    pub st_class: String,
    pub la_class: String,
    pub st_la_score: f64,
    pub b_st_score: f64,
    pub b_la_score: f64,
    pub club: String,
    pub citi: String,
    pub trainer: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScannedJudge {
    pub id: i32,
    pub name: String,
    pub second_name: String,
    pub categori: String,
    pub assignment_date: Option<chrono::NaiveDate>,
    pub club: String,
    pub citi: String,
    pub number_of_participation_in_festivals: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JudgeRow {
    pub external_id: Option<i32>,
    pub last_name: Option<String>,
    pub first_name: Option<String>,
    pub second_name: Option<String>,
    pub nick_name: Option<String>,
    pub categori: Option<String>,
    pub assignment_date: Option<chrono::NaiveDate>,
    pub club: Option<String>,
    pub citi: Option<String>,
    pub number_of_participation_in_festivals: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JudgeSpreadsheetRow {
    pub id: Option<i32>,
    pub external_id: Option<i32>,
    pub last_name: String,
    pub first_name: String,
    pub second_name: Option<String>,
    pub nick_name: Option<String>,
    pub categori: Option<String>,
    pub assignment_date: Option<chrono::NaiveDate>,
    pub club: Option<String>,
    pub citi: Option<String>,
    pub number_of_participation_in_festivals: Option<i32>,
    pub is_archive: Option<bool>,
    // pub is_archive: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ClubSpreadsheetRow {
    pub id: Option<i32>,
    pub club: String,
    pub citi: String,
    pub chief_last_name: Option<String>,
    pub chief_first_name: Option<String>,
    pub chief_second_name: Option<String>,
    pub chief_nick_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DancerSpreadsheetRow {
    pub id: Option<i32>,
    pub external_id: Option<i32>,
    pub last_name: String,
    pub first_name: String,
    pub second_name: Option<String>,
    pub nick_name: Option<String>,
    pub birth_date: Option<chrono::NaiveDate>,
    pub trainer_last_name: Option<String>,
    pub trainer_first_name: Option<String>,
    pub trainer_second_name: Option<String>,
    pub trainer_nick_name: Option<String>,
    pub trainer2_last_name: Option<String>,
    pub trainer2_first_name: Option<String>,
    pub trainer2_second_name: Option<String>,
    pub trainer2_nick_name: Option<String>,
    pub club: Option<String>,
    pub citi: Option<String>,
    pub st_class: Option<String>,
    pub la_class: Option<String>,
    pub st_score: f64,
    pub la_score: f64,
    pub st_la_score: f64,
    pub points: f64,
    pub is_archive: Option<bool>,
}

pub struct ImportRet {
    pub clubs: Vec<ClubSpreadsheetRow>,
    pub judges: Vec<JudgeSpreadsheetRow>,
    pub dancers: Vec<DancerSpreadsheetRow>,
}

pub type ImportEventRet = Vec<ImportEventRow>;

use chrono::NaiveDate;
#[derive(Debug)]
pub struct ImportEventRow {
    pub date: NaiveDate,
    pub title: String,
    pub category: String,
    pub couple_number: i16,
    pub st_score: Option<i16>,
    pub la_score: Option<i16>,
    pub st_la_score: Option<i16>,
    pub points: Option<i16>,
    pub external_id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub dancer_class: Option<String>,
    pub birthdate: Option<NaiveDate>,
    pub club: String,
    pub city: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DancerRow {
    pub external_id: Option<i32>,
    pub last_name: Option<String>,
    pub first_name: Option<String>,
    pub second_name: Option<String>,
    pub nick_name: Option<String>,
    pub birth_date: Option<chrono::NaiveDate>,
    pub trainer_last_name: Option<String>,
    pub trainer_first_name: Option<String>,
    pub trainer_second_name: Option<String>,
    pub trainer_nick_name: Option<String>,
    pub club: Option<String>,
    pub citi: Option<String>,
    pub st_class: Option<String>,
    pub la_class: Option<String>,
    pub st_score: f64,
    pub la_score: f64,
    pub st_la_score: f64,
    pub points: f64,
}

pub struct ForAntonDancerRow {
    pub external_id: Option<i32>,
    pub name: Option<String>,
    pub second_name: Option<String>,
    pub birth_date: Option<chrono::NaiveDate>,
    pub st_class: Option<String>,
    pub la_class: Option<String>,
    pub club: Option<String>,
    pub citi: Option<String>,
    pub trainer: Option<String>,
    pub trainer2: Option<String>,
    pub chief: Option<String>,
    pub region: Option<i32>,
    pub gender: Option<String>,
}

#[derive(Debug)]
pub struct AsFtsarrClub {
    pub club_id: i32,
    pub club_name: String,
    pub cityid: i32,
    pub chief1_sur_name: Option<String>,
    pub chief1_first_name: Option<String>,
}
pub struct AsFtsarr {
    pub club: Vec<AsFtsarrClub>,
}

// ==================================================

fn compress_bincoded<S: Serialize>(data: &S) -> std::result::Result<Vec<u8>, String> {
    let mut e = GzEncoder::new(Vec::new(), Compression::default());
    bincode::serialize(&data)
        .map_err(|err| format!("{err}"))
        .and_then(|encoded| {
            e.write_all(&encoded)
                .map_err(|err| format!("{err}"))
                .and_then(|_| e.finish().map_err(|err| format!("{err}")))
        })
}

fn decompress_bincoded<T: serde::de::DeserializeOwned>(
    compressed_bytes: Vec<u8>,
) -> std::result::Result<T, Box<bincode::ErrorKind>> {
    let mut d = GzDecoder::new(&*compressed_bytes);
    let mut encoded = vec![];
    d.read_to_end(&mut encoded).unwrap();
    bincode::deserialize::<T>(&encoded)
}

pub fn is_active(external_id: &Option<i32>, event_date: &Option<chrono::NaiveDate>) -> bool {
    (external_id
        .as_ref()
        .map(|external_id| ACTIVE_IN_2023.contains(external_id))
        .unwrap_or(false)
        || (event_date
            .as_ref()
            .map(|event_date| event_date < &ACTIVE_SINCE))
        .unwrap_or(false))
        && external_id
            .as_ref()
            .map(|external_id| {
                !(external_id == &5500251
                    && event_date
                        .map(|event_date| event_date == *CURSED_DATE)
                        .unwrap_or(false))
            })
            .unwrap_or(true)
    // }
}

lazy_static::lazy_static! {
    pub static ref BASE_DATE: chrono::NaiveDate = chrono::NaiveDate::from_ymd_opt(2022, 10, 1).unwrap();
    pub static ref CURSED_DATE: chrono::NaiveDate = chrono::NaiveDate::from_ymd_opt(2023, 2, 19).unwrap();
    pub static ref ACTIVE_SINCE: chrono::NaiveDate = chrono::NaiveDate::from_ymd_opt(2023, 2, 1).unwrap();
    pub static ref ACTIVE_IN_2023: std::collections::HashSet<i32> = [
5500007,
5500017,
5500018,
5500021,
5500022,
5500027,
5500028,
5500030,
5500031,
5500033,
5500037,
5500038,
5500043,
5500044,
5500052,
5500055,
5500074,
5500075,
5500082,
5500083,
5500086,
5500087,
5500091,
5500093,
5500095,
5500097,
5500098,
5500106,
5500122,
5500123,
5500124,
5500125,
5500128,
5500133,
5500134,
5500148,
5500176,
5500179,
5500180,
5500194,
5500195,
5500198,
5500199,
5500201,
5500202,
5500203,
5500204,
5500205,
5500206,
5500207,
5500209,
5500210,
5500211,
5500215,
5500216,
5500217,
5500218,
5500219,
5500220,
5500221,
5500225,
5500226,
5500230,
5500234,
5500235,
5500243,
5500244,
5500246,
5500247,
5500248,
5500249,
5500260,
5500261,
5500262,
5500263,
5500264,
5500265,
5500266,
5500267,
5500269,
5500270,
5500271,
5500275,
5500276,
5500278,
5500281,
5500288,
5500289,
5500291,
5500292,
5500293,
5500294,
5500298,
5500301,
5500302,
5500304,
5500305,
5500306,
5500343,
5500344,
5500346,
5500354,
5500357,
5500358,
5500360,
5500366,
5500367,
5500368,
5500369,
5500370,
5500371,
5500372,
5500373,
5500375,
5500377,
5500378,
5500379,
5500380,
5500381,
5500382,
5500383,
5530033,
5530098,
5530102,
5530103,
5530107,
5530108,
5530114,
5530115,
5530117,
5530119,
5530120,
5530122,
5530125,
5530134,
5530135,
5530136,
5530137,
5530138,
5590102,
5590103,
5590106,
5590108,
5590119,
5590121,
5590122,
5590124,
5500251, // Зудикова
5530104, // Ефимова
// Латина плюс (Кубинка):
5500056,
5500057,
5500237,
5530042,
5530106,
5502601,
5504242,
// Шляхов, АЛС:
5500313,
5500314,
5500315,
5500316,
5500317,
5500318,
5500319,
5500320,
5500321,
5500322,
5500323,
5500324,
5500325,
5500326,
5500330,
5500331,
5500332,
5500333,
5500334,
5500335,
// Евсеева, Алиев
5500252,
5500253,
// Горькова Лилия
5500257,
// Конюшко Анна
5500296,
    ].into_iter().collect();
}

// ==================================================================

pub mod route;
