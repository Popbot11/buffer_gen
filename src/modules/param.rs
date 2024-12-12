use std::collections::HashMap;
use crate::core::{module::Module, sample::Sample};
use text_io::{self, read};


pub struct Param {
    pub value: f32,
}
impl Module for Param {
    fn tick_sample(&self, mdl_cache: &HashMap<String, Box<dyn Module>>, i: usize) -> Sample {
        Sample::new(i, self.value)
    }

    
    fn render_text(&self) -> String {
        todo!()
    }
    
    // fn create_module(&self, mdl_cache: HashMap<String, Box<dyn Module>>) -> HashMap<String, Box<dyn Module>> {
    //     todo!()
    // }
}
impl Param {
    pub fn new(value: f32) -> Box<dyn Module> {
        
        
        Box::from(Self{
            value: value,
        })
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
