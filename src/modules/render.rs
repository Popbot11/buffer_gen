use std::{cell::RefCell, collections::HashMap, rc::Rc};
use toml::Value;

use crate::core::{module::Module, sample::Sample};


//looks at a buffer that's been pushed to memory, and renders that as a file
//Render needs to be passed the name of the buffer it's rendering, and the name of the module that creates that buffer so that it can actually trigger the chain. 
pub struct Render {
    pub rendered_module: String,
    pub buffer_name: String,
    pub buffer_cache: Rc<RefCell<HashMap<String, Vec<Sample>>>>,
}
impl Render {
    pub fn new(rendered_module: String, buffer_name: String, buffer_cache: Rc<RefCell<HashMap<String, Vec<Sample>>>>) -> Box<dyn Module>{
        println!("created new render module. rendered_module: {rendered_module}, buffer_name: {buffer_name}\n");
        Box::from(Self{
            rendered_module: rendered_module,
            buffer_name: buffer_name,
            buffer_cache: buffer_cache
        })
    }

    pub fn new_entry(params: &Vec<Value>, buffer_cache: Rc<RefCell<HashMap<String, Vec<Sample>>>>) -> Box<dyn Module> {
        Render::new(params[0].as_str().unwrap().to_string(), params[1].as_str().unwrap().to_string(), buffer_cache)
    }
}

impl Module for Render {
    fn tick_sample(&self, mdl_cache: &HashMap<String, Box<dyn Module>>, i: usize) -> Sample {
        mdl_cache[&self.rendered_module].tick_sample(mdl_cache, i); //

        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::create(format!("{}{}","test", ".wav"), spec).unwrap();
        
        let samples = &self.buffer_cache.borrow_mut()[&self.buffer_name];
        // let mut buffer: Vec<f32> = Vec::new();
        
        for s in samples {
            let amplitude = i16::MAX as f32;
            writer.write_sample((amplitude * s.val) as i16).unwrap();
        }
        return Sample::new(0, 0.0); //just because this needs to return a value. this sample value isn't actually intended to be used in any way. 
    }

    fn render_text(&self) -> String {
        todo!()
    }
}
