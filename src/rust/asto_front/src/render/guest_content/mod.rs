use super::*;

mod content;
mod header;

pub use content::DancerScorePeriod;

mod dancer_hash_filter;
use dancer_hash_filter::*;

mod judge_hash_filter;
use judge_hash_filter::*;

mod trainer_hash_filter;
use trainer_hash_filter::*;

mod club_hash_filter;
use club_hash_filter::*;

mod main_filter;
use main_filter::*;

pub fn guest_content() -> Dom {
    html!("div", {
        .class("guest_content")
        .child(header::render())
        .child(content::render())
    })
}

impl App {
    fn last_name_text_signal(person: &Option<Arc<Person>>) -> impl Signal<Item = Option<String>> {
        APP.data
            .last_namelar_map
            .signal_map_cloned()
            .key_cloned(
                person
                    .as_ref()
                    .map(|person| person.last_name)
                    .unwrap_or_default(),
            )
            .map(|last_name| {
                APP.data
                    .textlar_map
                    .signal_map_cloned()
                    .key_cloned(
                        last_name
                            .map(|last_name| last_name.value)
                            .unwrap_or_default(),
                    )
                    .map(|text| text.map(|text| text.value.clone()))
            })
            .flatten()
    }
    fn first_name_text_signal(person: &Option<Arc<Person>>) -> impl Signal<Item = Option<String>> {
        APP.data
            .first_namelar_map
            .signal_map_cloned()
            .key_cloned(
                person
                    .as_ref()
                    .map(|person| person.first_name)
                    .unwrap_or_default(),
            )
            .map(|first_name| {
                APP.data
                    .textlar_map
                    .signal_map_cloned()
                    .key_cloned(
                        first_name
                            .map(|first_name| first_name.value)
                            .unwrap_or_default(),
                    )
                    .map(|text| text.map(|text| text.value.clone()))
            })
            .flatten()
    }
    fn second_name_text_signal(person: &Option<Arc<Person>>) -> impl Signal<Item = Option<String>> {
        APP.data
            .second_namelar_map
            .signal_map_cloned()
            .key_cloned(
                person
                    .as_ref()
                    .map(|person| person.second_name)
                    .unwrap_or_default(),
            )
            .map(|second_name| {
                APP.data
                    .textlar_map
                    .signal_map_cloned()
                    .key_cloned(
                        second_name
                            .map(|second_name| second_name.value)
                            .unwrap_or_default(),
                    )
                    .map(|text| text.map(|text| text.value.clone()))
            })
            .flatten()
    }
    fn nick_name_text_signal(person: &Option<Arc<Person>>) -> impl Signal<Item = Option<String>> {
        APP.data
            .nick_namelar_map
            .signal_map_cloned()
            .key_cloned(
                person
                    .as_ref()
                    .map(|person| person.nick_name)
                    .unwrap_or_default(),
            )
            .map(|nick_name| {
                APP.data
                    .textlar_map
                    .signal_map_cloned()
                    .key_cloned(
                        nick_name
                            .map(|nick_name| nick_name.value)
                            .unwrap_or_default(),
                    )
                    .map(|text| text.map(|text| text.value.clone()))
            })
            .flatten()
    }
    fn person_name(
        last_name_text: &Option<String>,
        first_name_text: &Option<String>,
        second_name_text: &Option<String>,
        nick_name_text: &Option<String>,
    ) -> String {
        let mut ret = String::new();
        if let Some(s) = (*last_name_text).as_deref() {
            if !s.is_empty() {
                ret.push_str(s);
            }
        }
        if let Some(s) = (*first_name_text).as_deref() {
            if !ret.is_empty() {
                ret.push(' ');
            }
            if !s.is_empty() {
                ret.push_str(s);
            }
        }
        if let Some(s) = (*second_name_text).as_deref() {
            if !ret.is_empty() {
                ret.push(' ');
            }
            if !s.is_empty() {
                ret.push_str(s);
            }
        }
        if let Some(s) = (*nick_name_text).as_deref() {
            if !ret.is_empty() {
                ret.push(' ');
            }
            if !s.is_empty() {
                use std::fmt::Write;
                let _ = write!(ret, "({})", s);
            }
        }
        ret
    }
    fn person_name_signal(person_id: i32) -> impl Signal<Item = String> {
        APP.data
            .personlar_map
            .signal_map_cloned()
            .key_cloned(person_id)
            .map(move |person| {
                map_ref! {
                    let last_name_text = Self::last_name_text_signal(&person)
                    , let first_name_text = Self::first_name_text_signal(&person)
                    , let second_name_text = Self::second_name_text_signal(&person)
                    , let nick_name_text = Self::nick_name_text_signal(&person)
                =>
                    Self::person_name(last_name_text, first_name_text, second_name_text, nick_name_text)
                }
            })
        .flatten()
    }
    fn filtered_judgelar(
        judgelar: &[Arc<Judge>],
        route: &Option<Route>,
        public_search: &Option<String>,
    ) -> Vec<Arc<Judge>> {
        if let Route::Guest(GuestRoute {
            kind: GuestRouteKind::Judge(sort_by),
            ..
        }) = route.clone().unwrap_or_else(route_default)
        {
            if let Some(public_search) = public_search {
                if let Some((main_filterlar, hash_filterlar)) =
                    App::get_judge_filterlar(public_search)
                {
                    App::sort_filtered_judgelar(
                        &sort_by,
                        judgelar
                            .iter()
                            .filter(|i| {
                                if main_filterlar.conforms_judge(i) {
                                    hash_filterlar.is_empty() || hash_filterlar.conforms(i)
                                } else {
                                    main_filterlar.is_empty() && hash_filterlar.conforms(i)
                                }
                            })
                            .cloned()
                            .collect::<Vec<Arc<Judge>>>(),
                    )
                } else {
                    vec![]
                }
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }
    fn filtered_judgelar_signal() -> impl Signal<Item = Vec<Arc<Judge>>> {
        map_ref! {
            let judgelar = APP.data.judgelar.signal_cloned(),
            let route = APP.data.route.signal_cloned(),
            let public_search = APP.data.public_search.signal_cloned()
        => Self::filtered_judgelar(judgelar, route, public_search)
        }
    }
    fn filtered_trainerlar(
        trainerlar: &[Arc<Trainer>],
        route: &Option<Route>,
        public_search: &Option<String>,
    ) -> Vec<Arc<Trainer>> {
        if let Route::Guest(GuestRoute {
            kind: GuestRouteKind::Trainer(sort_by),
            ..
        }) = route.clone().unwrap_or_else(route_default)
        {
            if let Some(public_search) = public_search {
                if let Some((main_filterlar, hash_filterlar)) =
                    App::get_trainer_filterlar(public_search)
                {
                    App::sort_filtered_trainerlar(
                        &sort_by,
                        trainerlar
                            .iter()
                            .filter(|i| {
                                if main_filterlar.conforms_trainer(i) {
                                    hash_filterlar.is_empty() || hash_filterlar.conforms(i)
                                } else {
                                    main_filterlar.is_empty() && hash_filterlar.conforms(i)
                                }
                            })
                            .cloned()
                            .collect::<Vec<Arc<Trainer>>>(),
                    )
                } else {
                    vec![]
                }
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }
    fn filtered_trainerlar_signal() -> impl Signal<Item = Vec<Arc<Trainer>>> {
        map_ref! {
            let trainerlar = APP.data.trainerlar.signal_cloned(),
            let route = APP.data.route.signal_cloned(),
            let public_search = APP.data.public_search.signal_cloned()
        => Self::filtered_trainerlar(trainerlar, route, public_search)
        }
    }
    fn filtered_clublar(
        clublar: &[Arc<Club>],
        route: &Option<Route>,
        public_search: &Option<String>,
    ) -> Vec<Arc<Club>> {
        if let Route::Guest(GuestRoute {
            kind: GuestRouteKind::Club(sort_by),
            ..
        }) = route.clone().unwrap_or_else(route_default)
        {
            if let Some(public_search) = public_search {
                if let Some((main_filterlar, hash_filterlar)) =
                    App::get_club_filterlar(public_search)
                {
                    App::sort_filtered_clublar(
                        &sort_by,
                        clublar
                            .iter()
                            .filter(|i| {
                                if main_filterlar.conforms_club(i) {
                                    hash_filterlar.is_empty() || hash_filterlar.conforms(i)
                                } else {
                                    main_filterlar.is_empty() && hash_filterlar.conforms(i)
                                }
                            })
                            .cloned()
                            .collect::<Vec<Arc<Club>>>(),
                    )
                } else {
                    vec![]
                }
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }
    fn filtered_clublar_signal() -> impl Signal<Item = Vec<Arc<Club>>> {
        map_ref! {
            let clublar = APP.data.clublar.signal_cloned(),
            let route = APP.data.route.signal_cloned(),
            let public_search = APP.data.public_search.signal_cloned()
        => Self::filtered_clublar(clublar, route, public_search)
        }
    }
    fn filtered_dancerlar(
        dancerlar: &[Arc<Dancer>],
        route: &Option<Route>,
        public_search: &Option<String>,
    ) -> Vec<Arc<Dancer>> {
        if let Route::Guest(GuestRoute {
            kind: GuestRouteKind::Dancer { sort_by, .. },
            ..
        }) = route.clone().unwrap_or_else(route_default)
        {
            if let Some(public_search) = public_search {
                if let Some((main_filterlar, hash_filterlar)) =
                    App::get_dancer_filterlar(public_search)
                {
                    App::sort_filtered_dancerlar(
                        &sort_by,
                        dancerlar
                            .iter()
                            .filter(|i| {
                                if main_filterlar.conforms_dancer(i) {
                                    hash_filterlar.is_empty() || hash_filterlar.conforms(i)
                                } else {
                                    main_filterlar.is_empty() && hash_filterlar.conforms(i)
                                }
                            })
                            .cloned()
                            .collect::<Vec<Arc<Dancer>>>(),
                    )
                } else {
                    vec![]
                }
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }
    fn filtered_dancerlar_signal() -> impl Signal<Item = Vec<Arc<Dancer>>> {
        map_ref! {
            let dancerlar = APP.data.dancerlar.signal_cloned(),
            let route = APP.data.route.signal_cloned(),
            let public_search = APP.data.public_search.signal_cloned()
        => Self::filtered_dancerlar(dancerlar, route, public_search)
        }
    }
    fn get_judge_filterlar(public_search: &str) -> Option<(MainFilterlar, JudgeHashFilterlar)> {
        let mut main_filterlar = MainFilterlar::default();
        let mut hash_filterlar = JudgeHashFilterlar::default();
        let mut hashtag: Vec<String> = vec![];
        let mut ss: Vec<String> = vec![];
        for s in public_search
            .split(' ')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(prepare_to_check_if_same_name)
        {
            if let Some(stripped) = s.strip_prefix('#') {
                if !ss.is_empty() {
                    if main_filterlar.judge_fill(&ss) {
                        ss.clear();
                    } else {
                        return None;
                    }
                }
                if !hashtag.is_empty() {
                    if hash_filterlar.fill(&hashtag) {
                        hashtag.clear();
                    } else {
                        return None;
                    }
                }
                hashtag.push(stripped.to_lowercase());
            } else if !hashtag.is_empty() {
                hashtag.push(s);
            } else {
                ss.push(s);
            }
        }
        if !ss.is_empty() && !main_filterlar.judge_fill(&ss) {
            return None;
        }
        if !hashtag.is_empty() && !hash_filterlar.fill(&hashtag) {
            return None;
        }
        Some((main_filterlar, hash_filterlar))
    }
    fn get_club_filterlar(public_search: &str) -> Option<(MainFilterlar, ClubHashFilterlar)> {
        let mut main_filterlar = MainFilterlar::default();
        let mut hash_filterlar = ClubHashFilterlar::default();
        let mut hashtag: Vec<String> = vec![];
        let mut ss: Vec<String> = vec![];
        for s in public_search
            .split(' ')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(prepare_to_check_if_same_name)
        {
            if let Some(stripped) = s.strip_prefix('#') {
                if !ss.is_empty() {
                    if main_filterlar.club_fill(&ss) {
                        ss.clear();
                    } else {
                        return None;
                    }
                }
                if !hashtag.is_empty() {
                    if hash_filterlar.fill(&hashtag) {
                        hashtag.clear();
                    } else {
                        return None;
                    }
                }
                hashtag.push(stripped.to_lowercase());
            } else if !hashtag.is_empty() {
                hashtag.push(s);
            } else {
                ss.push(s);
            }
        }
        if !ss.is_empty() && !main_filterlar.club_fill(&ss) {
            return None;
        }
        if !hashtag.is_empty() && !hash_filterlar.fill(&hashtag) {
            return None;
        }
        Some((main_filterlar, hash_filterlar))
    }
    fn get_trainer_filterlar(public_search: &str) -> Option<(MainFilterlar, TrainerHashFilterlar)> {
        let mut main_filterlar = MainFilterlar::default();
        let mut hash_filterlar = TrainerHashFilterlar::default();
        let mut hashtag: Vec<String> = vec![];
        let mut ss: Vec<String> = vec![];
        for s in public_search
            .split(' ')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(prepare_to_check_if_same_name)
        {
            if let Some(stripped) = s.strip_prefix('#') {
                if !ss.is_empty() {
                    if main_filterlar.trainer_fill(&ss) {
                        ss.clear();
                    } else {
                        return None;
                    }
                }
                if !hashtag.is_empty() {
                    if hash_filterlar.fill(&hashtag) {
                        hashtag.clear();
                    } else {
                        return None;
                    }
                }
                hashtag.push(stripped.to_lowercase());
            } else if !hashtag.is_empty() {
                hashtag.push(s);
            } else {
                ss.push(s);
            }
        }
        if !ss.is_empty() && !main_filterlar.trainer_fill(&ss) {
            return None;
        }
        if !hashtag.is_empty() && !hash_filterlar.fill(&hashtag) {
            return None;
        }
        Some((main_filterlar, hash_filterlar))
    }
    fn get_dancer_filterlar(public_search: &str) -> Option<(MainFilterlar, DancerHashFilterlar)> {
        let mut main_filterlar = MainFilterlar::default();
        let mut hash_filterlar = DancerHashFilterlar::default();
        let mut hashtag: Vec<String> = vec![];
        let mut ss: Vec<String> = vec![];
        for s in public_search
            .split(' ')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(prepare_to_check_if_same_name)
        {
            if let Some(stripped) = s.strip_prefix('#') {
                if !ss.is_empty() {
                    if main_filterlar.dancer_fill(&ss) {
                        ss.clear();
                    } else {
                        return None;
                    }
                }
                if !hashtag.is_empty() {
                    if hash_filterlar.fill(&hashtag) {
                        hashtag.clear();
                    } else {
                        return None;
                    }
                }
                hashtag.push(stripped.to_lowercase());
            } else if !hashtag.is_empty() {
                hashtag.push(s);
            } else {
                ss.push(s);
            }
        }
        if !ss.is_empty() && !main_filterlar.dancer_fill(&ss) {
            return None;
        }
        if !hashtag.is_empty() && !hash_filterlar.fill(&hashtag) {
            return None;
        }
        Some((main_filterlar, hash_filterlar))
    }
    fn sort_filtered_dancerlar(
        sort_by: &DancerSortBy,
        mut ret: Vec<Arc<Dancer>>,
    ) -> Vec<Arc<Dancer>> {
        if ret.is_empty() {
            ret
        } else {
            let personlar_map = &*APP.data.personlar_map.lock_ref();
            ret.sort_by(|a, b| dancer_sort_by_cmp(sort_by, a, b, personlar_map));
            ret
        }
    }
    fn sort_filtered_judgelar(sort_by: &JudgeSortBy, mut ret: Vec<Arc<Judge>>) -> Vec<Arc<Judge>> {
        if ret.is_empty() {
            ret
        } else {
            let personlar_map = &*APP.data.personlar_map.lock_ref();
            ret.sort_by(|a, b| judge_sort_by_cmp(sort_by, a, b, personlar_map));
            ret
        }
    }
    fn sort_filtered_trainerlar(
        sort_by: &TrainerSortBy,
        mut ret: Vec<Arc<Trainer>>,
    ) -> Vec<Arc<Trainer>> {
        if ret.is_empty() {
            ret
        } else {
            let personlar_map = &*APP.data.personlar_map.lock_ref();
            ret.sort_by(|a, b| trainer_sort_by_cmp(sort_by, a, b, personlar_map));
            ret
        }
    }
    fn sort_filtered_clublar(sort_by: &ClubSortBy, mut ret: Vec<Arc<Club>>) -> Vec<Arc<Club>> {
        if ret.is_empty() {
            ret
        } else {
            ret.sort_by(|a, b| club_sort_by_cmp(sort_by, a, b));
            ret
        }
    }
}

fn prepare_to_check_if_same_name(s: &str) -> String {
    s.to_lowercase().replace('ё', "е")
}

static mut SET_URL_TIMEOUT: Option<Timeout> = None;
pub const SET_URL_TIMEOUT_MILLIS: u32 = 500;
static mut URL: Option<String> = None;

fn delayed_update_url() {
    unsafe {
        if RENDER_TIMEOUT.is_some() {
            return;
        }
    }
    let url = route_to_url(&match (*APP.data.route.lock_ref())
        .clone()
        .unwrap_or_else(route_default)
    {
        Route::Guest(GuestRoute { kind, .. }) => Route::Guest(GuestRoute {
            kind: match kind {
                GuestRouteKind::Dancer { sort_by, .. } => GuestRouteKind::Dancer {
                    sort_by,
                    expanded: HashSet::new(),
                },
                _ => kind.clone(),
            },
            did_press: *APP.data.did_press.lock_ref(),
            search: APP.data.public_search.lock_ref().clone(),
        }),
        _ => unreachable!(),
    });
    delayed_go_to_url(url);
}

fn cancel_delayed_go_to_url() {
    unsafe {
        if let Some(timeout) = SET_URL_TIMEOUT.take() {
            timeout.cancel();
        }
    }
}

fn delayed_go_to_url(url: String) {
    cancel_delayed_go_to_url();
    unsafe {
        URL = Some(url);
        SET_URL_TIMEOUT = Some(Timeout::new(SET_URL_TIMEOUT_MILLIS, move || {
            SET_URL_TIMEOUT = None;
            if let Some(url) = URL.take() {
                if let Some(route_new) = route_from_url_hash(&url) {
                    if route_new
                        != (*APP.data.route.lock_ref())
                            .clone()
                            .unwrap_or_else(route_default)
                    {
                        go_to_url(&url);
                    }
                } else {
                    warn!(@ "url: {url}");
                }
            }
        }));
    }
}

fn is_conform_str2(filters: &[String], mut tst: Vec<String>) -> bool {
    let mut ret = false;
    for filter in filters.iter() {
        ret = false;
        let mut i = 0;
        loop {
            if i >= tst.len() {
                break;
            }
            if tst[i] == *filter {
                ret = true;
                tst.remove(i);
                break;
            }
            i += 1;
        }
        if !ret {
            break;
        }
    }
    ret
}

fn get_age_of_today_by_birth_date(today: &NaiveDate, birth_date: &NaiveDate) -> i32 {
    today.year() - birth_date.year()
        + match today.month().cmp(&birth_date.month()) {
            Ordering::Less => -1,
            Ordering::Greater => 0,
            Ordering::Equal => {
                if matches!(today.day().cmp(&birth_date.day()), Ordering::Less) {
                    -1
                } else {
                    0
                }
            }
        }
}

fn get_person_birth_date(person: Option<&Arc<Person>>) -> Option<NaiveDate> {
    person
        .as_ref()
        .and_then(|person| get_birth_date_opt(person.birth_date))
}
