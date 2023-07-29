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

#[derive(Debug, Clone, Copy)]
pub enum RoundMode {
    Ball,
    Skating,
    Sum,
}
#[derive(Debug, Clone)]
pub struct Couple {
    pub n: Option<i64>,
    pub points: Option<f64>,
    pub place: Option<i64>,
}

// ==============================================================================

use xml::attribute::OwnedAttribute;
fn attr(local_name: &str, attributes: &[OwnedAttribute]) -> Option<String> {
    attributes
        .iter()
        .find(|attr| attr.name.local_name == local_name)
        .map(|attr| attr.value.clone())
}

// ==============================================================================
//
pub type SaxRet = HashMap<String, Vec<SaxDancer>>;
pub type SaxResult = Result<SaxRet>;
pub fn sax(file_pathlar: &Vec<PathBuf>) -> SaxResult {
    let mut ret: SaxRet = HashMap::new();
    for file_path in file_pathlar {
        let file = File::open(file_path)?;
        let file_stem = file_path.file_stem().unwrap().to_str().unwrap();
        let file = BufReader::new(file);
        let mut dancers = Vec::new();
        let mut dancers_of_group_data = Vec::new();
        let parser = EventReader::new(file);
        let mut title: Option<String> = None;
        let mut date: Option<NaiveDate> = None;
        let mut male = None;
        let mut couple: Option<Couple> = None;
        let mut round_mode: Option<RoundMode> = None;
        let mut category: Option<String> = None;
        #[derive(Debug, Clone, Hash, PartialEq, Eq)]
        struct Dancer {
            first_name: Option<String>,
            last_name: Option<String>,
            class: Option<String>,
            book_number: Option<String>,
            birth_day: Option<String>,
        }
        let mut female = None;
        let mut club = None;
        #[derive(Debug, Clone, Hash, PartialEq, Eq)]
        struct Club {
            city: Option<String>,
            name: Option<String>,
            chief1_last_name: Option<String>,
            chief1_first_name: Option<String>,
            chief2_last_name: Option<String>,
            chief2_first_name: Option<String>,
            trener1_last_name: Option<String>,
            trener1_first_name: Option<String>,
            trener2_last_name: Option<String>,
            trener2_first_name: Option<String>,
        }
        let mut in_group = false;
        let mut in_title = false;
        for e in parser {
            match e {
                Ok(XmlEvent::Characters(s)) => {
                    if in_group {
                        category = Some(s);
                    } else if in_title {
                        title = Some(s);
                    }
                }
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if couple.is_some() {
                        match name.local_name.as_str() {
                            "Male" => {
                                male = Some(Dancer {
                                    first_name: attr("firstName", &attributes[..]),
                                    last_name: attr("lastName", &attributes[..]),
                                    class: attr("class", &attributes[..]),
                                    book_number: attr("bookNumber", &attributes[..]),
                                    birth_day: attr("birthDay", &attributes[..]),
                                    // couple: couple.clone(),
                                })
                            }
                            "Female" => {
                                female = Some(Dancer {
                                    first_name: attr("firstName", &attributes[..]),
                                    last_name: attr("lastName", &attributes[..]),
                                    class: attr("class", &attributes[..]),
                                    book_number: attr("bookNumber", &attributes[..]),
                                    birth_day: attr("birthDay", &attributes[..]),
                                    // couple: couple.clone(),
                                })
                            }
                            "Club" => {
                                club = Some(Club {
                                    city: attr("city", &attributes[..]),
                                    name: attr("name", &attributes[..]),
                                    chief1_last_name: attr("chief1LastName", &attributes[..]),
                                    chief1_first_name: attr("chief1FirstName", &attributes[..]),
                                    chief2_last_name: attr("chief2LastName", &attributes[..]),
                                    chief2_first_name: attr("chief2FirstName", &attributes[..]),
                                    trener1_last_name: attr("trener1LastName", &attributes[..]),
                                    trener1_first_name: attr("trener1FirstName", &attributes[..]),
                                    trener2_last_name: attr("trener2LastName", &attributes[..]),
                                    trener2_first_name: attr("trener2FirstName", &attributes[..]),
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
                        "Round" => {
                            round_mode = Some(match attr("mode", &attributes[..]).as_deref() {
                                Some("ball") => RoundMode::Ball,
                                Some("skating") => RoundMode::Skating,
                                Some("sum") => RoundMode::Sum,
                                s => unreachable!("{s:?}"),
                            })
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name }) => match name.local_name.as_str() {
                    "Title" => {
                        in_title = false;
                    }
                    "Couple" => {
                        let couple = couple.take();
                        let club = club.take();
                        if let Some(Dancer {
                            first_name,
                            last_name,
                            class,
                            book_number,
                            birth_day,
                        }) = male.take()
                        {
                            let Club {
                                city,
                                name,
                                chief1_last_name,
                                chief1_first_name,
                                chief2_last_name,
                                chief2_first_name,
                                trener1_last_name,
                                trener1_first_name,
                                trener2_last_name,
                                trener2_first_name,
                            } = club.clone().unwrap();
                            let couple = couple.clone();
                            let dancer = SaxDancer {
                                first_name,
                                last_name,
                                class,
                                book_number,
                                birth_day,
                                city,
                                club: name,
                                chief1_last_name,
                                chief1_first_name,
                                chief2_last_name,
                                chief2_first_name,
                                trener1_last_name,
                                trener1_first_name,
                                trener2_last_name,
                                trener2_first_name,
                                couple,
                                ..SaxDancer::default()
                            };
                            dancers_of_group_data.push(dancer);
                        }
                        if let Some(Dancer {
                            first_name,
                            last_name,
                            class,
                            book_number,
                            birth_day,
                        }) = female.take()
                        {
                            let Club {
                                city,
                                name,
                                chief1_last_name,
                                chief1_first_name,
                                chief2_last_name,
                                chief2_first_name,
                                trener1_last_name,
                                trener1_first_name,
                                trener2_last_name,
                                trener2_first_name,
                            } = club.clone().unwrap();
                            let couple = couple.clone();
                            let dancer = SaxDancer {
                                first_name,
                                last_name,
                                class,
                                book_number,
                                birth_day,
                                city,
                                club: name,
                                chief1_last_name,
                                chief1_first_name,
                                chief2_last_name,
                                chief2_first_name,
                                trener1_last_name,
                                trener1_first_name,
                                trener2_last_name,
                                trener2_first_name,
                                couple,
                                ..SaxDancer::default()
                            };
                            dancers_of_group_data.push(dancer);
                        }
                    }
                    "GroupData" => {
                        let mut vec = vec![];
                        std::mem::swap(&mut vec, &mut dancers_of_group_data);
                        let round_mode = round_mode.take().unwrap();
                        let category = category.take();
                        let title = title.take();
                        let date = date.take();
                        if !matches!(round_mode, RoundMode::Sum) {
                            for mut dancer in vec {
                                let couple = dancer.couple.take().unwrap();
                                dancer.category = category.clone();
                                dancer.title = title.clone();
                                dancer.date = date;
                                match round_mode {
                                    RoundMode::Ball => dancer.ball = couple.points,
                                    RoundMode::Skating => {
                                        if file_stem == "smm" {
                                            dancer.ball = Some(1f64);
                                        } else {
                                            dancer.place = couple.place;
                                            let category =
                                                category.as_deref().unwrap().to_lowercase();
                                            if category.contains("двоеборье") {
                                                dancer.st_la_score = couple.points;
                                            } else if category.contains("европей") {
                                                dancer.st_score = couple.points;
                                            } else if category.contains("латиноамериканск")
                                            {
                                                dancer.la_score = couple.points;
                                            } else {
                                                unreachable!("{category}");
                                            }
                                        }
                                    }
                                    _ => {
                                        unreachable!();
                                    }
                                }
                                dancer.n = couple.n;
                                dancers.push(dancer);
                            }
                        }
                    }
                    "Group" => {
                        in_group = false;
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
        ret.insert(
            file_path.file_stem().unwrap().to_str().unwrap().to_string(),
            dancers,
        );
    }
    Ok(ret)
}
