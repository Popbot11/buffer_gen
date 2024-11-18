use std::collections::HashMap;
use crate::core::{module::Module, sample::Sample};

pub struct Gain {
    pub signal: String,
    pub gain: String,
}
impl Module for Gain {
    fn tick_sample(&self, mdl_cache: &HashMap<String, Box<dyn Module>>, i: usize) -> Sample {
        let signal = mdl_cache[&self.signal].tick_sample(mdl_cache, i);
        let gain = mdl_cache[&self.gain].tick_sample(mdl_cache, i);
        Sample::new(signal.i, signal.val * gain.val)
    }
}
impl Gain {
    pub fn new(signal: String, gain: String) -> Box<dyn Module> {
        Box::from(Self{
            signal: signal,
            gain: gain,
        })
    }
}