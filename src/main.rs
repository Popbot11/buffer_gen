mod core;
mod modules;
use core::module::Module;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::core::sample::Sample;

fn go(renderer: String, mdl_cache: &mut HashMap<String, Box<dyn Module>>, buff_cache: Rc<RefCell<HashMap<String, Vec<Sample>>>>) -> () {
    mdl_cache[&renderer].tick_sample(mdl_cache, 0); //i does nothing here
    // Sample::new(signal.i, signal.val)
}

fn main() {
    // let test: Sample = Sample::new(0, 0.0);

    
    let mut buffer_cache: Rc<RefCell<HashMap<String, Vec<Sample>>>> = Rc::new(RefCell::new(HashMap::new()));
    let mut module_cache:HashMap<String, Box<dyn Module>> = HashMap::new();

    module_cache.insert("renderer module".to_string(), Render::new("create buffer".to_string(), "audio".to_string(), buffer_cache.clone()));
    module_cache.insert("create buffer".to_string(), Buffer::new("parameter".to_string(), "audio".to_string(), 22050, buffer_cache.clone()));
    module_cache.insert("parameter".to_string(), Param::new(0.5));

    go("renderer module".to_string(), &mut module_cache, buffer_cache);

    // let something: Render = Render { rendered_module: "()".to_string(), buffer_name: "()".to_string(), buffer_cache: buffer_cache };

    // something.insert(mdl_cache);
}