use std::collections::HashMap;
use toml::Value;
use rand::prelude::*;

use crate::core::{module::{Module, ModuleInfo}, sample::Sample};


// note for future me: just generates white noise
pub struct Noise {
    pub unused: (),
}
impl Module for Noise {
    fn tick_sample(&self, mdl_cache: &HashMap<String, Box<dyn Module>>, info: ModuleInfo) -> Sample {
        let mut rng = rand::thread_rng();

        
        Sample::new(info.i, (rng.gen::<f32>() * 2.0) - 1.0)
    }
}
impl Noise{
    pub fn new() -> Box<dyn Module> {
        println!("created new noise module. \n");
        Box::from(Self{
            unused: (),
        })
    }
    pub fn new_entry(params: &Vec<Value>) -> Box<dyn Module> {
        Noise::new()
    }
    
}

