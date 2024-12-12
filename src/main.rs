mod core;
mod modules;
use core::{module::Module, sample::Sample};
use std::{cell::RefCell, collections::HashMap, fs::File, io::Read, rc::Rc};
use modules::{buffer::Buffer, gain::Gain, param::Param, render::Render};
use toml::Table;

fn go(renderer: String, mdl_cache: &mut HashMap<String, Box<dyn Module>>, buff_cache: Rc<RefCell<HashMap<String, Vec<Sample>>>>) -> () {
    mdl_cache[&renderer].tick_sample(mdl_cache, 0); //i does nothing here
    // Sample::new(signal.i, signal.val)
    println!("fully itterated through module cache")
}


fn main() {
    let mut buffer_cache: Rc<RefCell<HashMap<String, Vec<Sample>>>> = Rc::new(RefCell::new(HashMap::new()));
    let mut module_cache: HashMap<String, Box<dyn Module>> = HashMap::new();

    let toml_file = {
        let mut file = File::open("test.toml").expect("Unable to open file");
        let mut data = String::new();
        file.read_to_string(&mut data).expect("Unable to read string");
        println!("{}", data);
        data.parse::<Table>().unwrap()
    }; //I LOVE RUST YEAGAh
    
    // names cannot contain spaces.
    module_cache.insert("renderer_module".to_string(), Render::new("create_buffer".to_string(), "audio".to_string(), buffer_cache.clone()));
    module_cache.insert("create_buffer".to_string(), Buffer::new("gain".to_string(), "audio".to_string(), 22050, buffer_cache.clone()));
    module_cache.insert("gain".to_string(), Gain::new("parameter".to_string(), "parameter_two".to_string()));
    module_cache.insert("parameter".to_string(), Param::new(0.5));
    module_cache.insert("parameter_two".to_string(), Param::new(1.0));
        
    go("renderer_module".to_string(), &mut module_cache, buffer_cache);

}



