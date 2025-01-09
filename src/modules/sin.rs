use std::{collections::HashMap, f32::consts::PI};
use toml::Value;

use crate::core::{module::{Module, ModuleInfo}, sample::Sample};

pub struct Sine {
    pub frequency: String
}
impl Sine {
    fn new(frequency: String) -> Box<dyn Module> {
        Box::from(Self{
            frequency: frequency,
        })
    }
    pub fn new_entry(params: &Vec<Value>) -> Box<dyn Module> {
        Sine::new(params[0].as_str().unwrap().to_string())
    }
}
impl Module for Sine {
    fn tick_sample(&self, mdl_cache: &HashMap<String, Box<dyn Module>>, info: ModuleInfo) -> f32 {
        let freq = mdl_cache[&self.frequency].tick_sample(mdl_cache, info);
        
        ((info.i as f32 * 2.0 * PI * freq) / (info.spec.sample_rate as f32)).sin()
        
    }
}