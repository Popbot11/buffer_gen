use std::{cell::RefCell, collections::HashMap, rc::Rc};
use text_io::read;
use toml::Value;

use crate::core::{module::{Module, ModuleInfo}, sample::Sample};

pub struct Buffer {
    pub signal: String,
    pub name: String,
    pub len: usize,
    pub buff_cache: Rc<RefCell<HashMap<String, Vec<Sample>>>>,
}
impl Buffer {
    pub fn new(signal: String, name: String, len: usize, buff_cache: Rc<RefCell<HashMap<String, Vec<Sample>>>>) -> Box<dyn Module> {
        println!("created new buffer module. signal: {signal}, len: {len}, name: {name} \n");
        Box::from(Self{
            signal: signal, 
            len: len,
            name: name,
            buff_cache: buff_cache,
        })
    }

    pub fn new_entry(params: &Vec<Value>, buffer_cache: Rc<RefCell<HashMap<String, Vec<Sample>>>>) -> Box<dyn Module> {
        Buffer::new(params[0].as_str().unwrap().to_string(), params[1].as_str().unwrap().to_string(), params[2].as_integer().unwrap() as usize, buffer_cache)
    }
    // dear lord

    // unused code for console ui for like. whatever. should probabl get rid of this
    pub fn create_new(mut mdl_cache: HashMap<String, Box<dyn Module>>, buff_cache: Rc<RefCell<HashMap<String, Vec<Sample>>>>) -> HashMap<String, Box<dyn Module>> {

        print!("enter module name: ");
        let name: String = read!("{}\n");

        print!("enter source signal name: ");
        let signal: String = read!("{}\n");

        print!("enter buffer name: ");
        let buff_name: String = read!("{}\n");

        print!("enter buffer length (in samples): ");
        let len: String = read!("{}\n");

        // println!("test");
        // println!("Param name: {}, value: {}", name.trim(), value.trim());
        mdl_cache.insert(name.trim().to_string(), Buffer::new(signal.trim().to_string(), buff_name.trim().to_string(), len.trim().parse().unwrap(), buff_cache));

        mdl_cache
    }

}
impl Module for Buffer {
    fn tick_sample(&self, mdl_cache: &HashMap<String, Box<dyn Module>>, info: ModuleInfo) -> Sample {
        // get the name for the buffer; default if theres no reps but if there are reps any names after 0 will have the int appended to the end
        let name = ModuleInfo::rep_name(info, self.name.clone());
        
        // This loop is the primary way that the sample index (i) field in ModuleInfo is incremented. 
        let mut buff: Vec<Sample> = Vec::new();
        for index in 0..self.len{
            let info = ModuleInfo{i: index, ..info};
            buff.push(Sample::new(index, mdl_cache[&self.signal].tick_sample(mdl_cache, info).val))
        };

        let sample = Sample::new(info.i, buff[info.i].val);
        self.buff_cache.borrow_mut().insert(name, buff);
        
        sample //this currently does nothing; i just need to return a value. 
    }
}