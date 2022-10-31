use std::path::{Path, PathBuf};

#[cfg(not(target_family = "wasm"))]
use ::open_jtalk::*;

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

// see: voicevox-core
// https://github.com/VOICEVOX/voicevox_core/blob/main/crates/voicevox_core/src/engine/open_jtalk.rs
#[cfg(not(target_family = "wasm"))]
pub struct OpenJTalkNative {
    mecab: ManagedResource<Mecab>,
    njd: ManagedResource<Njd>,
    jpcommon: ManagedResource<JpCommon>,
    dict_loaded: bool,
}

#[cfg(not(target_family = "wasm"))]
impl TextToLabel for OpenJTalkNative {
    fn initialize() -> Self {
        Self {
            mecab: ManagedResource::initialize(),
            njd: ManagedResource::initialize(),
            jpcommon: ManagedResource::initialize(),
            dict_loaded: false,
        }
    }

    fn extract_fullcontext(&mut self, text: impl AsRef<str>) -> Result<Vec<String>> {
        let result = self.extract_fullcontext_non_reflesh(text);
        self.jpcommon.refresh();
        self.njd.refresh();
        self.mecab.refresh();
        result
    }

    fn load(&mut self, mecab_dict_dir: impl AsRef<Path>) -> Result<()> {
        let result = self.mecab.load(mecab_dict_dir.as_ref());
        if result {
            self.dict_loaded = true;
            Ok(())
        } else {
            self.dict_loaded = false;
            Err(OpenJTalkError::Load {
                mecab_dict_dir: mecab_dict_dir.as_ref().into(),
            })
        }
    }

    fn dict_loaded(&self) -> bool {
        self.dict_loaded
    }
}

#[cfg(not(target_family = "wasm"))]
impl OpenJTalkNative {
    fn extract_fullcontext_non_reflesh(&mut self, text: impl AsRef<str>) -> Result<Vec<String>> {
        let mecab_text =
            text2mecab(text.as_ref()).map_err(|e| OpenJTalkError::ExtractFullContext {
                text: text.as_ref().into(),
                source: Some(e.into()),
            })?;
        if self.mecab.analysis(mecab_text) {
            self.njd.mecab2njd(
                self.mecab
                    .get_feature()
                    .ok_or(OpenJTalkError::ExtractFullContext {
                        text: text.as_ref().into(),
                        source: None,
                    })?,
                self.mecab.get_size(),
            );
            self.njd.set_pronunciation();
            self.njd.set_digit();
            self.njd.set_accent_phrase();
            self.njd.set_accent_type();
            self.njd.set_unvoiced_vowel();
            self.njd.set_long_vowel();
            self.jpcommon.njd2jpcommon(&self.njd);
            self.jpcommon.make_label();
            self.jpcommon
                .get_label_feature_to_iter()
                .ok_or_else(|| OpenJTalkError::ExtractFullContext {
                    text: text.as_ref().into(),
                    source: None,
                })
                .map(|iter| iter.map(|s| s.to_string()).collect())
        } else {
            Err(OpenJTalkError::ExtractFullContext {
                text: text.as_ref().into(),
                source: None,
            })
        }
    }
}

#[cfg(target_family = "wasm")]
pub struct OpenJTalkWasm {}

#[cfg(target_family = "wasm")]
impl TextToLabel for OpenJTalkWasm {
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
