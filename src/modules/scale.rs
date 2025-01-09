use std::collections::HashMap;
use toml::Value;

use crate::core::{module::{Module, ModuleInfo}, sample::Sample};

pub struct Scale { 
    pub signal: String,
    pub in_min: String,
    pub in_max: String,
    pub out_min: String,
    pub out_max: String,
}
impl Scale{
    pub fn new(signal: String, in_min: String, in_max: String, out_min: String, out_max: String) -> Box<dyn Module> {
        println!("created new scale module. signal: {signal}, in_min: {in_min}, in_max: {in_max}, out_min: {out_min}, out_max: {out_max} \n");
        Box::from(Self{
            signal: signal,
            in_min: in_min,
            in_max: in_max,
            out_min: out_min,
            out_max: out_max,
        })
    }
    pub fn new_entry(params: &Vec<Value>) -> Box<dyn Module> {
        Scale::new(params[0].as_str().unwrap().to_string(),
                 params[1].as_str().unwrap().to_string(),
                 params[2].as_str().unwrap().to_string(),
                 params[3].as_str().unwrap().to_string(),
                 params[4].as_str().unwrap().to_string()
                )
    }
}
impl Module for Scale {
    fn tick_sample(&self, mdl_cache: &HashMap<String, Box<dyn Module>>, info: ModuleInfo) -> f32 {
        let output: f32 = {
            let signal = mdl_cache[&self.signal].tick_sample(mdl_cache, info); 
            let in_min = mdl_cache[&self.in_min].tick_sample(mdl_cache, info);
            let in_max = mdl_cache[&self.in_max].tick_sample(mdl_cache, info);
            let out_min = mdl_cache[&self.out_min].tick_sample(mdl_cache, info);
            let out_max = mdl_cache[&self.out_max].tick_sample(mdl_cache, info);
            
            (signal - in_min)*((out_max-out_min)/(in_max-in_min)) + out_min
        };
        output
    }
}