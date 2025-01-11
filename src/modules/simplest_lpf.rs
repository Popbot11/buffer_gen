//  y(n) = x(n) + x(n-1)

use std::{cell::RefCell, collections::HashMap, rc::Rc, vec};
use toml::Value;

use crate::core::module::{Module, ModuleInfo};

pub struct SimplestLPF {
    pub signal: String,
    pub self_name: String, 
    pub buff_cache: Rc<RefCell<HashMap<String, Vec<f32>>>>,
}


impl  Module for SimplestLPF  {
    fn tick_sample(&self, mdl_cache: &HashMap<String, Box<dyn Module>>, info: ModuleInfo) -> f32 {
        let signal = mdl_cache[&self.signal].tick_sample(mdl_cache, info);
        let history_name: String = format!("z^-{}-{}", self.self_name, "signal");

        self.buff_cache.borrow_mut().entry(history_name.clone())
                                    .and_modify(|f| f.insert(0, signal))
                                    .or_insert(vec![signal, 0.0]);
                                    // currentlu the .and_modify doesn't remove unneeded terms, which is an issue.   
        let history = &self.buff_cache.borrow()[&history_name];

        history[1] + history[0]
    }
}
impl SimplestLPF{
    pub fn new(signal: String, self_name: String, buff_cache: Rc<RefCell<HashMap<String, Vec<f32>>>>) -> Box<dyn Module> {
        println!("created new simplest_lpf module. signal: {signal} \n");

        Box::from(Self{
            signal, 
            self_name,
            buff_cache,
        })
    }
    pub fn new_entry(params: &Vec<Value>, self_name: String, buffer_cache: Rc<RefCell<HashMap<String, Vec<f32>>>>) -> Box<dyn Module> {
        SimplestLPF::new(params[0].as_str().unwrap().to_string(), self_name, buffer_cache)
    }
}
