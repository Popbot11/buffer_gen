use std::collections::HashMap;
use toml::Value;

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
    
    fn render_text(&self) -> String {
        todo!()
    }
}
impl Gain {
    pub fn new(signal: String, gain: String) -> Box<dyn Module> {
        println!("created new gain module. signal: {signal}, len: {gain} \n");
        Box::from(Self{
            signal: signal,
            gain: gain,
        })
    }
    pub fn new_entry(params: &Vec<Value>) -> Box<dyn Module> {
        Gain::new(params[0].as_str().unwrap().to_string(), params[1].as_str().unwrap().to_string())
    }
    // https://i.imgur.com/utJyz9b.png
}