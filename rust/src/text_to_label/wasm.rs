use wasm_bindgen::prelude::wasm_bindgen;

use crate::text_to_label::shared::*;

use std::path::Path;

#[wasm_bindgen]
extern "C" {
    pub fn this_is_from_env() -> bool;
}

pub struct OpenJTalk {}

impl TextToLabel for OpenJTalk {
    fn initialize() -> Self {
        Self {}
    }

    fn dict_loaded(&self) -> bool {
        this_is_from_env()
    }

    fn extract_fullcontext(&mut self, _text: impl AsRef<str>) -> Result<Vec<String>> {
        Ok(Vec::new())
    }

    fn load(&mut self, _mecab_dict_dir: impl AsRef<Path>) -> Result<()> {
        Ok(())
    }
}
