use std::{cell::RefCell, collections::HashMap, rc::Rc};
use toml::Value;

use crate::core::{module::{Module, ModuleInfo}, sample::Sample};

pub struct Buffer {
    pub signal: String,
    pub name: String,
    pub len: usize,
    pub buff_cache: Rc<RefCell<HashMap<String, Vec<f32>>>>,
}
impl Buffer {
    pub fn new(signal: String, name: String, len: usize, buff_cache: Rc<RefCell<HashMap<String, Vec<f32>>>>) -> Box<dyn Module> {
        println!("created new buffer module. signal: {signal}, len: {len}, name: {name} \n");
        Box::from(Self{
            signal: signal, 
            len: len,
            name: name,
            buff_cache: buff_cache,
        })
    }

    pub fn new_entry(params: &Vec<Value>, buffer_cache: Rc<RefCell<HashMap<String, Vec<f32>>>>) -> Box<dyn Module> {
        Buffer::new(params[0].as_str().unwrap().to_string(), params[1].as_str().unwrap().to_string(), params[2].as_integer().unwrap() as usize, buffer_cache)
    }
    // dear lord

    // unused code for console ui for like. whatever. should probabl get rid of this

}
impl Module for Buffer {
    fn tick_sample(&self, mdl_cache: &HashMap<String, Box<dyn Module>>, info: ModuleInfo) -> f32 {
        // get the name for the buffer; default if theres no reps but if there are reps any names after 0 will have the int appended to the end
        let name = ModuleInfo::rep_name(info, self.name.clone());
        
        // This loop is the primary way that the sample index (i) field in ModuleInfo is incremented. 
        let mut buff: Vec<f32> = Vec::new();
        for index in 0..self.len{
            let info = ModuleInfo{i: index, ..info};
            buff.push(mdl_cache[&self.signal].tick_sample(mdl_cache, info))
        };
        println!("added {} to buffer cache, len: {}", name, buff.len());
        
        let sample =  buff[info.i];
        self.buff_cache.borrow_mut().insert(name, buff);
        
        sample //this currently does nothing; i just need to return a value. 
    }
}