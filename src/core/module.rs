use std::collections::HashMap;
use crate::core::sample::Sample;

pub trait Module{
    fn tick_sample(&self, mdl_cache: &HashMap<String, Box<dyn Module>>, i: usize) -> Sample;

    // fn add(&self, mdl_cache: &mut HashMap<String, Box<dyn Module>>, mdl_name: String){
    //     let v = &self::new();
    //     mdl_cache.insert(mdl_name, v);
    // }
}