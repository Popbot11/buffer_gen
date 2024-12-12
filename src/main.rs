mod core;
mod modules;
use core::{module::Module, sample::Sample};
use std::{cell::RefCell, collections::HashMap, fs::File, io::Read, rc::Rc};
use modules::{buffer::Buffer, gain::Gain, param::Param, render::Render};
use toml::{map::Map, Table, Value};

fn toml_to_hashmap(toml_file: Map<String, Value>)-> HashMap<String, Box<dyn Module>>{
    let mut cache: HashMap<String, Box<dyn Module>> = HashMap::new();
    for entry in toml_file {
        let key = entry.0; //module name
        let params = entry.1["params"].as_array().unwrap();
        let value = {
            match entry.1["type"].as_str().unwrap() {
                "gain" => Gain::new(params[0].as_str().unwrap().to_string(), params[1].as_str().unwrap().to_string()),
                // https://i.imgur.com/utJyz9b.png
                _ => todo!()
            }
        };
        cache.insert(key, value);

    }
    todo!()
}

fn go(renderer: String, mdl_cache: &mut HashMap<String, Box<dyn Module>>, buff_cache: Rc<RefCell<HashMap<String, Vec<Sample>>>>) -> () {
    mdl_cache[&renderer].tick_sample(mdl_cache, 0); //i does nothing here
    // Sample::new(signal.i, signal.val)
    println!("fully itterated through module cache")
}


fn main() {
    // import toml file
    let toml_file = {
        let mut file = File::open("test.toml").expect("Unable to open file");
        let mut data = String::new();
        file.read_to_string(&mut data).expect("Unable to read string");
        println!("{}", data);
        data.parse::<Table>().unwrap()
    }; 


    //convert toml file into hashmap
    let buffer_cache_toml: HashMap<String, Box<dyn Module>> = toml_to_hashmap(toml_file);
    


    let mut buffer_cache: Rc<RefCell<HashMap<String, Vec<Sample>>>> = Rc::new(RefCell::new(HashMap::new()));
    let mut module_cache: HashMap<String, Box<dyn Module>> = HashMap::new();

    // names cannot contain spaces.
    module_cache.insert("renderer_module".to_string(), Render::new("create_buffer".to_string(), "audio".to_string(), buffer_cache.clone()));
    module_cache.insert("create_buffer".to_string(), Buffer::new("gain".to_string(), "audio".to_string(), 22050, buffer_cache.clone()));
    module_cache.insert("gain".to_string(), Gain::new("parameter".to_string(), "parameter_two".to_string()));
    module_cache.insert("parameter".to_string(), Param::new(0.5));
    module_cache.insert("parameter_two".to_string(), Param::new(1.0));
        
    go("renderer_module".to_string(), &mut module_cache, buffer_cache);

}



