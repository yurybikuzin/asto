use super::*;

#[derive(Debug, strum::EnumDiscriminants)]
#[strum_discriminants(derive(Hash))]
pub enum TrainerHashFilter {
    All,
    Clublar(HashSet<i16>),
}
#[derive(Debug, Default)]
pub struct TrainerHashFilterlar(HashMap<TrainerHashFilterDiscriminants, TrainerHashFilter>);
impl TrainerHashFilterlar {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn conforms(&self, i: &Arc<Trainer>) -> bool {
        self.conforms_all(i) && self.conforms_clublar(i)
        // && self.conforms_eventlar(i)
    }
    fn conforms_all(&self, _i: &Arc<Trainer>) -> bool {
        true
    }
    fn conforms_clublar(&self, i: &Arc<Trainer>) -> bool {
        if let Some(TrainerHashFilter::Clublar(idlar)) =
            self.0.get(&TrainerHashFilterDiscriminants::Clublar)
        {
            idlar.contains(&i.club)
        } else {
            true
        }
    }
    pub fn fill(&mut self, hashtag: &[String]) -> bool {
        self.fill_all(hashtag) || self.fill_clublar(hashtag)
    }
    fn fill_all(&mut self, hashtag: &[String]) -> bool {
        hashtag.len() == 1 && matches!(hashtag.get(0).map(|s| s.as_str()), Some("все")) && {
            self.0
                .insert(TrainerHashFilterDiscriminants::All, TrainerHashFilter::All);
            true
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
                    common_macros2::entry!(self.0, TrainerHashFilterDiscriminants::Clublar
                    =>
                        and_modify |e| {
                            if let TrainerHashFilter::Clublar(idlar) = e {
                                idlar.insert(*id);
                            } else {
                                unreachable!();
                            }
                        }
                        or_insert TrainerHashFilter::Clublar(vec![*id].into_iter().collect())
                    );
                    ret = true;
                }
            }
        }
        ret
    }
}
