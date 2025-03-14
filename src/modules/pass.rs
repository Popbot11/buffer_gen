use std::collections::HashMap;
use toml::Value;

use crate::core::{module::{Module, ModuleInfo}, sample::Sample};

pub struct Pass {
    pub signal: String,
}
impl Module for Pass {
    fn tick_sample(&self, mdl_cache: &HashMap<String, Box<dyn Module>>, info: ModuleInfo) -> Sample {
        let signal: Sample = mdl_cache[&self.signal].tick_sample(mdl_cache, info);
        
        Sample::new(signal.i, signal.val)
    }
}
impl Pass{
    pub fn new(signal: String) -> Box<dyn Module> {
        println!("created new pass module. signal: {signal} \n");
        Box::from(Self{
            signal: signal,
        })
    }
    pub fn new_entry(params: &Vec<Value>) -> Box<dyn Module> {
        Pass::new(params[0].as_str().unwrap().to_string())
    }
    
}
