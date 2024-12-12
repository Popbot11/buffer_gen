mod core;
mod modules;
use core::{module::Module, sample::Sample};
use std::{cell::RefCell, collections::HashMap, fs::File, io::Read, rc::Rc};
use modules::{buffer::Buffer, gain::Gain, param::Param, pass::Pass, render::Render, scale::Scale};
use toml::{map::Map, Table, Value};

fn toml_to_hashmap(toml_file: Map<String, Value>, buffer_cache: Rc<RefCell<HashMap<String, Vec<Sample>>>>)-> HashMap<String, Box<dyn Module>>{
    let mut cache: HashMap<String, Box<dyn Module>> = HashMap::new();

    for entry in toml_file {
        // println!("ENTRY: {:?}", entry);
        let name = entry.0; //module name
        println!("NAME: {}", name);
        let params = entry.1["params"].as_array().unwrap(); //all of the module's unique parameters
        println!("PARAMS: {:?}", params);
        println!("TYPE: {}", entry.1["type"].as_str().unwrap());
        
        //functional code is better than bad code, but still make this better later
        let module = {
            println!("adding module to cache...");
            //entry.1 is the value of each entry, which contains  the module type and the module parameters
            match entry.1["type"].as_str().unwrap() {
                "buffer" => Buffer::new_entry(params, buffer_cache.clone()),
                "gain" => Gain::new_entry(params),
                "param" => Param::new_entry(params),
                "pass" => Pass::new_entry(params),
                "render" => Render::new_entry(params, buffer_cache.clone()),
                "scale" => Scale::new_entry(params),
                // "sin" => todo!(),
                _ => panic!("AAAGAGHH")
            }
        };
        cache.insert(name, module);

    }
    println!("successfully added all entries to module cache!");
    cache
}

fn go(renderer: String, mdl_cache: &HashMap<String, Box<dyn Module>>, buff_cache: Rc<RefCell<HashMap<String, Vec<Sample>>>>) -> () {
    mdl_cache[&renderer].tick_sample(mdl_cache, 0); //i does nothing here
    // Sample::new(signal.i, signal.val)
    println!("fully itterated through module cache")
}


fn main() {
    // import toml file
    let toml_file = {
        let mut file = File::open("testchain.toml").expect("Unable to open file");
        let mut data = String::new();
        file.read_to_string(&mut data).expect("Unable to read string");
        // println!("{}", data);
        data.parse::<Table>().unwrap()
    }; 


    
    let mut buffer_cache: Rc<RefCell<HashMap<String, Vec<Sample>>>> = Rc::new(RefCell::new(HashMap::new())); 
    let module_cache: HashMap<String, Box<dyn Module>> = toml_to_hashmap(toml_file, buffer_cache.clone());
        
    go("renderer_module".to_string(), &module_cache, buffer_cache.clone());

}
