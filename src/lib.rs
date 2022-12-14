// ---------- //
// 0. Imports //
// ---------- //
use rand::Rng;
use std::borrow::BorrowMut;
use vst::prelude::*;


// For later vst construction
// https://github.com/free-audio/clap-wrapper


// ----------------------------- //
// 1. Define the plugin struct //
// ----------------------------- //
struct Synthy;

impl Plugin for Synthy {
    fn new(_host: HostCallback) -> Self {
        Synthy
    }

    // -------------- //
    // 2. Plugin info //
    // -------------- //
    fn get_info(&self) -> Info {
        Info {
            name: "synthy".into(),
            vendor: "rusty".into(),
            unique_id: 128956,
            category: Category::Synth,
            inputs: 0,
            outputs: 2,
            parameters: 0,
            ..Info::default()
        }
    }

    // -------------------------- //
    // 3. Modify the audio buffer //
    // -------------------------- //
    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let (_, mut outputs) = buffer.split();
        for output in outputs.borrow_mut() {
            rand::thread_rng().fill(output);
        }
    }
}

// ------------------- //
// 4. Build the plugin //
// ------------------- //
vst::plugin_main!(Synthy);