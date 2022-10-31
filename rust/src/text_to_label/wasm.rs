use crate::text_to_label::shared::*;

use std::path::Path;

pub struct OpenJTalk{}

impl TextToLabel for OpenJTalk {
    fn initialize() -> Self {
        Self {}
    }

    fn dict_loaded(&self) -> bool {
        true
    }

    fn extract_fullcontext(&mut self, _text: impl AsRef<str>) -> Result<Vec<String>> {
        Ok(Vec::new())
    }

    fn load(&mut self, _mecab_dict_dir: impl AsRef<Path>) -> Result<()> {
        Ok(())
    }
}
