mod text_to_label;

use text_to_label::{implement, shared::TextToLabel};

pub fn dummy_impl() {
    if let Ok(v) = implement::OpenJTalkNative::initialize().extract_fullcontext("ほげふが") {
        println!("{:?}", v);
    }
}

// TODO: Test Native Library
