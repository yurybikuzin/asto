use super::*;

#[derive(Debug, strum::EnumDiscriminants)]
#[strum_discriminants(derive(Hash))]
pub enum JudgeHashFilter {
    All,
    Clublar(HashSet<i16>),
    IsArchive(bool),
}
#[derive(Debug, Default)]
pub struct JudgeHashFilterlar(HashMap<JudgeHashFilterDiscriminants, JudgeHashFilter>);
impl JudgeHashFilterlar {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn conforms(&self, i: &Arc<Judge>) -> bool {
        self.conforms_all(i) && self.conforms_is_archive(i) && self.conforms_clublar(i)
    }
    fn conforms_all(&self, _i: &Arc<Judge>) -> bool {
        true
    }
    fn conforms_clublar(&self, i: &Arc<Judge>) -> bool {
        if let Some(JudgeHashFilter::Clublar(idlar)) =
            self.0.get(&JudgeHashFilterDiscriminants::Clublar)
        {
            idlar.contains(&i.club)
        } else {
            true
        }
    }
    fn conforms_is_archive(&self, i: &Arc<Judge>) -> bool {
        let is_archive = i.is_archive;
        if let Some(JudgeHashFilter::IsArchive(only)) =
            self.0.get(&JudgeHashFilterDiscriminants::IsArchive)
        {
            !*only || is_archive
        } else {
            !is_archive
        }
    }
    pub fn fill(&mut self, hashtag: &[String]) -> bool {
        self.fill_all(hashtag) || self.fill_is_archive(hashtag) || self.fill_clublar(hashtag)
    }
    fn fill_all(&mut self, hashtag: &[String]) -> bool {
        hashtag.len() == 1 && matches!(hashtag.get(0).map(|s| s.as_str()), Some("все")) && {
            self.0
                .insert(JudgeHashFilterDiscriminants::All, JudgeHashFilter::All);
            true
        }
    }
    fn fill_is_archive(&mut self, hashtag: &[String]) -> bool {
        if hashtag.iter().any(|s| s.as_str() == "архив") {
            let only = hashtag.iter().any(|s| s.as_str() == "только");
            if !only && hashtag.len() == 1 || only && hashtag.len() == 2 {
                self.0.insert(
                    JudgeHashFilterDiscriminants::IsArchive,
                    JudgeHashFilter::IsArchive(only),
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
                    common_macros2::entry!(self.0, JudgeHashFilterDiscriminants::Clublar
                    =>
                        and_modify |e| {
                            if let JudgeHashFilter::Clublar(idlar) = e {
                                idlar.insert(*id);
                            } else {
                                unreachable!();
                            }
                        }
                        or_insert JudgeHashFilter::Clublar(vec![*id].into_iter().collect())
                    );
                    ret = true;
                }
            }
        }
        ret
    }
}
