use std::collections::HashMap;
use toml::Value;

use crate::core::module::{Module, ModuleInfo};

pub struct Multiply {
    pub signal_a: String,
    pub signal_b: String,
}
impl Module for Multiply {
    fn tick_sample(&self, mdl_cache: &HashMap<String, Box<dyn Module>>, info: ModuleInfo) -> f32 {
        let signal_a = mdl_cache[&self.signal_a].tick_sample(mdl_cache, info);
        let signal_b = mdl_cache[&self.signal_b].tick_sample(mdl_cache, info);
        signal_a * signal_b
    }
    
}
impl Multiply {
    pub fn new(signal_a: String, signal_b: String) -> Box<dyn Module> {
        println!("created new multiply module. signal_a: {signal_a}, len: {signal_b} \n");
        Box::from(Self{
            signal_a: signal_a,
            signal_b: signal_b,
        })
    }
    pub fn new_entry(params: &Vec<Value>) -> Box<dyn Module> {
        Multiply::new(params[0].as_str().unwrap().to_string(), params[1].as_str().unwrap().to_string())
    }
    // https://i.imgur.com/utJyz9b.png
}
