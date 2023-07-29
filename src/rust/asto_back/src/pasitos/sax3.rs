use super::*;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;

use chrono::NaiveDate;
use xml::reader::{EventReader, XmlEvent};

// ==============================================================================

#[derive(Clone)]
pub struct Sax3Opts {
    pub proto: bool,
    pub summary: bool,
    pub database: bool,
    pub dry_run: bool,
    pub judges: bool,
    pub db_url: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Sax3Compet {
    pub category: String,
    pub couples: HashMap<CoupleNumber, Sax3Couple>,
    pub rounds: HashMap<Sax3RoundNumber, Sax3Round>,
    pub judges: Option<Sax3Judges>,
}

pub type Sax3RoundNumber = i16;

#[derive(Debug, Clone, Serialize)]
pub struct Sax3Round {
    pub name: String,
    pub board_point: Option<i16>,
    pub mode: Sax3RoundMode,
    pub total_results: HashMap<CoupleNumber, Sax3RoundTotalRes>,
    pub judges: Option<Sax3Judges>,
    pub result_details: HashMap<Sax3DanceKey, HashMap<CoupleNumber, Sax3RoundDanceRes>>,
}
pub type CoupleNumber = i16;

#[derive(Debug, Clone, Serialize)]
pub struct Sax3RoundDanceRes {
    pub sum: Option<f64>,
    pub place: Option<f64>,
    pub details: Sax3RoundDanceResDetails,
}

#[derive(Debug, Clone, Serialize)]
pub enum Sax3RoundDanceResDetails {
    Crosses(HashSet<i16>),     // set of private judges, who gave a vote to couple
    Places(HashMap<i16, i16>), // map of private judges to places
}

#[derive(Debug, Clone, Copy, strum::Display, Serialize)]
#[strum(serialize_all = "snake_case")]
pub enum Sax3RoundMode {
    Ball,
    Skating,
    Sum,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct Sax3DanceKey {
    pub number: i16,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Sax3RoundTotalRes {
    pub place: Option<f64>,
    pub sum: Option<f64>,
}

#[derive(Debug)]
pub struct Sax3RoundPreRes {
    pub place: Option<f64>,
    pub sum: Option<f64>,
    pub couple_number: i16,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Sax3Judges(pub HashMap<Sax3JudgeId, Sax3Judge>);
impl Sax3Judges {
    pub fn private(&self) -> Vec<(Sax3JudgeId, Sax3Judge)> {
        let mut ret = self
            .clone()
            .0
            .into_iter()
            .filter(|(id, _)| matches!(id, Sax3JudgeId::Private { .. }))
            .collect::<Vec<_>>();
        ret.sort_by_key(|i| i.0);
        ret
    }
    pub fn all(&self) -> Vec<(Sax3JudgeId, Sax3Judge)> {
        let mut ret = self.clone().0.into_iter().collect::<Vec<_>>();
        ret.sort_by_key(|i| i.0);
        ret
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Sax3Judge {
    pub last_name: String,
    pub first_name: String,
    pub second_name: String,
    pub club: Option<String>,
    pub city: Option<String>,
    pub book_number: Option<i16>,
    pub category: Option<String>,
}

impl Sax3Judge {
    fn key_tuple(&self) -> (&String, &String) {
        (&self.last_name, &self.first_name)
    }
}
impl Hash for Sax3Judge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key_tuple().hash(state);
    }
}

impl PartialEq for Sax3Judge {
    fn eq(&self, other: &Self) -> bool {
        self.key_tuple() == other.key_tuple()
    }
}

impl Eq for Sax3Judge {}

impl Ord for Sax3Judge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key_tuple().cmp(&other.key_tuple())
    }
}

impl PartialOrd for Sax3Judge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize)]
pub enum Sax3JudgeId {
    Main(i16),
    Deputy(i16),
    Private(i16, char),
}
impl Sax3JudgeId {
    pub fn get_i(&self) -> i16 {
        *match self {
            Self::Main(i) => i,
            Self::Deputy(i) => i,
            Self::Private(i, _) => i,
        }
    }
}
use std::cmp::Ordering;
impl PartialOrd for Sax3JudgeId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Sax3JudgeId {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Main(i), Self::Main(j)) => i.cmp(j),
            (Self::Main(_), _) => Ordering::Less,
            (_, Self::Main(_)) => Ordering::Greater,
            (Self::Deputy(i), Self::Deputy(j)) => i.cmp(j),
            (Self::Deputy(_), _) => Ordering::Less,
            (_, Self::Deputy(_)) => Ordering::Greater,
            (Self::Private(i, _), Self::Private(j, _)) => i.cmp(j),
        }
    }
}
common_macros2::impl_display!(Sax3JudgeId, self, f, {
    match self {
        Self::Main(i) => write!(f, "ГС({i})"),
        Self::Deputy(i) => write!(f, "ЗГС({i})"),
        Self::Private(_, ch) => write!(f, "{ch}"),
    }
});

#[derive(Debug, Clone, Serialize)]
pub struct Sax3Couple {
    // pub points: Option<f64>,
    pub place: Option<i16>,
    pub class: Option<String>,
    pub club: Sax3Club,
    pub male: Sax3Dancer,
    pub female: Option<Sax3Dancer>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Sax3Dancer {
    pub points: Option<f64>,
    pub first_name: String,
    pub last_name: String,
    pub book_number: Option<i32>,
    pub birth_day: Option<chrono::NaiveDate>,
    pub class: Option<String>,
    // pub class_place: Option<i32>,
    // pub class_int_reg: Option<i32>,
}

impl Sax3Dancer {
    pub fn new(attributes: &[OwnedAttribute], couple_points: &Option<f64>) -> Self {
        Self {
            first_name: attr("firstName", attributes).unwrap(),
            last_name: attr("lastName", attributes)
                .map(|s| match s.as_str() {
                    "Витенбург" => "Витенбeрг".to_owned(),
                    _ => s,
                })
                .unwrap(),
            class: attr("class", attributes),
            book_number: attr("bookNumber", attributes).and_then(|s| s.parse::<i32>().ok()),
            birth_day: attr("birthDay", attributes)
                .and_then(|s| NaiveDate::parse_from_str(&s, "%d.%m.%Y").ok()),
            points: {
                let ret = attr("points", attributes)
                    .map(|s| {
                        s.replace(',', ".")
                            .parse::<f64>()
                            // .context(format!("{s}"))
                            .context(s.to_string())
                            .unwrap()
                    })
                    .or(*couple_points)
                    .unwrap();
                (ret > 0f64).then_some(ret)
            },
        }
    }
}

impl Hash for Sax3Dancer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.first_name.hash(state);
        self.last_name.hash(state);
    }
}

impl PartialEq for Sax3Dancer {
    fn eq(&self, other: &Self) -> bool {
        self.first_name == other.first_name && self.last_name == other.last_name
    }
}

impl Eq for Sax3Dancer {}

#[derive(Debug, Clone, Eq, Serialize)]
pub struct Sax3Club {
    pub city: String,
    pub name: String,
    pub chief1_last_name: Option<String>,
    pub chief1_first_name: Option<String>,
    pub trener1_last_name: Option<String>,
    pub trener1_first_name: Option<String>,
}

use std::hash::{Hash, Hasher};
impl Hash for Sax3Club {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.city.hash(state);
        self.name.hash(state);
    }
}

impl PartialEq for Sax3Club {
    fn eq(&self, other: &Self) -> bool {
        self.city == other.city && self.name == other.name
    }
}

const STR_RE_JUDGE_STATUS: &str = r#"^(?P<code>!|ЗГС|[A-Z])\((?P<id>\d+)\)$"#;
lazy_static::lazy_static! {
    pub static ref RE_JUDGE_STATUS: regex::Regex = regex::Regex::new(STR_RE_JUDGE_STATUS).unwrap();
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, PartialOrd, Ord)]
pub struct Sax3RetKey {
    pub date: chrono::NaiveDate,
    pub title: String,
}
common_macros2::impl_display!(Sax3RetKey, self, "{}::{}", self.date, self.title);
pub type Sax3Ret = HashMap<Sax3RetKey, Vec<Sax3Compet>>;
pub type Sax3Result = Result<Sax3Ret>;
pub fn sax3(file_pathlar: &Vec<PathBuf>) -> Sax3Result {
    let mut ret: Sax3Ret = HashMap::new();
    for file_path in file_pathlar {
        let file = File::open(file_path)?;
        let file = BufReader::new(file);
        let parser = EventReader::new(file);
        let mut date: Option<NaiveDate> = None;
        let mut ret_key: Option<Sax3RetKey> = None;
        let mut category: Option<String> = None;
        let mut in_group = false;
        let mut in_title = false;
        let mut round: Option<Sax3Round> = None;
        let mut round_number: Option<i16> = None;
        #[derive(Debug, Clone)]
        pub struct Couple {
            n: i16,
            place: Option<i16>,
            class: Option<String>,
        }
        let mut couple: Option<Couple> = None;
        let mut couple_points: Option<f64> = None;
        let mut club = None;
        let mut male = None;
        let mut female = None;
        let mut couples: HashMap<i16, Sax3Couple> = HashMap::new();
        let mut rounds: HashMap<Sax3RoundNumber, Sax3Round> = HashMap::new();
        let mut total_results: HashMap<i16, Sax3RoundTotalRes> = HashMap::new();
        let mut dance_key: Option<Sax3DanceKey> = None;
        let mut dance_pre_res: Option<Sax3RoundPreRes> = None;
        let mut judges = None;
        for e in parser {
            match e {
                Ok(XmlEvent::Characters(s)) => {
                    if in_group {
                        category = Some(s);
                    } else if in_title {
                        ret_key = Some(Sax3RetKey {
                            title: {
                                if date.as_ref().unwrap().to_string() == "2023-05-13" {
                                    "DANCE LIKЕ".to_owned()
                                } else {
                                    let s = s
                                        .strip_suffix(" массовый спорт")
                                        .map(|s| s.to_owned())
                                        .unwrap_or(s);
                                    let s =
                                        s.strip_suffix(" СВД").map(|s| s.to_owned()).unwrap_or(s);
                                    let s =
                                        s.strip_suffix(" (СММ)").map(|s| s.to_owned()).unwrap_or(s);
                                    s.strip_suffix(" СММ").map(|s| s.to_owned()).unwrap_or(s)
                                }
                            },
                            date: date.take().unwrap(),
                        })
                    } else if let Some(Sax3RoundPreRes {
                        place,
                        sum,
                        couple_number,
                    }) = dance_pre_res.take()
                    {
                        let private_judges = round
                            .as_ref()
                            .map(|r| r.judges.as_ref().or(judges.as_ref()).unwrap().private())
                            .unwrap();
                        assert_eq!(
                            s.chars().count(),
                            private_judges.len(),
                            "s: {s:?}, private_judges: {private_judges:?}, category: {category:?}, round: {round:#?}"
                        );
                        let details = if s.chars().next().unwrap().is_ascii_digit() {
                            let places = s
                                .chars()
                                .enumerate()
                                .filter_map(|(i, ch)| {
                                    ('1'..='9').contains(&ch).then(|| {
                                        (
                                            private_judges.get(i).unwrap().0.get_i(),
                                            ch as i16 - '1' as i16,
                                        )
                                    })
                                })
                                .collect::<HashMap<_, _>>();
                            Sax3RoundDanceResDetails::Places(places)
                        } else {
                            let crosses = s
                                .chars()
                                .enumerate()
                                .filter_map(|(i, ch)| {
                                    (ch == 'X').then(|| private_judges.get(i).unwrap().0.get_i())
                                })
                                .collect::<HashSet<_>>();
                            Sax3RoundDanceResDetails::Crosses(crosses)
                        };
                        let Some(round) = round.as_mut() else {
                            unreachable!();
                        };
                        let round_dance_res = Sax3RoundDanceRes {
                            sum,
                            place,
                            details,
                        };
                        common_macros2::entry!(round.result_details, dance_key.clone().unwrap()
                        =>
                            and_modify |e| {
                                e.insert(couple_number, round_dance_res)
                            }
                            or_insert vec![(couple_number, round_dance_res)].into_iter().collect()

                        );
                    }
                }
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if couple.is_some() {
                        match name.local_name.as_str() {
                            "Male" => male = Some(Sax3Dancer::new(&attributes[..], &couple_points)),
                            "Female" => {
                                female = Some(Sax3Dancer::new(&attributes[..], &couple_points))
                            }
                            "Club" => {
                                club = Some(Sax3Club {
                                    city: attr("city", &attributes[..])
                                        .unwrap_or_else(|| "Не указан".to_owned()),
                                    name: attr("name", &attributes[..])
                                        .map(|s| match s.as_str() {
                                            "ТСК Прожектор" => {
                                                "ДК Прожектор".to_owned()
                                            }
                                            "ТСК Дэнс Арт Скул" => {
                                                "Арт Дэнс Сколково".to_owned()
                                            }
                                            "Шоколад" => "Мистерия".to_owned(),
                                            "Терпсихора" => "Мистерия".to_owned(),
                                            "Останкино" => "TV-DANCE".to_owned(), // 5500305 Шеховцов Егор Александрович - 5500306 Спиридонова Ольга Андреевна
                                            _ => {
                                                if let Some(s) = s.strip_prefix("ТСК ") {
                                                    s.to_owned()
                                                } else {
                                                    s
                                                }
                                            }
                                        })
                                        .unwrap_or_else(|| "Не указан".to_owned()),
                                    chief1_last_name: attr("chief1LastName", &attributes[..]),
                                    chief1_first_name: attr("chief1FirstName", &attributes[..]),
                                    trener1_last_name: attr("trener1LastName", &attributes[..]),
                                    trener1_first_name: attr("trener1FirstName", &attributes[..]),
                                })
                            }
                            s => unreachable!("{s}"),
                        }
                    }
                    match name.local_name.as_str() {
                        "Dance" => {
                            let need_correct = if let Some(category) = category.as_ref() {
                                category
                                    == "Соло, Юниоры-1, Юниоры-2, Молодежь, Румба (до D класса)"
                                    && {
                                        if let Some(round) = round.as_ref() {
                                            round.name == "Финал"
                                        } else {
                                            false
                                        }
                                    }
                            } else {
                                false
                            };
                            dance_key = if !need_correct {
                                Some(Sax3DanceKey {
                                    number: attr("no", &attributes[..])
                                        .map(|s| parse_i16(s).unwrap())
                                        .unwrap(),
                                    name: attr("name", &attributes[..]).unwrap(),
                                })
                            } else if attr("name", &attributes[..]).unwrap() == "F" {
                                Some(Sax3DanceKey {
                                    number: 1,
                                    name: "R".to_owned(),
                                })
                            } else {
                                None
                            };
                        }
                        "Couple" => {
                            couple = Some({
                                couple_points = attr("points", &attributes[..]).map(|s| {
                                    s.replace(',', ".")
                                        .parse::<f64>()
                                        // .context(format!("{s}"))
                                        .context(s.to_string())
                                        .unwrap()
                                });
                                Couple {
                                    n: attr("n", &attributes[..])
                                        .map(|s| s.parse::<i16>().unwrap())
                                        .unwrap(),
                                    class: attr("class", &attributes[..]),
                                    place: {
                                        let ret = attr("place", &attributes[..])
                                            .map(|s| {
                                                s.parse::<i16>().context(s.to_string()).unwrap()
                                            })
                                            .unwrap();
                                        (ret > 0).then_some(ret)
                                    },
                                }
                            });
                        }
                        "Title" => {
                            date = attr("dateComp", &attributes[..])
                                .map(|s| NaiveDate::parse_from_str(&s, "%d.%m.%Y").unwrap());
                            in_title = true;
                        }
                        "Group" => {
                            in_group = true;
                        }
                        "Round" => {
                            round_number = attr("no", &attributes[..])
                                .map(|s| s.parse::<i16>().context(s.to_string()).unwrap());
                            round = Some(Sax3Round {
                                name: attr("name", &attributes[..]).unwrap(),
                                board_point: {
                                    let ret = attr("boardPoint", &attributes[..])
                                        .map(|s| s.parse::<i16>().context(s.to_string()).unwrap())
                                        .unwrap();
                                    (ret > 0).then_some(ret)
                                },
                                mode: match attr("mode", &attributes[..]).as_deref() {
                                    Some("ball") => Sax3RoundMode::Ball,
                                    Some("skating") => Sax3RoundMode::Skating,
                                    Some("sum") => Sax3RoundMode::Sum,
                                    s => unreachable!("{s:?}"),
                                },
                                judges: None,
                                total_results: HashMap::new(),
                                result_details: HashMap::new(),
                            })
                        }
                        "Judge" => {
                            let judges = if let Some(Sax3Round { judges, .. }) = round.as_mut() {
                                judges
                            } else {
                                &mut judges
                            };
                            if judges.is_none() {
                                *judges = Some(Sax3Judges::default());
                            }
                            let judges = if let Some(judges) = judges.as_mut() {
                                judges
                            } else {
                                unreachable!();
                            };
                            let judge_id = {
                                let s = attr("id", &attributes[..]).unwrap();
                                let Some(caps) = RE_JUDGE_STATUS.captures(&s) else {
                                            panic!("Judge@id={s:?}, does not match re'{STR_RE_JUDGE_STATUS}'");
                                        };
                                let id = caps
                                    .name("id")
                                    .expect("id")
                                    .as_str()
                                    .parse::<i16>()
                                    .expect("i16");
                                match caps.name("code").expect("code").as_str() {
                                    "!" => Sax3JudgeId::Main(id),
                                    "ЗГС" => Sax3JudgeId::Deputy(id),
                                    code => Sax3JudgeId::Private(id, code.chars().next().unwrap()),
                                }
                            };
                            common_macros2::entry!(judges.0, judge_id
                            =>
                                and_modify |_e| {
                                    panic!("already {judge_id:?}");
                                }
                                or_insert Sax3Judge {
                                    first_name: attr("firstName", &attributes[..]).unwrap().trim().to_owned(),
                                    second_name: attr("fatherName", &attributes[..]).unwrap().trim().to_owned(),
                                    last_name: attr("lastName", &attributes[..]).unwrap().trim().to_owned(),
                                    city: attr("city", &attributes[..]).map(|s| s.trim().to_owned()),
                                    club: attr("club", &attributes[..]).map(|s| s.trim().to_owned()),
                                    book_number: attr("bookNumber", &attributes[..])
                                        .and_then(|s| s.parse::<i16>().ok()),
                                    category: attr("category", &attributes[..]).map(|s| s.trim().to_owned()),
                                }
                            );
                        }
                        "Result" => {
                            let couple_number = attr("n", &attributes[..])
                                .map(|n| n.parse::<i16>().unwrap())
                                .unwrap();
                            let sum = attr("sum", &attributes[..]).map(|s| parse_float(s).unwrap());
                            let place =
                                attr("place", &attributes[..]).map(|s| parse_float(s).unwrap());
                            if dance_key.is_some() {
                                dance_pre_res = Some(Sax3RoundPreRes {
                                    place,
                                    sum,
                                    couple_number,
                                });
                            } else {
                                total_results
                                    .insert(couple_number, Sax3RoundTotalRes { sum, place });
                            }
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name }) => match name.local_name.as_str() {
                    "Title" => {
                        in_title = false;
                    }
                    "Group" => {
                        in_group = false;
                    }
                    "Dance" => {
                        dance_key = None;
                    }
                    "Total" => {
                        if let Some(round) = round.as_mut() {
                            std::mem::swap(&mut round.total_results, &mut total_results);
                        }
                    }
                    "Couple" => {
                        let Couple {
                            n: number,
                            place,
                            class,
                        } = couple.take().unwrap();
                        let male = male.take().unwrap();
                        let mut female = female.take();
                        let need_drop_female = if let Some(female) = female.as_mut() {
                            female.first_name.is_empty() && female.last_name.is_empty()
                        } else {
                            false
                        };
                        if need_drop_female {
                            female = None;
                        }
                        let couple = Sax3Couple {
                            male,
                            female,
                            club: club.take().unwrap(),
                            place,
                            class,
                        };
                        couples.insert(number, couple);
                    }
                    "Round" => {
                        rounds.insert(round_number.take().unwrap(), round.take().unwrap());
                    }
                    "GroupData" => {
                        let ret_key = ret_key.clone().unwrap();
                        let category = category.take().unwrap();
                        let mut compet_couples = HashMap::new();
                        let mut compet_rounds = HashMap::new();
                        let mut compet_judges = None;
                        std::mem::swap(&mut compet_couples, &mut couples);
                        std::mem::swap(&mut compet_rounds, &mut rounds);
                        std::mem::swap(&mut compet_judges, &mut judges);
                        let compet = Sax3Compet {
                            category,
                            couples: compet_couples,
                            rounds: compet_rounds,
                            judges: compet_judges,
                        };
                        common_macros2::entry!(ret, ret_key.clone()
                        =>
                            and_modify |e| {
                                e.push(compet);
                            }
                            or_insert vec![compet]
                        );
                    }
                    _ => {}
                },
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    }
    Ok(ret)
}

use xml::attribute::OwnedAttribute;
fn attr(local_name: &str, attributes: &[OwnedAttribute]) -> Option<String> {
    attributes
        .iter()
        .find(|attr| attr.name.local_name == local_name)
        .map(|attr| attr.value.clone())
}

fn parse_float(s: String) -> Result<f64> {
    s.replace(',', ".")
        .parse::<f64>()
        .map_err(|err| anyhow!("{s:?}: {err}"))
}

fn parse_i16(s: String) -> Result<i16> {
    s.parse::<i16>().map_err(|err| anyhow!("{s:?}: {err}"))
}

// ==============================================================================
