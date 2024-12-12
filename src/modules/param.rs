use std::collections::HashMap;
use crate::core::{module::{Module, ModuleInfo}, sample::Sample};
use text_io::{self, read};
use toml::Value;


pub struct Param {
    pub value: f32,
}
impl Module for Param {
    fn tick_sample(&self, mdl_cache: &HashMap<String, Box<dyn Module>>, info: ModuleInfo) -> Sample {
        Sample::new(info.i, self.value)
    }

    
}
impl Param {
    pub fn new(value: f32) -> Box<dyn Module> {
        println!("created new param module. value: {value} \n");
        Box::from(Self{
            value: value,
        })
    }

    pub fn new_entry(params: &Vec<Value>) -> Box<dyn Module> {
        Param::new(params[0].as_float().unwrap() as f32)
    }

    pub fn create_new(mut mdl_cache: HashMap<String, Box<dyn Module>>) -> HashMap<String, Box<dyn Module>> {

        print!("enter module name: ");
        let name: String = read!("{}\n");
        print!("enter parameter value: ");
        let value: String = read!("{}\n");
        // println!("test");
        println!("Param name: {}, value: {}", name.trim(), value.trim());
        mdl_cache.insert(name.trim().to_string(), Param::new(value.trim().parse().unwrap()));

        mdl_cache
    }
}
