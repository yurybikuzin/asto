use super::*;

#[derive(Debug, strum::EnumDiscriminants)]
#[strum_discriminants(derive(Hash))]
pub enum ClubHashFilter {
    All,
}
#[derive(Debug, Default)]
pub struct ClubHashFilterlar(HashMap<ClubHashFilterDiscriminants, ClubHashFilter>);
impl ClubHashFilterlar {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn conforms(&self, i: &Arc<Club>) -> bool {
        self.conforms_all(i)
    }
    fn conforms_all(&self, _i: &Arc<Club>) -> bool {
        true
    }
    pub fn fill(&mut self, hashtag: &[String]) -> bool {
        self.fill_all(hashtag)
    }
    fn fill_all(&mut self, hashtag: &[String]) -> bool {
        hashtag.len() == 1 && matches!(hashtag.get(0).map(|s| s.as_str()), Some("все")) && {
            self.0
                .insert(ClubHashFilterDiscriminants::All, ClubHashFilter::All);
            true
        }
    }
}
