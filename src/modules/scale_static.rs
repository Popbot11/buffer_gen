use std::collections::HashMap;
use toml::Value;

use crate::core::{module::{Module, ModuleInfo}, sample::Sample};

pub struct ScaleStatic { 
    pub signal: String,
    pub in_min: f32,
    pub in_max: f32,
    pub out_min: f32,
    pub out_max: f32,
}
impl ScaleStatic{
    pub fn new(signal: String, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> Box<dyn Module> {
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
        ScaleStatic::new(params[0].as_str().unwrap().to_string(),
                 params[1].as_float().unwrap() as f32,
                 params[2].as_float().unwrap() as f32,
                 params[3].as_float().unwrap() as f32,
                 params[4].as_float().unwrap() as f32
                )
    }
}
impl Module for ScaleStatic {
    fn tick_sample(&self, mdl_cache: &HashMap<String, Box<dyn Module>>, info: ModuleInfo) -> f32 {
        let output = {
            let signal = mdl_cache[&self.signal].tick_sample(mdl_cache, info); 
            let in_min = &self.in_min;
            let in_max = &self.in_max;
            let out_min = &self.out_min;
            let out_max = &self.out_max;
            (signal - in_min)*((out_max-out_min)/(in_max-in_min)) + out_min
        };
        output
    }
}