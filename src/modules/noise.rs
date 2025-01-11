use std::collections::HashMap;
use toml::Value;
use rand::prelude::*;

use crate::core::module::{Module, ModuleInfo};


// note for future me: just generates white noise
pub struct Noise {
    pub _unused: (),
}
impl Module for Noise {
    fn tick_sample(&self, mdl_cache: &HashMap<String, Box<dyn Module>>, info: ModuleInfo) -> f32 {
        let mut rng = rand::thread_rng();
        
        
        (rng.gen::<f32>() * 2.0) - 1.0
    }
}
impl Noise{
    pub fn new() -> Box<dyn Module> {
        println!("created new noise module. \n");
        Box::from(Self{
            _unused: (),
        })
    }
    pub fn new_entry(params: &Vec<Value>) -> Box<dyn Module> {
        Noise::new()
    }
    
}

