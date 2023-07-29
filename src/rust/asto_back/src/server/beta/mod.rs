use super::*;

pub fn api() -> impl Filter<
    Extract = (
        impl Reply, /* https://github.com/seanmonstar/warp/issues/646 */
    ),
    Error = Rejection,
> + Copy {
    warp::get()
        .and(warp::path("beta"))
        .and(warp::path::end())
        .and(
            warp::query()
                .or(warp::any().map(QueryParams::default))
                .unify(),
        )
        .and_then(handle)
}

// https://maud.lambda.xyz/faq.html
use crate::pasitos::sax3::*;
use crate::pasitos::spreadsheet::sax3::utils::*;
use maud::{html, Markup, DOCTYPE}; // ,PreEscaped
use std::collections::{HashMap, HashSet};

#[derive(Deserialize, Default)]
struct QueryParams {
    fest: Option<String>,
}

use crate::pasitos::data::*;

use strum::IntoEnumIterator;
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, strum::Display, strum::EnumIter)]
enum CategoryClass {
    N,
    E,
    D,
    C,
    B,
    O, // Open
    U, // Unknown
}

lazy_static::lazy_static! {
    pub static ref JUDGES_CACHE: std::sync::RwLock<Option<(Markup, String)>> = std::sync::RwLock::new(None);
    pub static ref MAIN_CACHE: std::sync::RwLock<Option<(Markup, String)>> = std::sync::RwLock::new(None);
    pub static ref FEST_CACHE: std::sync::RwLock<HashMap<String, (Markup, String)>> = std::sync::RwLock::new(HashMap::new());
}

async fn handle(QueryParams { fest }: QueryParams) -> std::result::Result<impl Reply, Rejection> {
    let (body, title) = if let Some(fest) = fest {
        if fest.starts_with("judg") {
            let ret = (*JUDGES_CACHE.read().unwrap()).clone();
            if let Some(ret) = ret {
                ret
            } else {
                let ret = judges_body_title().await.map_err(|err| {
                    warp::reject::custom(crate::server::common::error::Error::Anyhow(anyhow!(
                        "{err}"
                    )))
                })?;
                *JUDGES_CACHE.write().unwrap() = Some(ret.clone());
                ret
            }
        } else {
            let ret = (*FEST_CACHE.read().unwrap()).get(&fest).cloned();
            if let Some(ret) = ret {
                ret
            } else {
                let ret = fest_body_title(&fest).await.map_err(|err| {
                    warp::reject::custom(crate::server::common::error::Error::Anyhow(anyhow!(
                        "{err}"
                    )))
                })?;
                FEST_CACHE.write().unwrap().insert(fest, ret.clone());
                ret
            }
        }
    } else {
        let ret = (*MAIN_CACHE.read().unwrap()).clone();
        if let Some(ret) = ret {
            ret
        } else {
            let ret = main_body_title().await.map_err(|err| {
                warp::reject::custom(crate::server::common::error::Error::Anyhow(anyhow!(
                    "{err}"
                )))
            })?;
            *MAIN_CACHE.write().unwrap() = Some(ret.clone());
            ret
        }
    };

    Ok(warp::reply::html(String::from(html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=1, minimum-scale=1, user-scalable=no, viewport-fit=auto";
                link rel="icon" type="image/x-icon" href="css/assets/logo32.png";
                link rel="stylesheet" href="css/index.css";
                title { (title) }
            }
            body {
                (body)
            }
        }
    })))
}

fn render_toc(
    dancerlar: &FestDancerlar,
    judgelar: &FestJudgelar,
    beginner_compets: &Compets,
    non_beginner_compets: &Compets,
) -> Markup {
    html! {
        h2.toc { }
        (render_toc_dancerlar(dancerlar))
        (render_toc_judgelar(judgelar))
        (render_toc_categorilar(beginner_compets, non_beginner_compets))
    }
}

const DANCERLAR: &str = "Танцоры";
const JUDGELAR: &str = "Судьи";
const CATEGORILAR: &str = "Категории";

fn render_menu() -> Markup {
    html! {
        .menu.outer {
            .inner {
                a.dancerlar href=(format!("#{}", translit(DANCERLAR))) { }
                a.judgelar href=(format!("#{}", translit(JUDGELAR))) { }
                a.categorilar href=(format!("#{}", translit(CATEGORILAR))) { }
            }
        }
    }
}

fn render_toc_categorilar(beginner_compets: &Compets, non_beginner_compets: &Compets) -> Markup {
    html! {
        @if !beginner_compets.is_empty() || !non_beginner_compets.is_empty() {
            h3.toc.categorilar #(translit(CATEGORILAR)) { }
            ul.toc.categorilar {
                @if !beginner_compets.is_empty() {
                    li.beginner_compets {
                        (count_compets(beginner_compets.len()))
                        ul { (render_toc_compets(beginner_compets)) }
                    }
                }
                @if !non_beginner_compets.is_empty() {
                    li.non_beginner_compets {
                        (count_compets(non_beginner_compets.len()))
                        ul { (render_toc_compets(non_beginner_compets)) }
                    }
                }
            }
        }
    }
}

fn render_toc_dancerlar(dancerlar: &FestDancerlar) -> Markup {
    html! {
        h3.toc.dancerlar #(translit(DANCERLAR)) {
            (count(dancerlar.len(),
                r#"человек"#,
                r#"человека"#,
                r#"человек"#,
            ))
        }

        .outer {
            .inner {
                table.toc.dancerlar #(translit(DANCERLAR)) {
                    tbody {
                        @for (i, (_, dancer_categorilar)) in dancerlar.iter().enumerate() {
                            @let even_odd_css_class = if (i + 1) % 2 == 0 { "even" } else { "odd" };
                            @let is_many = dancer_categorilar.len() > 1;
                            @for (j, (FestDancer { first_name, last_name, book_number, birth_day, class, club}, categorilar)) in dancer_categorilar.iter().enumerate() {
                                tr.(even_odd_css_class).many[is_many] {
                                    td.alarm { }
                                    td.categorilar {
                                        a href=(format!("#{}", dancer_id(i, j))) {
                                            (count(categorilar.len(),
                                                r#"категория ("старт")"#,
                                                r#"категории ("старта")"#,
                                                r#"категорий ("стартов")"#,
                                            ))
                                        }
                                    }
                                    (render_dancer_name_class(book_number, last_name, first_name, class, birth_day))
                                    (render_dancer_club(club))
                                }
                            }
                        }
                    }
                }
            }
        }

        h4.toc.dancerlar_categorilar { }

        @for (i, (_, dancer_categorilar)) in dancerlar.iter().enumerate() {
            @let even_odd_css_class = if (i + 1) % 2 == 0 { "even" } else { "odd" };
            @let is_many = dancer_categorilar.len() > 1;
            @for (j, (FestDancer { first_name, last_name, book_number, birth_day, class, club}, categorilar)) in dancer_categorilar.iter().enumerate() {
                .toc.dancerlar_categorilar #(dancer_id(i, j)) {
                    .outer {
                        .inner {
                            table {
                                tbody {
                                    tr.(even_odd_css_class).many[is_many] {
                                        td.alarm { }
                                        (render_dancer_name_class(book_number, last_name, first_name, class, birth_day))
                                        (render_dancer_club(club))
                                        td.categorilar {
                                            (count(categorilar.len(),
                                                r#"категория ("старт")"#,
                                                r#"категории ("старта")"#,
                                                r#"категорий ("стартов")"#,
                                            ))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ul {
                        @for (category, couples_len) in categorilar {
                            (render_toc_categori(category, None, *couples_len, None))
                        }
                    }
                }
            }
        }
    }
}

fn judge_id(i: usize, j: usize) -> String {
    id_helper(i, j, "judge")
}

fn dancer_id(i: usize, j: usize) -> String {
    id_helper(i, j, "dancer")
}

fn id_helper(i: usize, j: usize, entity: &str) -> String {
    translit(&{
        let mut ret = format!("{entity}-{i}");
        if j > 0 {
            ret.push_str(&format!("-{j}"))
        }
        ret
    })
}

fn render_toc_judgelar(judgelar: &FestJudgelar) -> Markup {
    html! {
        h3.toc.judgelar #(translit(JUDGELAR)) {
            (count(judgelar.len(),
                r#"человек"#,
                r#"человека"#,
                r#"человек"#,
            ))
        }

        .outer {
            .inner {
                table.toc.judgelar {
                    tbody {
                        @for (i, (_, judge_categorilar)) in judgelar.iter().enumerate() {
                            @let even_odd_css_class = if (i + 1) % 2 == 0 { "even" } else { "odd" };
                            @let is_many = judge_categorilar.len() > 1;
                            @for (j, (judge, categorilar)) in judge_categorilar.iter().enumerate() {
                                tr.(even_odd_css_class).many[is_many] {
                                    td.alarm { }
                                    td.categorilar {
                                        a href=(format!("#{}", judge_id(i, j))) {
                                            (count(categorilar.len(),
                                                r#"категория ("старт")"#,
                                                r#"категории ("старта")"#,
                                                r#"категорий ("стартов")"#,
                                            ))
                                        }
                                    }
                                    (render_judge(judge))
                                    // (render_judge_name_class(book_number, last_name, first_name, class, birth_day))
                                    // (render_judge_club(club))
                                }
                            }
                        }
                    }
                }
            }
        }

        h4.toc.judgelar_categorilar { }

        @for (i, (_, judge_categorilar)) in judgelar.iter().enumerate() {
            @let even_odd_css_class = if (i + 1) % 2 == 0 { "even" } else { "odd" };
            @let is_many = judge_categorilar.len() > 1;
            @for (j, (judge, categorilar)) in judge_categorilar.iter().enumerate() {
                .toc.judgelar_categorilar #(judge_id(i, j)) {
                    .outer {
                        .inner {
                            table {
                                tbody {
                                    tr.(even_odd_css_class).many[is_many] {
                                        td.alarm { }
                                        (render_judge(judge))
                                        td.categorilar {
                                            (count(categorilar.len(),
                                                r#"категория ("старт")"#,
                                                r#"категории ("старта")"#,
                                                r#"категорий ("стартов")"#,
                                            ))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ul {
                        @for (category, tour_name_opt, couples_len) in categorilar {
                            (render_toc_categori(category, tour_name_opt.as_deref(), *couples_len, None))
                        }
                    }
                }
            }
        }
    }
}

fn render_toc_compets(compets: &Compets) -> Markup {
    html! {
        @for (_, Sax3Compet { category, couples, ..}) in compets {
            (render_toc_categori(category, None, couples.len(), None))
        }
    }
}

fn render_toc_categori(
    category: &str,
    tour_name_opt: Option<&str>,
    couples_len: usize,
    ret_key: Option<&Sax3RetKey>,
) -> Markup {
    html! {
        li {
            a href=(
                if ret_key.is_none() {
                    format!("#{}", categori_tour_id(category, tour_name_opt, ret_key))
                } else {
                    categori_tour_id(category, tour_name_opt, ret_key)
                }
            ) {
                (category)
            }
            (count(couples_len,
                "пара (участник)",
                "пары (участника)",
                "пар (участников)",
            ))
        }
    }
}

fn categori_tour_id(
    category: &str,
    tour_name_opt: Option<&str>,
    ret_key: Option<&Sax3RetKey>,
) -> String {
    let hash_value = {
        let mut h = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        category.hash(&mut h);
        if let Some(tour_name) = tour_name_opt {
            tour_name.hash(&mut h);
        }
        let h = h.finish();
        let n = 8;
        let mut s = String::with_capacity(2 * n);
        use std::fmt::Write;
        for byte in h.to_be_bytes() {
            let _ = write!(s, "{:02X}", byte);
        }
        s
    };

    if let Some(ret_key) = ret_key {
        format!(
            "{}/asto_back/beta?fest={}#{}",
            OP_MODE.read().unwrap().route_prefix(),
            ret_key.date,
            hash_value
        )
    } else {
        hash_value
    }
}

fn count_compets(value: usize) -> Markup {
    count(
        value,
        r#"категория ("старт")"#,
        r#"категории ("старта")"#,
        r#"категорий ("стартов")"#,
    )
}
fn count(count: usize, one: &str, two: &str, five: &str) -> Markup {
    html! {
        .count { (count) }
        .count_plural {
            (common_macros2::plural!(count,
                1 one,
                2 two,
                5 five,
            ))
        }
    }
}

fn translit(s: &str) -> String {
    (*TRANSLITERATOR.read().unwrap()).convert(s, false)
}

lazy_static::lazy_static! {
    pub static ref TRANSLITERATOR: std::sync::RwLock<translit::Transliterator> = std::sync::RwLock::new(translit::Transliterator::new(translit::gost779b_ru()));
}

fn render_name_of_dancer_depending_on_book_number(
    book_number: &Option<i32>,
    last_name: &str,
    first_name: &str,
) -> Markup {
    html! {
        @if let Some(book_number) = book_number {
            @if (5500001..=5599999).contains(book_number) {
                a target="_blank" href=(format!("https://asto.dance/{}", route_to_url(&Route::Guest(GuestRoute {
                    did_press: false,
                    search: Some(book_number.to_string()),
                    kind: GuestRouteKind::Dancer {
                        sort_by: DancerSortBy::Name,
                        expanded: HashSet::new(),
                    }
                })))) {
                    (render_name_of_dancer_depending_on_book_number_helper(last_name, first_name))
                }
            } @else {
                (render_name_of_dancer_depending_on_book_number_helper(last_name, first_name)) "::" (book_number)
            }
        } @else {
            (render_name_of_dancer_depending_on_book_number_helper(last_name, first_name))
        }
    }
}

fn render_name_of_dancer_depending_on_book_number_helper(
    last_name: &str,
    first_name: &str,
) -> Markup {
    html! {
         (last_name) " " (first_name)
    }
}

fn render_compets(compets: &[(CompetKind, Sax3Compet)]) -> Markup {
    html! {
        @for (compet_kind, Sax3Compet { category, couples, rounds, judges }) in compets {
            @let round_kind = if matches!(compet_kind, CompetKind::Кубок) {
                RoundKind::BeginnersCup
            } else {
                RoundKind::Skating
            };
            @let mut rounds_peekable = rounds.iter().map(|(key, value)|(*key, (*value).clone())).peekable();
            @let (
                _round_number,
                Sax3Round {
                    name: _,
                    board_point: _,
                    mode,
                    judges: _,
                    total_results: _,
                    result_details,
                },
            ) = rounds_peekable.next().unwrap();
            @let points_kind = match mode {
                Sax3RoundMode::Ball => PointsKind::Ball,
                Sax3RoundMode::Skating | Sax3RoundMode::Sum => {
                    if matches!(round_kind, RoundKind::BeginnersCup) {
                        PointsKind::Ball
                    } else {
                        let mut ret = None;
                        for (Sax3DanceKey { name, .. }, _) in result_details.iter() {
                            ret = match (ret, name.as_str()) {
                                (None | Some(PointsKind::ScoreSt), "W" | "T" | "V" | "F" | "Q") => {
                                    Some(PointsKind::ScoreSt)
                                }
                                (None | Some(PointsKind::ScoreLa), "S" | "Ch" | "R" | "P" | "J") => {
                                    Some(PointsKind::ScoreLa)
                                }
                                (
                                    Some(_),
                                    "W" | "T" | "V" | "F" | "Q" | "S" | "Ch" | "R" | "P" | "J",
                                ) => Some(PointsKind::ScoreStLa),
                                (_, "Polka") => {
                                    Some(PointsKind::Ball)
                                }
                                _ => unreachable!("Dance name={name:?}, ret: {ret:?}, mode: {mode:?}, round_kind: {round_kind:?}"),
                            }
                        }
                        ret.unwrap()
                    }
                }
            };
            h3 #(categori_tour_id(category, None, None)) {
                @if category.starts_with(&compet_kind.to_string()) {
                    (category)
                } @else {
                    (compet_kind) "::" (category)
                }
                .debug {
                    label.round_kind { }
                    .value { (&format!(r#"{round_kind:?}"#)) }

                    label.mode { }
                    .value { (&format!(r#"{mode:?}"#)) }

                    label.points_kind { }
                    .value { (&format!(r#"{points_kind:?}"#)) }
                }
            }
            @let sorted_couples = get_sorted_couples(couples);
            .couples_judges {
                (render_couples(&sorted_couples, *compet_kind, points_kind))
                (render_judges(judges.as_ref()))
            }
            (render_rounds(category, judges.as_ref(), rounds, *compet_kind, &sorted_couples))
        }
    }
}

fn render_rounds(
    category: &str,
    judges: Option<&Sax3Judges>,
    rounds: &HashMap<i16, Sax3Round>,
    compet_kind: CompetKind,
    // couple_numbers: &[i16],
    sorted_couples: &[(i16, Sax3Couple)],
) -> Markup {
    html! {
        @let round_count = rounds.len();
        @for (i, round) in get_sorted_rounds(rounds).into_iter().enumerate() {
            @let is_place = !matches!(compet_kind, CompetKind::Аттестация) && i == round_count - 1;
            @if round_count > 1 {
                h4 #(categori_tour_id(category, Some(&round.name), None)){ (round.name) }
            }

            (render_judges(
                match (judges.as_ref(), round.judges.as_ref()) {
                    (Some(_), _) => None,
                    (None, ret) => ret,
                }
            ))

            @let judges = round.judges.as_ref().or(judges).unwrap();
            @if sorted_couples.len() == 1 && round.result_details.len() > 1 {
                @let dance_key = round.result_details.keys().map(|Sax3DanceKey { name, .. }| name.clone()).collect::<Vec<String>>().join(", ");
                @let dance_res = round.result_details.values().next().unwrap();
                (render_dance_res(dance_key, dance_res, judges, sorted_couples, is_place, true))
            } @else {
                @let sorted_result_details = get_sorted_result_details(round.result_details);
                @if sorted_result_details.len() > 1 {
                    (render_compound_dance_res(&sorted_result_details, judges, sorted_couples, is_place))
                } @else {
                    @for (dance_key, dance_res) in sorted_result_details {
                        (render_dance_res(dance_key, &dance_res, judges, sorted_couples, is_place, false))
                    }
                }
            }
        }
    }
}

fn render_dance_res(
    dance_key: String,
    dance_res: &HashMap<i16, Sax3RoundDanceRes>,
    judges: &Sax3Judges,
    // couple_numbers: &[i16],
    sorted_couples: &[(i16, Sax3Couple)],
    is_place: bool,
    is_all: bool,
) -> Markup {
    html! {
        .outer {
            .inner {
                table.dance_res.all[is_all] {
                    caption { (dance_key) }
                    thead {
                        tr {
                            th.couple_number { }
                            @if is_place {
                                th.place { }
                            } @else {
                                th.sum { }
                            }
                            (judge_thlar(judges, None))
                        }
                    }
                    tbody {
                        @for (couple_number, Sax3RoundDanceRes { sum, place, details}) in sorted_couples.iter().filter_map(|(couple_number, _)| dance_res.get(couple_number).map(|dance_res| (couple_number, dance_res))) {
                            tr {
                                td.couple_number { (couple_number) }
                                @if is_place {
                                    td.place {
                                        (place.map(|i| i.to_string()).unwrap_or_default())
                                    }
                                } @else {
                                    td.sum {
                                        (sum.map(|i| i.to_string()).unwrap_or_default())
                                    }
                                }
                                @for id in get_sorted_judges(judges).iter().filter_map(|(judge_id, _)| if let Sax3JudgeId::Private(id, _) = judge_id { Some(id) } else { None } ) {
                                    td.judge_id {
                                        @match details {
                                            Sax3RoundDanceResDetails::Crosses(crosses) => {
                                                (if crosses.contains(id) { "X" } else { "" })
                                            },
                                            Sax3RoundDanceResDetails::Places(places) => {
                                                (places.get(id).map(|i| (i + 1).to_string()).unwrap_or_default())
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn judge_thlar(judges: &Sax3Judges, dance_count: Option<usize>) -> Markup {
    html! {
        @for (ch, name) in get_sorted_judges(judges).iter().filter_map(|(judge_id, judge)| if let Sax3JudgeId::Private(_, ch) = judge_id { Some((ch, judge.last_name.clone())) } else { None } ) {
            th.judge_id colspan=[dance_count] {
                (ch)
                .name { (name) }
            }
        }
    }
}

fn render_compound_dance_res(
    sorted_result_details: &[(String, HashMap<i16, Sax3RoundDanceRes>)],
    judges: &Sax3Judges,
    // couple_numbers: &[i16],
    sorted_couples: &[(i16, Sax3Couple)],
    is_place: bool,
) -> Markup {
    let dance_count = sorted_result_details.len();
    html! {
        .outer {
            .inner {
                table.dance_res.compound {
                    caption { (sorted_result_details.iter().map(|(dance,_)| dance.clone()).collect::<Vec<String>>().join(", ")) }
                    thead {
                        tr {
                            th.couple_number { }

                            th.dance { }

                            th.(if is_place { "place" } else { "sum" })
                                colspan=(dance_count) { }

                            (judge_thlar(judges, Some(dance_count)))
                        }
                    }
                    tbody {
                        @for (i, (couple_number, couple)) in sorted_couples.iter().enumerate() {
                            @let even_odd_css_class = if (i + 1) % 2 == 0 { "even" } else { "odd" };
                            @for (i, (dance, Sax3RoundDanceRes { sum, place, details })) in sorted_result_details.iter().filter_map(|(dance, dance_res)| dance_res.get(couple_number).map(|dance_res| (dance, dance_res))).enumerate() {
                                tr.(even_odd_css_class).(if i == 0 { "first-of-tr-group" } else { "" } ) {
                                    @if i == 0 {
                                        td.couple_number rowspan=(dance_count){
                                            (couple_number)
                                            .male {
                                                (couple.male.last_name)
                                            }
                                            @if dance_count >= 3 {
                                                @if let Some(female) = couple.female.as_ref() {
                                                    .female {
                                                        (female.last_name)
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    td.dance { (dance) }

                                    // =============================
                                    @let (css_class, place_or_sum) =
                                        if is_place {
                                            ("place", (place.map(|i| i.to_string()).unwrap_or_default()))
                                        } else {
                                            ("sum", (sum.map(|i| i.to_string()).unwrap_or_default()))
                                        }
                                    ;
                                    @let aggregated = {
                                        let ret = sorted_result_details.iter().filter_map(|(_, dance_res)|
                                            dance_res.get(couple_number).map(|Sax3RoundDanceRes { sum, place, details: _ }| {
                                            if is_place {
                                                place.map(|i| i.to_string())
                                            } else {
                                                sum.map(|i| i.to_string())
                                            }.unwrap_or_default()
                                        })).collect::<HashSet<_>>();
                                        (ret.len() == 1).then(|| ret.iter().next().unwrap().to_owned())
                                    };
                                    @let mut first_of_col_group_is_set = false;

                                    @if let Some(aggregated) = aggregated {
                                        @if i == 0 {
                                            td.aggregated.(css_class).first_of_col_group[is_first_of_col_group(&mut first_of_col_group_is_set)] rowspan=(dance_count) colspan=(dance_count) {
                                                (aggregated)
                                            }
                                        }
                                    } @else {
                                        @for _ in 0..i {
                                            td.first_of_col_group[is_first_of_col_group(&mut first_of_col_group_is_set)] {}
                                        }
                                        td.(css_class).first_of_col_group[is_first_of_col_group(&mut first_of_col_group_is_set)] { (place_or_sum) }
                                        @for _ in (i + 1)..dance_count { td { }}
                                    }
                                    // =============================

                                    @for id in get_sorted_judges(judges).iter().filter_map(|(judge_id, _)| if let Sax3JudgeId::Private(id, _) = judge_id { Some(id) } else { None } ) {

                                        @let aggregated = {
                                            let ret = sorted_result_details.iter().filter_map(|(_, dance_res)|
                                                dance_res.get(couple_number).map(|Sax3RoundDanceRes { sum: _, place: _, details }| {
                                                    match details {
                                                        Sax3RoundDanceResDetails::Crosses(crosses) => {
                                                            if crosses.contains(id) { "X" } else { "" }.to_owned()
                                                        },
                                                        Sax3RoundDanceResDetails::Places(places) => {
                                                            places.get(id).map(|i| (i + 1).to_string()).unwrap_or_default()
                                                        },
                                                    }
                                            })).collect::<HashSet<_>>();
                                            (ret.len() == 1).then(|| ret.iter().next().unwrap().to_owned())
                                        };
                                        @let mut first_of_col_group_is_set = false;

                                        @if let Some(aggregated) = aggregated {
                                            @if i == 0 {
                                                td.aggregated.first_of_col_group[is_first_of_col_group(&mut first_of_col_group_is_set)] rowspan=(dance_count) colspan=(dance_count) {
                                                    (aggregated)
                                                }
                                            }
                                        } @else {
                                            @for _ in 0..i { td.first_of_col_group[is_first_of_col_group(&mut first_of_col_group_is_set)] {} }
                                            td.judge_id.first_of_col_group[is_first_of_col_group(&mut first_of_col_group_is_set)] {
                                                @match details {
                                                    Sax3RoundDanceResDetails::Crosses(crosses) => {
                                                        (if crosses.contains(id) { "X" } else { "" })
                                                    },
                                                    Sax3RoundDanceResDetails::Places(places) => {
                                                        (places.get(id).map(|i| (i + 1).to_string()).unwrap_or_default())
                                                    },
                                                }
                                            }
                                            @for _ in (i + 1)..dance_count { td {} }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn is_first_of_col_group(first_of_col_group_is_set: &mut bool) -> bool {
    let ret = *first_of_col_group_is_set;
    *first_of_col_group_is_set = true;
    !ret
}

use asto_common::route::*;

fn render_judges(judges: Option<&Sax3Judges>) -> Markup {
    html! {
        @if let Some(judges) = judges {
            .outer {
                .inner {
                    table.judges {
                        caption { }
                        thead {
                            tr {
                                th.judge_id { }
                                th.name { }
                                th.category { }
                                th.club_city { }
                            }
                        }
                        tbody {
                            @for (judge_id, judge) in get_sorted_judges(judges) {
                                tr {
                                    td.judge_id.private[matches!(judge_id, Sax3JudgeId::Private(..))] {
                                        (judge_id)
                                    }
                                    (render_judge(judge))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn render_judge(judge: &Sax3Judge) -> Markup {
    html! {
        td.name {
            @if let Some(book_number) = judge.book_number {
                a target="_blank" href=(format!("https://asto.dance/{}", route_to_url(&Route::Guest(GuestRoute {
                    did_press: false,
                    search: Some(book_number.to_string()),
                    kind: GuestRouteKind::Judge(JudgeSortBy::Name),
                })))) {
                    (judge.last_name) " " (judge.first_name)
                }
            } @else {
                (judge.last_name) " " (judge.first_name)
            }
        }
        td.category {
            (judge.category.as_deref().unwrap_or_default())
        }
        td.club_city {
            @match  (&judge.club, &judge.city) {
                (Some(club), Some(city)) => (club) "::" (city),
                (None, Some(city)) => (city),
                (Some(club), None) => (club),
                (None, None) => "",
            }
        }
    }
}

fn get_sorted_rounds(rounds: &HashMap<i16, Sax3Round>) -> Vec<Sax3Round> {
    let mut rounds: Vec<_> = rounds
        .iter()
        .map(|(key, value)| (*key, (*value).clone()))
        .collect();
    rounds.sort_by_key(|(key, _)| *key);
    rounds.into_iter().map(|(_, value)| value).collect()
}

fn get_sorted_result_details(
    result_details: HashMap<Sax3DanceKey, HashMap<i16, Sax3RoundDanceRes>>,
) -> Vec<(String, HashMap<i16, Sax3RoundDanceRes>)> {
    let mut result_details: Vec<_> = result_details.into_iter().collect();
    result_details.sort_by_key(|(Sax3DanceKey { number, .. }, _)| *number);
    result_details
        .into_iter()
        .map(|(Sax3DanceKey { name, .. }, value)| (name, value))
        .collect()
}

fn render_couples(
    couples: &Vec<(i16, Sax3Couple)>,
    compet_kind: CompetKind,
    points_kind: PointsKind,
) -> Markup {
    html! {
        .outer {
            .inner {
                table.couples {
                    @let is_place = !matches!(compet_kind, CompetKind::Аттестация);
                    @let css_class = match points_kind {
                        PointsKind::Ball => "balllar" ,
                        PointsKind::ScoreStLa => "score_St_La",
                        PointsKind::ScoreSt => "score_St",
                        PointsKind::ScoreLa => "score_La",
                    };
                    caption {}
                    thead {
                        tr {
                            @if is_place {
                                th.place { }
                            }
                            th.couple_number { }
                            th.(css_class) { }
                            th.name { }
                            th.class { }
                            th.birth_day { }
                            th.club_city { }
                            th.chief { }
                        }
                    }
                    tbody {
                        @for (couple_number, couple) in couples {
                            tr.male {
                                @if is_place {
                                    td.place {
                                        (couple.place.map(|i| i.to_string()).unwrap_or_default())
                                    }
                                }
                                td.couple_number { (couple_number) }
                                @let dancer = &couple.male;
                                (render_dancer_points_name_class(dancer, css_class))
                                (render_dancer_club(&couple.club))
                            }
                            @if let Some(dancer) = couple.female.as_ref() {
                                tr.female {
                                    @if !matches!(compet_kind, CompetKind::Аттестация) {
                                        td.place { }
                                    }
                                    td.couple_number { }
                                    (render_dancer_points_name_class(dancer, css_class))
                                }
                            }
                        }

                    }
                }
            }
        }
    }
}

fn render_dancer_points_name_class(
    Sax3Dancer {
        points,
        book_number,
        last_name,
        first_name,
        class,
        birth_day,
        ..
    }: &Sax3Dancer,
    points_css_class: &str,
) -> Markup {
    html! {
        td.(points_css_class) {
            (points.unwrap_or(1f64))
        }
        (render_dancer_name_class(book_number, last_name, first_name, class, birth_day))
    }
}

fn render_dancer_club(club: &Sax3Club) -> Markup {
    html! {
        td.club_city {
            (club.name) "::" (club.city)
        }
        td.chief {
            (club.chief1_last_name.as_deref().unwrap_or_default()) " " (club.chief1_first_name.as_deref().unwrap_or_default())
            .trener {
                @if (&club.chief1_last_name, &club.chief1_first_name) != (&club.trener1_last_name, &club.trener1_first_name) && (club.trener1_last_name.is_some() || club.trener1_first_name.is_some()){
                    ({{
                        let mut ret = String::new();
                        if let Some(s) = club.trener1_last_name.as_deref() {
                            ret.push_str(s.trim());
                        }
                        if let Some(s) = club.trener1_first_name.as_deref() {
                            if !ret.is_empty() {
                                ret.push(' ');
                            }
                            ret.push_str(s.trim());
                        }
                        ret
                    }})
                }
            }
        }
    }
}

fn render_dancer_name_class(
    book_number: &Option<i32>,
    last_name: &str,
    first_name: &str,
    class: &Option<String>,
    birth_day: &Option<chrono::NaiveDate>,
) -> Markup {
    html! {
        td.name {
            (render_name_of_dancer_depending_on_book_number(book_number, last_name, first_name))
        }
        td.class {
            (class.as_deref().unwrap_or_default())
        }
        td.birth_day {
            (birth_day.as_ref().map(|i| i.to_string()).unwrap_or_default())
        }
    }
}

fn get_sorted_judges(Sax3Judges(judges): &Sax3Judges) -> Vec<(&Sax3JudgeId, &Sax3Judge)> {
    let mut judges: Vec<_> = judges.iter().collect();
    judges.sort_by_key(|(id, _value)| *id);
    judges
}

fn get_sorted_couples(couples: &HashMap<i16, Sax3Couple>) -> Vec<(i16, Sax3Couple)> {
    let mut couples: Vec<_> = couples
        .iter()
        .map(|(key, value)| (*key, (*value).clone()))
        .collect();
    use std::cmp::Ordering::*;
    couples.sort_by(|(a_couple_number, a_couple), (b_couple_number, b_couple)| {
        let ret = if let Some(a_place) = a_couple.place {
            if let Some(b_place) = b_couple.place {
                a_place.cmp(&b_place)
            } else {
                Equal
            }
        } else {
            Equal
        };
        if !matches!(ret, Equal) {
            ret
        } else {
            let ret = if let Some(a_points) = a_couple.male.points {
                if let Some(b_points) = b_couple.male.points {
                    a_points.partial_cmp(&b_points).unwrap_or(Equal).reverse()
                } else {
                    Equal
                }
            } else {
                Equal
            };
            if !matches!(ret, Equal) {
                ret
            } else {
                a_couple_number.cmp(b_couple_number)
            }
        }
    });
    couples
}

type FestJudgelarValueOfBTree =
    HashMap<Sax3Judge, BTreeMap<CategoryClass, BTreeMap<Sax3RetKey, Vec<FestJudgelarValueItem>>>>;
use std::collections::BTreeMap;
fn render_judgelar(judgelar: BTreeMap<FestJudgelarKey, FestJudgelarValueOfBTree>) -> Markup {
    html! {
        h3.toc.judgelar #(translit(JUDGELAR)) {
            (count(judgelar.len(),
                r#"человек"#,
                r#"человека"#,
                r#"человек"#,
            ))
        }

        .outer {
            .inner {
                table.toc.judgelar {
                    tbody {
                        @for (i, (_, judge_categorilar)) in judgelar.iter().enumerate() {
                            @let even_odd_css_class = if (i + 1) % 2 == 0 { "even" } else { "odd" };
                            @let is_many = judge_categorilar.len() > 1;
                            @for (j, (judge, categorilar)) in judge_categorilar.iter().enumerate() {
                                tr.(even_odd_css_class).many[is_many] {
                                    td.alarm { }
                                    (render_judge(judge))
                                    @let category_class_len = CategoryClass::iter().count();
                                    @for (k, category_class) in CategoryClass::iter().enumerate() {
                                        td {
                                            @if let Some(festlar) = categorilar.get(&category_class) {

                                                a href=(format!("#{}", judge_category_class_id(i, j, category_class))) {
                                                    (category_class.to_string())
                                                    ": "
                                                    (festlar.len()) "/" (festlar.iter().map(|(_ret_key, categorilar)|categorilar.len()).sum::<usize>())
                                                }
                                            } @else if k < category_class_len - 1 {
                                                " "
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        h4.toc.judgelar_categorilar { }

        @for (i, (_, judge_categorilar)) in judgelar.iter().enumerate() {
            @let even_odd_css_class = if (i + 1) % 2 == 0 { "even" } else { "odd" };
            @let is_many = judge_categorilar.len() > 1;
            @for (j, (judge, categorilar)) in judge_categorilar.iter().enumerate() {
                .toc.judgelar_categorilar #(judge_id(i, j)) {
                    .outer {
                        .inner {
                            table {
                                tbody {
                                    tr.(even_odd_css_class).many[is_many] {
                                        td.alarm { }
                                        (render_judge(judge))
                                        @for category_class in CategoryClass::iter() {
                                            @if let Some(festlar) = categorilar.get(&category_class) {
                                                td {
                                                    a href=(format!("#{}", judge_category_class_id(i, j, category_class))) {
                                                        (category_class.to_string())
                                                        ": "
                                                        (festlar.len()) "/" (festlar.iter().map(|(_ret_key, categorilar)|categorilar.len()).sum::<usize>())
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ul {
                        @for (category_class, festlar) in categorilar {
                            li #(judge_category_class_id(i, j, *category_class)) .judge_category_class {
                                .title {
                                    (category_class.to_string())
                                        ", "
                                        (judge.last_name) " " (judge.first_name)
                                        (count(festlar.len(),
                                            r#"фестиваль"#,
                                            r#"фестиваля"#,
                                            r#"фестивалей"#,
                                        ))
                                        (count(festlar.iter().map(|(_, categorilar)| categorilar.len()).sum::<usize>(),
                                            r#"категория ("старт")"#,
                                            r#"категории ("старта")"#,
                                            r#"категорий ("стартов")"#,
                                        ))
                                }
                                ul {
                                    @for (ret_key, categorilar) in festlar {
                                        li {
                                            (ret_key.to_string())
                                            (count(categorilar.len(),
                                                r#"категория ("старт")"#,
                                                r#"категории ("старта")"#,
                                                r#"категорий ("стартов")"#,
                                            ))
                                            ul {
                                                @for (category, tour_name_opt, couples_len) in categorilar {
                                                    (render_toc_categori(category, tour_name_opt.as_deref(), *couples_len, Some(ret_key)))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn judge_category_class_id(i: usize, j: usize, category_class: CategoryClass) -> String {
    translit(&{
        let mut ret = format!("judge-{i}");
        if j > 0 {
            ret.push_str(&format!("-{j}"));
        }
        ret.push_str(&format!("-{category_class}"));
        ret
    })
}

async fn main_body_title() -> Result<(Markup, String)> {
    let mut ret_keylar = crate::server::ws::common::macros::ws_send_receive!(RequestMessage::FestIndex => ResponseMessage::FestIndex(res) => res)??;
    ret_keylar.sort_by_key(|i| i.date);
    ret_keylar.reverse();
    let title = "Фестивали::asto";
    Ok((
        html! {
            h1 { (title) }
            ul.festlar {
                @for Sax3RetKey { title, date } in ret_keylar {
                    li {
                        a href=(format!("{}/asto_back/beta?fest={date}", OP_MODE.read().unwrap().route_prefix())) {
                            (date) " " (title)
                        }
                    }
                }
            }
            h2 {
                a href=(
                    format!(
                        "{}/asto_back/beta?fest=judges",
                        OP_MODE.read().unwrap().route_prefix(),
                    )
                ) {
                    "Судьи"
                }
            }
        },
        title.to_owned(),
    ))
}

type FestJudgelarValueOfHashMap =
    HashMap<CategoryClass, HashMap<Sax3RetKey, Vec<FestJudgelarValueItem>>>;
async fn judges_body_title() -> Result<(Markup, String)> {
    let data = crate::server::ws::common::macros::ws_send_receive!(RequestMessage::FestJudges => ResponseMessage::FestJudges(res) => res)??;
    let mut judgelar: HashMap<FestJudgelarKey, HashMap<Sax3Judge, FestJudgelarValueOfHashMap>> =
        HashMap::new();
    for i in data {
        for (fest_judgelar_key, value) in i.judgelar.clone() {
            for (sax3_judge, fest_judgelar_value_itemlar) in value {
                for fest_judgelar_value_item in fest_judgelar_value_itemlar {
                    let category = fest_judgelar_value_item.0.to_lowercase();

                    let kind = crate::pasitos::spreadsheet::get_kind_of_category(&category);
                    let category_class = match kind {
                        CompetKind::Кубок | CompetKind::Аттестация => {
                            CategoryClass::N
                        }
                        CompetKind::Категория => {
                            if category.contains("e класс") || category.contains("е класс")
                            {
                                CategoryClass::E
                            } else if category.contains("d класс") || category.contains("д класс")
                            {
                                CategoryClass::D
                            } else if category.contains("c класс") || category.contains("с класс")
                            {
                                CategoryClass::C
                            } else if category.contains("в класс") || category.contains("b класс")
                            {
                                CategoryClass::B
                            } else if category.contains("открытый класс") {
                                CategoryClass::O
                            } else {
                                CategoryClass::U
                            }
                        }
                    };
                    let sax3_judge = sax3_judge.clone();
                    let ret_key = i.ret_key.clone();
                    entry!(judgelar, fest_judgelar_key.clone()
                    =>
                        and_modify |e| {
                            entry!(e, sax3_judge
                            =>
                                and_modify |e| {
                                    entry!(e, category_class
                                    =>
                                        and_modify |e| {
                                            entry!(e, ret_key
                                            =>
                                                and_modify |e| {
                                                    e.push(fest_judgelar_value_item);
                                                }
                                                or_insert vec![fest_judgelar_value_item]
                                            );
                                        }
                                        or_insert HashMap::from([
                                            (ret_key, vec![fest_judgelar_value_item])
                                        ])
                                    )
                                }
                                or_insert HashMap::from([
                                    (category_class, HashMap::from([
                                        (ret_key, vec![fest_judgelar_value_item])
                                    ]))
                                ])
                            );
                        }
                        or_insert HashMap::from([
                            (sax3_judge, HashMap::from([
                                (category_class, HashMap::from([
                                    (ret_key, vec![fest_judgelar_value_item])
                                ]))
                            ]))
                        ])
                    );
                }
            }
        }
    }
    type FestJudgelarValue = HashMap<
        Sax3Judge,
        BTreeMap<CategoryClass, BTreeMap<Sax3RetKey, Vec<FestJudgelarValueItem>>>,
    >;
    let judgelar: BTreeMap<FestJudgelarKey, FestJudgelarValue> = judgelar
        .into_iter()
        .map(|(fest_judgelar_key, value)| {
            (
                fest_judgelar_key,
                value
                    .into_iter()
                    .map(|(sax3_judge, value)| {
                        (
                            sax3_judge,
                            value
                                .into_iter()
                                .map(|(category_class, value)| {
                                    (category_class, value.into_iter().collect())
                                })
                                .collect(),
                        )
                    })
                    .collect(),
            )
        })
        .collect();
    Ok((render_judgelar(judgelar), "Судьи".to_owned()))
}

async fn fest_body_title(fest: &str) -> Result<(Markup, String)> {
    let fest_data = crate::server::ws::common::macros::ws_send_receive!(RequestMessage::Fest(fest.to_owned()) => ResponseMessage::Fest(res) => res)??;
    let title = {
        let Sax3RetKey { date, title } = fest_data.ret_key.clone();
        format!("{date}::{title}")
    };
    Ok((
        html! {
            (render_menu())

            h1 { (title) }

            (render_toc(&fest_data.dancerlar, &fest_data.judgelar, &fest_data.beginner_compets, &fest_data.non_beginner_compets))

            @if !fest_data.beginner_compets.is_empty() {
                h2.beginner_compets { }
                (render_compets(&fest_data.beginner_compets))
            }

            @if !fest_data.non_beginner_compets.is_empty() {
                h2.non_beginner_compets { }
                (render_compets(&fest_data.non_beginner_compets))
            }
        },
        title,
    ))
}
