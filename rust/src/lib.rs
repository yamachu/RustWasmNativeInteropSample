mod text_to_label;

use text_to_label::{OpenJTalk, TextToLabel};
use wasm_bindgen::prelude::wasm_bindgen;

struct SampleImpl {
    ojt: OpenJTalk,
}

impl SampleImpl {
    pub fn initialize() -> Self {
        Self {
            ojt: OpenJTalk::initialize(),
        }
    }

    pub fn run(&mut self) {
        if let Ok(v) = self.ojt.extract_fullcontext("ほげふが") {
            println!("{:?}", v);
        }
    }
}

pub fn dummy_impl() {
    if let Ok(v) = OpenJTalk::initialize().extract_fullcontext("ほげふが") {
        println!("{:?}", v);
    }

    SampleImpl::initialize().run();
}

#[wasm_bindgen]
pub fn greet() -> bool {
    let ojt = OpenJTalk::initialize();
    ojt.dict_loaded()
}

// TODO: Test Native Library
