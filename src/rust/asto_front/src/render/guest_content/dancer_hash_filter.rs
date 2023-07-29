use super::*;

#[derive(Debug, strum::EnumDiscriminants)]
#[strum_discriminants(derive(Hash))]
pub enum DancerHashFilter {
    All,
    IsBeginning(bool),
    IsArchive(bool), // bool - for only
    Clublar(HashSet<i16>),
    Eventlar(HashSet<i32>),
    Trainerlar(HashSet<i32>),
    Classlar(HashSet<i8>),
    IsOver(u8),
    IsOverBall(u8),
    // Show(DancerHashFilterShow),
}

// #[derive(Debug, PartialEq, Eq)]
// pub enum DancerHashFilterShow {
//     Trainerlar,
// }

#[derive(Debug, Default)]
pub struct DancerHashFilterlar(HashMap<DancerHashFilterDiscriminants, DancerHashFilter>);
impl DancerHashFilterlar {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn conforms(&self, i: &Arc<Dancer>) -> bool {
        self.conforms_all(i)
            && self.conforms_is_beginning(i)
            && self.conforms_is_archive(i)
            && self.conforms_is_over(i)
            && self.conforms_is_over_ball(i)
            && self.conforms_clublar(i)
            && self.conforms_classlar(i)
            && self.conforms_eventlar(i)
            && self.conforms_trainerlar(i)
    }
    pub fn fill(&mut self, hashtag: &[String]) -> bool {
        self.fill_all(hashtag)
            || self.fill_is_beginning(hashtag)
            || self.fill_is_archive(hashtag)
            || self.fill_is_over(hashtag)
            || self.fill_is_over_ball(hashtag)
            || self.fill_classlar(hashtag)
            || self.fill_clublar(hashtag)
            || self.fill_trainerlar(hashtag)
            || self.fill_eventlar(hashtag)
        // || self.fill_show(hashtag)
    }
    // fn fill_show(&mut self, hashtag: &[String]) -> bool {
    //     hashtag.len() == 1 && matches!(hashtag.get(0).map(|s| s.as_str()), Some("тренеры")) && {
    //         let show = DancerHashFilterShow::Trainerlar;
    //         common_macros2::entry!(self.0, DancerHashFilterDiscriminants::IsBeginning
    //         =>
    //             and_modify |e| {
    //                 if let DancerHashFilter::Show(show_eta) = e {
    //                     if *show_eta != show {
    //                         warn!(@ "Show is already set to {show_eta:?}");
    //                     }
    //                 } else {
    //                     unreachable!();
    //                 }
    //             }
    //             or_insert DancerHashFilter::Show(show)
    //         );
    //         true
    //     }
    // }
    fn fill_all(&mut self, hashtag: &[String]) -> bool {
        hashtag.len() == 1 && matches!(hashtag.get(0).map(|s| s.as_str()), Some("все")) && {
            self.0
                .insert(DancerHashFilterDiscriminants::All, DancerHashFilter::All);
            true
        }
    }
    fn fill_is_beginning(&mut self, hashtag: &[String]) -> bool {
        hashtag.len() == 1
            && matches!(
                hashtag.get(0).map(|s| s.as_str()),
                Some("классовые" | "начинающие")
            )
            && {
                let is_beginning = matches!(hashtag.get(0).map(|s| s.as_str()), Some("начинающие"));
                common_macros2::entry!(self.0, DancerHashFilterDiscriminants::IsBeginning
                =>
                    and_modify |e| {
                        if let DancerHashFilter::IsBeginning(is_beginning_eta) = e {
                            if *is_beginning_eta != is_beginning {
                                warn!(@ "IsBeginning is already set to {is_beginning_eta}");
                            }
                        } else {
                            unreachable!();
                        }
                    }
                    or_insert DancerHashFilter::IsBeginning(is_beginning)
                );
                true
            }
    }
    fn fill_is_archive(&mut self, hashtag: &[String]) -> bool {
        if hashtag.iter().any(|s| s.as_str() == "архив") {
            let only = hashtag.iter().any(|s| s.as_str() == "только");
            if !only && hashtag.len() == 1 || only && hashtag.len() == 2 {
                self.0.insert(
                    DancerHashFilterDiscriminants::IsArchive,
                    DancerHashFilter::IsArchive(only),
                );
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    fn fill_is_over_ball(&mut self, hashtag: &[String]) -> bool {
        if hashtag.len() == 2
            && matches!(
                hashtag.get(0).map(|s| s.as_str()),
                Some("перебравшие-баллы")
            )
        {
            if let Some(limit) = hashtag.get(1).and_then(|s| s.parse::<u8>().ok()) {
                self.0.insert(
                    DancerHashFilterDiscriminants::IsOverBall,
                    DancerHashFilter::IsOverBall(limit),
                );
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    fn fill_is_over(&mut self, hashtag: &[String]) -> bool {
        if hashtag.len() == 2 && matches!(hashtag.get(0).map(|s| s.as_str()), Some("перебравшие"))
        {
            if let Some(limit) = hashtag.get(1).and_then(|s| s.parse::<u8>().ok()) {
                self.0.insert(
                    DancerHashFilterDiscriminants::IsOver,
                    DancerHashFilter::IsOver(limit),
                );
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    fn fill_classlar(&mut self, hashtag: &[String]) -> bool {
        if hashtag.len() == 2 && matches!(hashtag.get(0).map(|s| s.as_str()), Some("класс")) {
            let class = hashtag.get(1).and_then(|s| match s.as_str() {
                "h2" | "н2" => Some(11),
                "h3" | "н3" => Some(10),
                "h4" | "н4" => Some(9),
                "h5" | "н5" => Some(8),
                "e" | "е" => Some(7),
                "d" | "д" => Some(6),
                "c" | "с" => Some(5),
                "b" | "б" => Some(4),
                "a" | "а" => Some(3),
                "s" => Some(2),
                "m" | "м" => Some(1),
                _ => None,
            });
            if let Some(class) = class {
                common_macros2::entry!(self.0, DancerHashFilterDiscriminants::Classlar
                =>
                    and_modify |e| {
                        if let DancerHashFilter::Classlar(classlar) = e {
                            classlar.insert(class);
                        } else {
                            unreachable!();
                        }
                    }
                    or_insert DancerHashFilter::Classlar(vec![class].into_iter().collect())
                );
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    fn fill_clublar(&mut self, hashtag: &[String]) -> bool {
        let mut ret = false;
        let textlar_map = &APP.data.textlar_map.lock_ref();
        let clublar_map = &APP.data.clublar_map.lock_ref();
        for (id, club) in clublar_map.iter() {
            if let Some(text) = textlar_map.get(&club.value) {
                let club_name = text
                    .value
                    .split(' ')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .map(prepare_to_check_if_same_name)
                    .collect::<Vec<_>>();
                if is_conform_str2(hashtag, club_name) {
                    common_macros2::entry!(self.0, DancerHashFilterDiscriminants::Clublar
                    =>
                        and_modify |e| {
                            if let DancerHashFilter::Clublar(idlar) = e {
                                idlar.insert(*id);
                            } else {
                                unreachable!();
                            }
                        }
                        or_insert DancerHashFilter::Clublar(vec![*id].into_iter().collect())
                    );
                    ret = true;
                }
            }
        }
        ret
    }
    fn fill_trainerlar(&mut self, hashtag: &[String]) -> bool {
        let mut ret = false;
        let trainerlar_map = &*APP.data.trainerlar_map.lock_ref();
        let personlar_map = &*APP.data.personlar_map.lock_ref();
        let textlar_map = &APP.data.textlar_map.lock_ref();
        let last_namelar_map = &APP.data.last_namelar_map.lock_ref();
        let first_namelar_map = &APP.data.first_namelar_map.lock_ref();
        let second_namelar_map = &APP.data.second_namelar_map.lock_ref();
        let nick_namelar_map = &APP.data.nick_namelar_map.lock_ref();
        for (id, trainer) in trainerlar_map.iter() {
            if let Some(person) = personlar_map.get(&trainer.person) {
                let person_name = [
                    last_namelar_map
                        .get(&person.last_name)
                        .and_then(|last_name| textlar_map.get(&last_name.value)),
                    first_namelar_map
                        .get(&person.first_name)
                        .and_then(|first_name| textlar_map.get(&first_name.value)),
                    second_namelar_map
                        .get(&person.second_name)
                        .and_then(|second_name| textlar_map.get(&second_name.value)),
                    nick_namelar_map
                        .get(&person.nick_name)
                        .and_then(|nick_name| textlar_map.get(&nick_name.value)),
                ]
                .into_iter()
                .filter_map(|text| text.map(|text| text.value.clone()))
                .filter(|s| !s.is_empty())
                .map(|s| prepare_to_check_if_same_name(s.as_str()))
                .collect::<Vec<String>>();
                if is_conform_str2(hashtag, person_name) {
                    common_macros2::entry!(self.0, DancerHashFilterDiscriminants::Trainerlar
                    =>
                        and_modify |e| {
                            if let DancerHashFilter::Trainerlar(idlar) = e {
                                idlar.insert(*id);
                            } else {
                                unreachable!();
                            }
                        }
                        or_insert DancerHashFilter::Trainerlar(vec![*id].into_iter().collect())
                    );
                    ret = true;
                }
            }
        }
        ret
    }
    fn fill_eventlar(&mut self, hashtag: &[String]) -> bool {
        let mut ret = false;
        let textlar_map = &APP.data.textlar_map.lock_ref();
        let eventlar_map = &APP.data.eventlar_map.lock_ref();
        for (id, event) in eventlar_map.iter() {
            if let Some(text) = textlar_map.get(&event.title) {
                let event_name = text
                    .value
                    .split(' ')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .map(prepare_to_check_if_same_name)
                    .collect::<Vec<_>>();
                if is_conform_str2(hashtag, event_name.clone()) {
                    common_macros2::entry!(self.0, DancerHashFilterDiscriminants::Eventlar
                    =>
                        and_modify |e| {
                            if let DancerHashFilter::Eventlar(idlar) = e {
                                idlar.insert(*id);
                            } else {
                                unreachable!();
                            }
                        }
                        or_insert DancerHashFilter::Eventlar(vec![*id].into_iter().collect())
                    );
                    ret = true;
                }
            }
        }
        ret
    }
    // ====================================
    fn conforms_all(&self, _i: &Arc<Dancer>) -> bool {
        true
    }
    fn conforms_is_beginning(&self, i: &Arc<Dancer>) -> bool {
        if let Some(DancerHashFilter::IsBeginning(is_beginning)) =
            self.0.get(&DancerHashFilterDiscriminants::IsBeginning)
        {
            *is_beginning == i.is_beginning(&None, false)
        } else {
            true
        }
    }
    fn conforms_is_archive(&self, i: &Arc<Dancer>) -> bool {
        let is_not_active = !(is_active(&i.external_id, &None) || i.is_beginning(&None, true));
        let is_archive = i.is_archive || is_not_active;
        if let Some(DancerHashFilter::IsArchive(only)) =
            self.0.get(&DancerHashFilterDiscriminants::IsArchive)
        {
            !*only || is_archive
        } else {
            !is_archive
        }
    }
    fn conforms_is_over(&self, i: &Arc<Dancer>) -> bool {
        if let Some(DancerHashFilter::IsOver(limit)) =
            self.0.get(&DancerHashFilterDiscriminants::IsOver)
        {
            !i.is_beginning(&None, false)
                && APP
                    .data
                    .dancer_score(i, DancerScorePeriod::FromUpgrade)
                    .values()
                    .sum::<i16>()
                    > *limit as i16 * 4
        } else {
            true
        }
    }
    fn conforms_is_over_ball(&self, i: &Arc<Dancer>) -> bool {
        if let Some(DancerHashFilter::IsOverBall(limit)) =
            self.0.get(&DancerHashFilterDiscriminants::IsOverBall)
        {
            i.is_beginning(&None, false)
                && APP.data.dancer_points(i, DancerScorePeriod::FromUpgrade) > *limit as i16 * 10
        } else {
            true
        }
    }
    fn conforms_classlar(&self, i: &Arc<Dancer>) -> bool {
        if let Some(DancerHashFilter::Classlar(classlar)) =
            self.0.get(&DancerHashFilterDiscriminants::Classlar)
        {
            classlar.contains(&i.st_class()) || classlar.contains(&i.la_class())
        } else {
            true
        }
    }
    fn conforms_clublar(&self, i: &Arc<Dancer>) -> bool {
        if let Some(DancerHashFilter::Clublar(idlar)) =
            self.0.get(&DancerHashFilterDiscriminants::Clublar)
        {
            idlar.contains(&i.club)
        } else {
            true
        }
    }
    fn conforms_eventlar(&self, i: &Arc<Dancer>) -> bool {
        if let Some(DancerHashFilter::Eventlar(idlar)) =
            self.0.get(&DancerHashFilterDiscriminants::Eventlar)
        {
            if let Some(external_id) = i.external_id {
                if let Some(event_resultlar) = APP.data.event_resultlar.lock_ref().get(&external_id)
                {
                    event_resultlar
                        .iter()
                        .any(|event_result| idlar.contains(&event_result.event))
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            true
        }
    }
    fn conforms_trainerlar(&self, i: &Arc<Dancer>) -> bool {
        if let Some(DancerHashFilter::Trainerlar(idlar)) =
            self.0.get(&DancerHashFilterDiscriminants::Trainerlar)
        {
            idlar.contains(&i.trainer) || idlar.contains(&i.trainer2)
        } else {
            true
        }
    }
}
