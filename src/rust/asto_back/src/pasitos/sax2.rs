use super::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use chrono::NaiveDate;
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug, Clone, Default)]
pub struct SaxDancer {
    pub date: Option<NaiveDate>,
    pub title: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub class: Option<String>,
    pub book_number: Option<String>,
    pub birth_day: Option<String>,
    pub city: Option<String>,
    pub club: Option<String>,
    pub chief1_last_name: Option<String>,
    pub chief1_first_name: Option<String>,
    pub chief2_last_name: Option<String>,
    pub chief2_first_name: Option<String>,
    pub trener1_last_name: Option<String>,
    pub trener1_first_name: Option<String>,
    pub trener2_last_name: Option<String>,
    pub trener2_first_name: Option<String>,
    //
    pub n: Option<i64>,
    pub place: Option<i64>,
    pub couple: Option<Couple>,
    //
    pub category: Option<String>,
    pub st_score: Option<f64>,
    pub la_score: Option<f64>,
    pub st_la_score: Option<f64>,
    pub ball: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct Couple {
    pub n: Option<i64>,
    pub points: Option<f64>,
    pub place: Option<i64>,
}

// ==============================================================================

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Sax2DanceKey {
    pub number: i16,
    pub name: String,
}

#[derive(Debug)]
pub struct Sax2RoundTotalRes {
    pub place: i16,
    pub sum: i16,
}

#[derive(Debug)]
pub struct Sax2Couple {
    pub number: i16,
    pub last_tur: i16,
    pub points: f64,
    pub male: Sax2Dancer,
    pub class: Option<String>,
    pub place: i16,
    pub place_down: i16,
    pub class_place: i16,
    pub class_int_req: i16,
    pub female: Sax2Dancer,
    pub club: Sax2Club,
}

#[derive(Debug, Eq)]
pub struct Sax2Dancer {
    pub first_name: String,
    pub last_name: String,
    pub book_number: Option<String>,
    pub birth_day: String,
    pub class: Option<String>,
}

impl Hash for Sax2Dancer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.first_name.hash(state);
        self.last_name.hash(state);
    }
}

impl PartialEq for Sax2Dancer {
    fn eq(&self, other: &Self) -> bool {
        self.first_name == other.first_name && self.last_name == other.last_name
    }
}

#[derive(Debug, Clone, Eq)]
pub struct Sax2Club {
    pub city: String,
    pub name: String,
    pub chief1_last_name: Option<String>,
    pub trener1_last_name: Option<String>,
}

use std::hash::{Hash, Hasher};
impl Hash for Sax2Club {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.city.hash(state);
        self.name.hash(state);
    }
}

impl PartialEq for Sax2Club {
    fn eq(&self, other: &Self) -> bool {
        self.city == other.city && self.name == other.name
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Sax2RetKey {
    title: String,
    date: chrono::NaiveDate,
}
common_macros2::impl_display!(Sax2RetKey, self, "{}::{}", self.date, self.title);
pub type Category = String;
pub type Sax2Ret = HashMap<Sax2RetKey, HashMap<Sax2Club, HashMap<Sax2Dancer, Vec<Category>>>>;
pub type Sax2Result = Result<Sax2Ret>;
pub fn sax2(file_pathlar: &Vec<PathBuf>) -> Sax2Result {
    let mut ret: Sax2Ret = HashMap::new();
    for file_path in file_pathlar {
        let file = File::open(file_path)?;
        let file = BufReader::new(file);
        let parser = EventReader::new(file);
        let mut date: Option<NaiveDate> = None;
        let mut ret_key: Option<Sax2RetKey> = None;
        let mut category: Option<String> = None;
        let mut in_group = false;
        let mut in_title = false;
        let mut couple: Option<Couple> = None;
        let mut club = None;
        let mut male = None;
        let mut female = None;
        for e in parser {
            match e {
                Ok(XmlEvent::Characters(s)) => {
                    if in_group {
                        category = Some(s);
                    } else if in_title {
                        ret_key = Some(Sax2RetKey {
                            title: {
                                let s = s
                                    .strip_suffix(" массовый спорт")
                                    .map(|s| s.to_owned())
                                    .unwrap_or(s);
                                s.strip_suffix(" СВД").map(|s| s.to_owned()).unwrap_or(s)
                            },
                            date: date.take().unwrap(),
                        })
                    }
                }
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if couple.is_some() {
                        match name.local_name.as_str() {
                            "Male" => {
                                male = Some(Sax2Dancer {
                                    first_name: attr("firstName", &attributes[..]).unwrap(),
                                    last_name: attr("lastName", &attributes[..]).unwrap(),
                                    class: attr("class", &attributes[..]),
                                    book_number: attr("bookNumber", &attributes[..]),
                                    birth_day: attr("birthDay", &attributes[..]).unwrap(),
                                })
                            }
                            "Female" => {
                                female = Some(Sax2Dancer {
                                    first_name: attr("firstName", &attributes[..]).unwrap(),
                                    last_name: attr("lastName", &attributes[..]).unwrap(),
                                    class: attr("class", &attributes[..]),
                                    book_number: attr("bookNumber", &attributes[..]),
                                    birth_day: attr("birthDay", &attributes[..]).unwrap(),
                                })
                            }
                            "Club" => {
                                club = Some(Sax2Club {
                                    city: attr("city", &attributes[..]).unwrap(),
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
                                            _ => {
                                                if let Some(s) = s.strip_prefix("ТСК ") {
                                                    s.to_owned()
                                                } else {
                                                    s
                                                }
                                            }
                                        })
                                        .unwrap(),
                                    chief1_last_name: attr("chief1LastName", &attributes[..]).map(
                                        |s| match s.as_str() {
                                            "Муравьева" => "Муравьёва".to_owned(),
                                            "Ветковская" => "Кварталова".to_owned(),
                                            _ => s,
                                        },
                                    ),
                                    trener1_last_name: attr("trener1LastName", &attributes[..])
                                        .map(|s| match s.as_str() {
                                            "Муравьева" => "Муравьёва".to_owned(),
                                            "Ветковская" => "Кварталова".to_owned(),
                                            _ => s,
                                        }),
                                })
                            }
                            s => unreachable!("{s}"),
                        }
                    }
                    match name.local_name.as_str() {
                        "Couple" => {
                            couple = Some(Couple {
                                n: attr("n", &attributes[..]).map(|s| s.parse::<i64>().unwrap()),
                                place: attr("place", &attributes[..])
                                    .map(|s| s.parse::<i64>().context(s.to_string()).unwrap()),
                                points: attr("points", &attributes[..]).map(|s| {
                                    s.replace(',', ".")
                                        .parse::<f64>()
                                        .context(s.to_string())
                                        .unwrap()
                                }),
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
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name }) => {
                    match name.local_name.as_str() {
                        "Title" => {
                            in_title = false;
                        }
                        "Group" => {
                            in_group = false;
                        }
                        "GroupData" => {}
                        "Couple" => {
                            let club = club.take();
                            let ret_key = ret_key.clone().unwrap();
                            if let Some(dancer) = male.take() {
                                let category = category.clone().unwrap();
                                let club = club.clone().unwrap();
                                common_macros2::entry!(ret, ret_key.clone()
                                =>
                                    and_modify |e| {
                                        common_macros2::entry!(e, club
                                        =>
                                            and_modify |e| {
                                                common_macros2::entry!(e, dancer
                                                =>
                                                    and_modify |e| {
                                                        e.push(category)
                                                    }
                                                    or_insert vec![category]
                                                );
                                            }
                                            or_insert vec![(dancer, vec![category])].into_iter().collect()
                                        );
                                    }
                                    or_insert vec![(club, vec![(dancer, vec![category])].into_iter().collect())].into_iter().collect()
                                );
                            }
                            if let Some(dancer) = female.take() {
                                let category = category.clone().unwrap();
                                let club = club.unwrap();
                                // let title = title.clone().unwrap();
                                common_macros2::entry!(ret, ret_key
                                =>
                                    and_modify |e| {
                                        common_macros2::entry!(e, club
                                        =>
                                            and_modify |e| {
                                                common_macros2::entry!(e, dancer
                                                =>
                                                    and_modify |e| {
                                                        e.push(category)
                                                    }
                                                    or_insert vec![category]
                                                );
                                            }
                                            or_insert vec![(dancer, vec![category])].into_iter().collect()
                                        );
                                    }
                                    or_insert vec![(club, vec![(dancer, vec![category])].into_iter().collect())].into_iter().collect()
                                );
                            }
                        }
                        _ => {}
                    }
                }
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

// ==============================================================================
