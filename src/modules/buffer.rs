use std::{cell::RefCell, collections::HashMap, rc::Rc};
use crate::core::{module::Module, sample::Sample};

struct Buffer {
    pub signal: String,
    pub name: String,
    pub len: usize,
    pub buff_cache: Rc<RefCell<HashMap<String, Vec<Sample>>>>,
}
impl Buffer {
    fn new(signal: String, name: String, len: usize, buff_cache: Rc<RefCell<HashMap<String, Vec<Sample>>>>) -> Box<dyn Module> {
        Box::from(Self{
            signal: signal, 
            len: len,
            name: name,
            buff_cache: buff_cache,
        })
    }
}
impl Module for Buffer {
    fn tick_sample(&self, mdl_cache: &HashMap<String, Box<dyn Module>>, i: usize) -> Sample {
        let mut buff: Vec<Sample> = Vec::new();
        for i in 0..self.len{
            buff.push(Sample::new(i, mdl_cache[&self.signal].tick_sample(mdl_cache, i).val))
        };
        
        let sample = Sample::new(i, buff[i].val);
        self.buff_cache.borrow_mut().insert(self.name.clone(), buff);
        sample
    }
}