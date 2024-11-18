use std::{cell::RefCell, collections::HashMap, rc::Rc};
use crate::core::sample::Sample;

pub trait Module{
    fn tick_sample(&self, mdl_cache: &HashMap<String, Box<dyn Module>>, i: usize) -> Sample;
    //&self -- the instance of the respective struct that implements tick_sample
    //i: the requested sample index


    // fn add(&self, mdl_cache: &mut HashMap<String, Box<dyn Module>>, mdl_name: String){
    //     let v = &self::new();
    //     mdl_cache.insert(mdl_name, v);
    // }
}


//into passed up the recursive chain of modules
//each 
pub struct ModuleInfo{
    mdl_cache: HashMap<String, Box<dyn Module>>,
    buff_cache: Rc<RefCell<HashMap<String, Vec<Sample>>>>,
    iter: usize //to be used once I implement multi-rendering to 
}