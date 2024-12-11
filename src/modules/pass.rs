use std::collections::HashMap;
use crate::core::{module::Module, sample::Sample};

pub struct Pass {
    pub signal: String,
}
impl Module for Pass {
    fn tick_sample(&self, mdl_cache: &HashMap<String, Box<dyn Module>>, i: usize) -> Sample {
        let signal: Sample = mdl_cache[&self.signal].tick_sample(mdl_cache, i);
        
        Sample::new(signal.i, signal.val)
    }
    
    // fn create_module(&self, mut mdl_cache: HashMap<String, Box<dyn Module>>) -> HashMap<String, Box<dyn Module>>  {
    //     todo!()
    // }
    
    fn render_text(&self) -> String {
        todo!()
    }
}
impl Pass{
    pub fn new(signal: String) -> Box<dyn Module> {
        Box::from(Self{
            signal: signal,
        })
    }
    
}
