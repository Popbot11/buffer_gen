use std::collections::HashMap;
use crate::core::{module::Module, sample::Sample};


pub struct Param {
    pub value: f32,
}
impl Module for Param {
    fn tick_sample(&self, mdl_cache: &HashMap<String, Box<dyn Module>>, i: usize) -> Sample {
        Sample::new(i, self.value)
    }
}
impl Param {
    pub fn new(value: f32) -> Box<dyn Module> {
        Box::from(Self{
            value: value,
        })
    }
}
