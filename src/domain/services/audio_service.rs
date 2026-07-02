use shaku::Interface;

pub trait AudioServiceInterface: Interface + Send + Sync {
    fn as_any(&self) -> &dyn std::any::Any;
    fn play_word(&self, word: &str);
    fn set_base_url(&self, url: String);
    fn has_base_url(&self) -> bool;
}
