use std::collections::HashMap;
use crate::core::{module::Module, sample::Sample};

pub struct Scale { 
    pub signal: String,
    pub in_min: String,
    pub in_max: String,
    pub out_min: String,
    pub out_max: String,
}
impl Scale{
    pub fn new(signal: String, in_min: String, in_max: String, out_min: String, out_max: String) -> Box<dyn Module> {
        Box::from(Self{
            signal: signal,
            in_min: in_min,
            in_max: in_max,
            out_min: out_min,
            out_max: out_max,
        })
    }
}
impl Module for Scale {
    fn tick_sample(&self, mdl_cache: &HashMap<String, Box<dyn Module>>, i: usize) -> Sample {
        let output: Sample = {
            let signal = mdl_cache[&self.signal].tick_sample(mdl_cache, i).val; 
            let in_min = mdl_cache[&self.in_min].tick_sample(mdl_cache, i).val;
            let in_max = mdl_cache[&self.in_max].tick_sample(mdl_cache, i).val;
            let out_min = mdl_cache[&self.out_min].tick_sample(mdl_cache, i).val;
            let out_max = mdl_cache[&self.out_max].tick_sample(mdl_cache, i).val;
            Sample::new(i, (signal - in_min)*((out_max-out_min)/(in_max-in_min)) + out_min)
        };
        output
    }
}