use std::path::{Path, PathBuf};

#[derive(thiserror::Error, Debug)]
pub enum OpenJTalkError {
    #[error("open_jtalk load error")]
    Load { mecab_dict_dir: PathBuf },
    #[error("open_jtalk extract_fullcontext error")]
    ExtractFullContext {
        text: String,
        #[source]
        source: Option<anyhow::Error>,
    },
}

pub type Result<T> = std::result::Result<T, OpenJTalkError>;

pub trait TextToLabel {
    fn initialize() -> Self;
    fn extract_fullcontext(&mut self, text: impl AsRef<str>) -> Result<Vec<String>>;
    fn load(&mut self, mecab_dict_dir: impl AsRef<Path>) -> Result<()>;
    fn dict_loaded(&self) -> bool;
}
