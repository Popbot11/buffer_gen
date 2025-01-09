use std::collections::HashMap;
use crate::core::{module::{Module, ModuleInfo}, sample::Sample};
use toml::Value;


pub struct ParamRep {
    pub starting_value: f32,
    pub increment: f32,
}
impl Module for ParamRep {
    fn tick_sample(&self, mdl_cache: &HashMap<String, Box<dyn Module>>, info: ModuleInfo) -> f32 {
        self.starting_value + (self.increment * (info.rep) as f32)
        // + 1 cause rep i zero-based
    }
}

impl ParamRep {
    pub fn new(starting_value: f32, increment: f32) -> Box<dyn Module> {
        println!("created new param_rep module. starting_value: {starting_value}, increment: {increment} \n");
        Box::from(Self{
            starting_value: starting_value,
            increment: increment,
        })
    }

    pub fn new_entry(params: &Vec<Value>) -> Box<dyn Module> {
        ParamRep::new(params[0].as_float().unwrap() as f32, params[1].as_float().unwrap() as f32)
    }
}