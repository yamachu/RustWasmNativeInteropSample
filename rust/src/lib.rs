mod text_to_label;

use text_to_label::{
    implement::{self, OpenJTalk},
    shared::TextToLabel,
};

struct SampleImpl {
    ojt: OpenJTalk,
}

impl SampleImpl {
    pub fn initialize() -> Self {
        Self {
            ojt: implement::OpenJTalk::initialize(),
        }
    }

    pub fn run(&mut self) {
        if let Ok(v) = self.ojt.extract_fullcontext("ほげふが") {
            println!("{:?}", v);
        }
    }
}

pub fn dummy_impl() {
    if let Ok(v) = implement::OpenJTalk::initialize().extract_fullcontext("ほげふが") {
        println!("{:?}", v);
    }

    SampleImpl::initialize().run();
}

// TODO: Test Native Library
