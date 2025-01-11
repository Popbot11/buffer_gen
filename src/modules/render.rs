use std::{cell::RefCell, collections::HashMap, rc::Rc};
use toml::Value;

use crate::core::module::{Module, ModuleInfo};


//looks at a buffer that's been pushed to memory, and renders that as a file
//Render needs to be passed the name of the buffer it's rendering, and the name of the module that creates that buffer so that it can actually trigger the chain. 
pub struct Render {
    pub file_name: String,
    pub rendered_module: String,
    pub buffer_name: String,
    pub buffer_cache: Rc<RefCell<HashMap<String, Vec<f32>>>>,
}
impl Render {
    pub fn new(file_name: String, rendered_module: String, buffer_name: String, buffer_cache: Rc<RefCell<HashMap<String, Vec<f32>>>>) -> Box<dyn Module>{
        println!("created new render module. rendered_module: {rendered_module}, buffer_name: {buffer_name}\n");
        Box::from(Self{
            file_name: file_name,
            rendered_module: rendered_module,
            buffer_name: buffer_name,
            buffer_cache: buffer_cache
        })
    }

    pub fn new_entry(params: &Vec<Value>, buffer_cache: Rc<RefCell<HashMap<String, Vec<f32>>>>) -> Box<dyn Module> {
        Render::new(params[0].as_str().unwrap().to_string(), params[1].as_str().unwrap().to_string(), params[2].as_str().unwrap().to_string(), buffer_cache)
    }
}

impl Module for Render {
    fn tick_sample(&self, mdl_cache: &HashMap<String, Box<dyn Module>>, info: ModuleInfo) -> f32 {
        mdl_cache[&self.rendered_module].tick_sample(mdl_cache, info); 
        let buffer_name = ModuleInfo::rep_name(info, self.buffer_name.clone());
        let file_name = ModuleInfo::rep_name(info, self.file_name.clone());
        let mut writer = hound::WavWriter::create(format!("{}{}",file_name, ".wav"), info.spec).unwrap();
        
        let samples = &self.buffer_cache.borrow_mut()[&buffer_name];
        // let mut buffer: Vec<f32> = Vec::new();
        
        for s in samples {
            let amplitude = i16::MAX as f32;
            writer.write_sample((amplitude * s) as i16).unwrap();
        }
        return 0.0; //just because this needs to return a value. this sample value isn't actually intended to be used in any way. 
    }
}
