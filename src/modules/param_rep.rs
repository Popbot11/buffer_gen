use std::collections::HashMap;
use crate::core::{module::{Module, ModuleInfo}, sample::Sample};
use text_io::{self, read};
use toml::Value;


pub struct ParamRep {
    pub scale_factor: f32,
}
impl Module for ParamRep {
    fn tick_sample(&self, mdl_cache: &HashMap<String, Box<dyn Module>>, info: ModuleInfo) -> Sample {
        Sample::new(info.i, self.scale_factor * (info.rep + 1) as f32)
        // + 1 cause rep i zero-based
    }
}
impl ParamRep {
    pub fn new(scale_factor: f32) -> Box<dyn Module> {
        println!("created new param_rep module. scale_factor: {scale_factor} \n");
        Box::from(Self{
            scale_factor: scale_factor,
        })
    }

    pub fn new_entry(params: &Vec<Value>) -> Box<dyn Module> {
        ParamRep::new(params[0].as_float().unwrap() as f32)
    }
}