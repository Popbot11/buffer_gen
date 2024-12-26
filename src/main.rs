mod core;
mod modules;
use core::{module::{Module, ModuleInfo}, sample::Sample};
use std::{cell::RefCell, collections::HashMap, fs::File, io::Read, rc::Rc};
use modules::{buffer::Buffer, multiply::Multiply, noise::Noise, param::Param, param_rep::ParamRep, pass::Pass, render::Render, repeat::Repeat, scale::Scale, scale_static::ScaleStatic, simplest_lpf::SimplestLPF, sin::Sine};
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
                "multiply" => Multiply::new_entry(params),
                "noise" => Noise::new_entry(params),
                "param" => Param::new_entry(params),
                "param_rep" => ParamRep::new_entry(params),
                "pass" => Pass::new_entry(params),
                "render" => Render::new_entry(params, buffer_cache.clone()),
                "repeat" => Repeat::new_entry(params),
                "scale" => Scale::new_entry(params),
                "scale_static" => ScaleStatic::new_entry(params),
                "simplest_lpf" => SimplestLPF::new_entry(params, name.clone(), buffer_cache.clone()),
                "sine" => Sine::new_entry(params),
                // "sin" => todo!(),
                _ => panic!("AAAGAGHH PANIC the meteor")
            }
        };
        cache.insert(name, module);

    }
    println!("successfully added all entries to module cache!");
    cache
}

fn go(root_module: String, mdl_cache: &HashMap<String, Box<dyn Module>>, buff_cache: Rc<RefCell<HashMap<String, Vec<Sample>>>>) -> () {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let info = ModuleInfo::new(0, 0, spec);
    
    mdl_cache[&root_module].tick_sample(mdl_cache, info); //i and rep do nothing here
    println!("fully itterated through module cache")
}


fn main() {
    // import toml file
    let toml_file = {
        let mut file = File::open("lpf_test.toml").expect("Unable to open file");
        let mut data = String::new();
        file.read_to_string(&mut data).expect("Unable to read string");
        // println!("{}", data);
        data.parse::<Table>().unwrap()
    }; 
    let mut buffer_cache: Rc<RefCell<HashMap<String, Vec<Sample>>>> = Rc::new(RefCell::new(HashMap::new())); 
    let module_cache: HashMap<String, Box<dyn Module>> = toml_to_hashmap(toml_file, buffer_cache.clone());
        
    go("repeat".to_string(), &module_cache, buffer_cache.clone());

}

