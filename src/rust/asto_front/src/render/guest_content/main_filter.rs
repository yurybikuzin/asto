use super::*;

#[derive(Debug, strum::EnumDiscriminants)]
#[strum_discriminants(derive(Hash))]
pub enum MainFilter {
    ExternalIdlar(HashSet<i32>),
    Entitilar(HashSet<i32>),
}

#[derive(Debug, Default)]
pub struct MainFilterlar(HashMap<MainFilterDiscriminants, MainFilter>);
impl MainFilterlar {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn conforms_club(&self, i: &Arc<Club>) -> bool {
        self.conforms_club_external_idlar(i) && self.conforms_club_entitilar(i)
    }
    fn conforms_club_entitilar(&self, i: &Arc<Club>) -> bool {
        if let Some(MainFilter::Entitilar(idlar)) = self.0.get(&MainFilterDiscriminants::Entitilar)
        {
            idlar.contains(&(i.id as i32))
        } else {
            true
        }
    }
    fn conforms_club_external_idlar(&self, _i: &Arc<Club>) -> bool {
        self.0
            .get(&MainFilterDiscriminants::ExternalIdlar)
            .is_none()
        // if let Some(MainFilter::ExternalIdlar(idlar)) =
        //     self.0.get(&MainFilterDiscriminants::ExternalIdlar)
        // {
        //     false
        //     // i.external_id
        //     //     .map(|external_id| idlar.contains(&external_id))
        //     //     .unwrap_or(false)
        // } else {
        //     true
        // }
    }
    pub fn conforms_trainer(&self, i: &Arc<Trainer>) -> bool {
        self.conforms_trainer_external_idlar(i) && self.conforms_trainer_entitilar(i)
    }
    fn conforms_trainer_entitilar(&self, i: &Arc<Trainer>) -> bool {
        if let Some(MainFilter::Entitilar(idlar)) = self.0.get(&MainFilterDiscriminants::Entitilar)
        {
            idlar.contains(&i.id)
        } else {
            true
        }
    }
    fn conforms_trainer_external_idlar(&self, _i: &Arc<Trainer>) -> bool {
        self.0
            .get(&MainFilterDiscriminants::ExternalIdlar)
            .is_none()
        // if let Some(MainFilter::ExternalIdlar(idlar)) =
        //     self.0.get(&MainFilterDiscriminants::ExternalIdlar)
        // {
        //     false
        //     // i.external_id
        //     //     .map(|external_id| idlar.contains(&external_id))
        //     //     .unwrap_or(false)
        // } else {
        //     true
        // }
    }
    pub fn conforms_judge(&self, i: &Arc<Judge>) -> bool {
        self.conforms_judge_external_idlar(i) && self.conforms_judge_entitilar(i)
    }
    pub fn conforms_dancer(&self, i: &Arc<Dancer>) -> bool {
        self.conforms_dancer_external_idlar(i) && self.conforms_dancer_entitilar(i)
    }
    fn conforms_judge_external_idlar(&self, i: &Arc<Judge>) -> bool {
        if let Some(MainFilter::ExternalIdlar(idlar)) =
            self.0.get(&MainFilterDiscriminants::ExternalIdlar)
        {
            i.external_id
                .map(|external_id| idlar.contains(&external_id))
                .unwrap_or(false)
        } else {
            true
        }
    }
    fn conforms_judge_entitilar(&self, i: &Arc<Judge>) -> bool {
        if let Some(MainFilter::Entitilar(idlar)) = self.0.get(&MainFilterDiscriminants::Entitilar)
        {
            idlar.contains(&i.id)
        } else {
            true
        }
    }
    fn conforms_dancer_external_idlar(&self, i: &Arc<Dancer>) -> bool {
        if let Some(MainFilter::ExternalIdlar(idlar)) =
            self.0.get(&MainFilterDiscriminants::ExternalIdlar)
        {
            i.external_id
                .map(|external_id| idlar.contains(&external_id))
                .unwrap_or(false)
        } else {
            true
        }
    }
    fn conforms_dancer_entitilar(&self, i: &Arc<Dancer>) -> bool {
        if let Some(MainFilter::Entitilar(idlar)) = self.0.get(&MainFilterDiscriminants::Entitilar)
        {
            idlar.contains(&i.id)
        } else {
            true
        }
    }
    pub fn dancer_fill(&mut self, ss: &[String]) -> bool {
        let name = self.fill_external_idlar(ss);
        self.fill_dancer_entitilar(name)
    }
    pub fn judge_fill(&mut self, ss: &[String]) -> bool {
        let name = self.fill_external_idlar(ss);
        self.fill_judge_entitilar(name)
    }
    pub fn trainer_fill(&mut self, ss: &[String]) -> bool {
        let name = self.fill_external_idlar(ss);
        self.fill_trainer_entitilar(name)
    }
    pub fn club_fill(&mut self, ss: &[String]) -> bool {
        let name = self.fill_external_idlar(ss);
        self.fill_club_entitilar(name)
    }
    fn fill_external_idlar(&mut self, ss: &[String]) -> Vec<String> {
        let mut name: Vec<String> = vec![];
        for s in ss.iter() {
            if let Ok(i) = s.parse::<i32>() {
                common_macros2::entry!(self.0, MainFilterDiscriminants::ExternalIdlar
                =>
                    and_modify |e| {
                        if let MainFilter::ExternalIdlar(ref mut ilar) = e {
                            ilar.insert(i);
                        } else {
                            unreachable!();
                        }
                    }
                    or_insert MainFilter::ExternalIdlar(vec![i].into_iter().collect())
                )
            } else {
                name.push(s.clone());
            }
        }
        name
    }
    fn fill_dancer_entitilar(&mut self, name: Vec<String>) -> bool {
        name.is_empty() || {
            let mut ret = false;
            let personlar_map = &*APP.data.personlar_map.lock_ref();
            let textlar_map = &APP.data.textlar_map.lock_ref();
            let last_namelar_map = &APP.data.last_namelar_map.lock_ref();
            let first_namelar_map = &APP.data.first_namelar_map.lock_ref();
            let second_namelar_map = &APP.data.second_namelar_map.lock_ref();
            let nick_namelar_map = &APP.data.nick_namelar_map.lock_ref();
            for (id, dancer) in (*APP.data.dancerlar_map.lock_ref()).iter() {
                if let Some(person) = personlar_map.get(&dancer.person) {
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
                    if is_conform_str2(&name, person_name) {
                        common_macros2::entry!(self.0, MainFilterDiscriminants::Entitilar
                        =>
                            and_modify |e| {
                                if let MainFilter::Entitilar(idlar) = e {
                                    idlar.insert(*id);
                                } else {
                                    unreachable!();
                                }
                            }
                            or_insert MainFilter::Entitilar(vec![*id].into_iter().collect())
                        );
                        ret = true;
                    }
                }
            }
            ret
        }
    }
    fn fill_judge_entitilar(&mut self, name: Vec<String>) -> bool {
        name.is_empty() || {
            let mut ret = false;
            let personlar_map = &*APP.data.personlar_map.lock_ref();
            let textlar_map = &APP.data.textlar_map.lock_ref();
            let last_namelar_map = &APP.data.last_namelar_map.lock_ref();
            let first_namelar_map = &APP.data.first_namelar_map.lock_ref();
            let second_namelar_map = &APP.data.second_namelar_map.lock_ref();
            let nick_namelar_map = &APP.data.nick_namelar_map.lock_ref();
            for (id, judge) in (*APP.data.judgelar_map.lock_ref()).iter() {
                if let Some(person) = personlar_map.get(&judge.person) {
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
                    if is_conform_str2(&name, person_name) {
                        common_macros2::entry!(self.0, MainFilterDiscriminants::Entitilar
                        =>
                            and_modify |e| {
                                if let MainFilter::Entitilar(idlar) = e {
                                    idlar.insert(*id);
                                } else {
                                    unreachable!();
                                }
                            }
                            or_insert MainFilter::Entitilar(vec![*id].into_iter().collect())
                        );
                        ret = true;
                    }
                }
            }
            ret
        }
    }
    fn fill_trainer_entitilar(&mut self, name: Vec<String>) -> bool {
        name.is_empty() || {
            let mut ret = false;
            let personlar_map = &*APP.data.personlar_map.lock_ref();
            let textlar_map = &APP.data.textlar_map.lock_ref();
            let last_namelar_map = &APP.data.last_namelar_map.lock_ref();
            let first_namelar_map = &APP.data.first_namelar_map.lock_ref();
            let second_namelar_map = &APP.data.second_namelar_map.lock_ref();
            let nick_namelar_map = &APP.data.nick_namelar_map.lock_ref();
            for (id, trainer) in (*APP.data.trainerlar_map.lock_ref()).iter() {
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
                    if is_conform_str2(&name, person_name) {
                        common_macros2::entry!(self.0, MainFilterDiscriminants::Entitilar
                        =>
                            and_modify |e| {
                                if let MainFilter::Entitilar(idlar) = e {
                                    idlar.insert(*id);
                                } else {
                                    unreachable!();
                                }
                            }
                            or_insert MainFilter::Entitilar(vec![*id].into_iter().collect())
                        );
                        ret = true;
                    }
                }
            }
            ret
        }
    }
    fn fill_club_entitilar(&mut self, name: Vec<String>) -> bool {
        name.is_empty() || {
            let mut ret = false;
            let textlar_map = &APP.data.textlar_map.lock_ref();
            for (id, club) in (*APP.data.clublar_map.lock_ref()).iter() {
                let club_name = textlar_map.get(&club.value).and_then(|text| {
                    (!text.value.is_empty()).then(|| {
                        text.value
                            .split(' ')
                            .filter(|s| !s.is_empty())
                            .map(prepare_to_check_if_same_name)
                            .collect::<Vec<_>>()
                    })
                });
                if let Some(club_name) = club_name {
                    if is_conform_str2(&name, club_name) {
                        // if name == club_name {
                        common_macros2::entry!(self.0, MainFilterDiscriminants::Entitilar
                        =>
                            and_modify |e| {
                                if let MainFilter::Entitilar(idlar) = e {
                                    idlar.insert(*id as i32);
                                } else {
                                    unreachable!();
                                }
                            }
                            or_insert MainFilter::Entitilar(vec![*id as i32].into_iter().collect())
                        );
                        ret = true;
                    }
                }
            }
            ret
        }
    }
}
