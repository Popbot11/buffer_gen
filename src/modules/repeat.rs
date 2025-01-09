use std::collections::HashMap;
use toml::Value;

use crate::core::{module::{Module, ModuleInfo}, sample::Sample};

pub struct Repeat {
    pub signal: String,
    pub repetitions: usize,
}
impl Module for Repeat {
    fn tick_sample(&self, mdl_cache: &HashMap<String, Box<dyn Module>>, info: ModuleInfo) -> f32 {
        for r in 0..self.repetitions {
            let info = ModuleInfo{rep: r, ..info};
            let signal = mdl_cache[&self.signal].tick_sample(mdl_cache, info);
        }
        
        0.0 //repeat isn't meant to return sample values; it works on modules such as Buffer and Render that also don't output samples
    }

}
//currently just passes the sample from the previous module; need to implement ModuleInfo first.
impl Repeat{
    pub fn new(signal: String, repetitions: usize) -> Box<dyn Module> {
        println!("created new repeat module. signal: {signal}, reps: {repetitions} \n");
        Box::from(Self{
            signal: signal,
            repetitions: repetitions,
        })
    }
    // entry_name = {type = "repeat", params = ["buffer_module_name", 4]}
    // params = ["buffer_module_name", number of reps (usize)]
    pub fn new_entry(params: &Vec<Value>) -> Box<dyn Module> {
        Repeat::new(params[0].as_str().unwrap().to_string(), params[1].as_integer().unwrap() as usize)
    }
}