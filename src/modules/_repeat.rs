use std::collections::HashMap;
use toml::Value;

use crate::core::{module::Module, sample::Sample};

pub struct Repeat {
    pub signal: String,
    pub repetitions: usize,
}
impl Module for Repeat {
    fn tick_sample(&self, mdl_cache: &HashMap<String, Box<dyn Module>>, i: usize) -> Sample {
        let signal: Sample = mdl_cache[&self.signal].tick_sample(mdl_cache, i);
        
        Sample::new(signal.i, signal.val)
    }

}
//currently just passes the sample from the previous module; need to implement ModuleInfo first.
impl Repeat{
    pub fn new(signal: String, repetitions: usize) -> Box<dyn Module> {
        println!("created new pass module. signal: {signal} \n");
        Box::from(Self{
            signal: signal,
            repetitions: repetitions,
        })
    }
    pub fn new_entry(params: &Vec<Value>) -> Box<dyn Module> {
        Repeat::new(params[0].as_str().unwrap().to_string(), params[1].as_integer().unwrap() as usize)
    }
    
}