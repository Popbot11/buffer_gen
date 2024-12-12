mod core;
mod modules;
use core::{module::Module, sample::Sample};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use modules::{buffer::Buffer, gain::Gain, param::Param, render::Render};
use text_io::read;

fn go(renderer: String, mdl_cache: &mut HashMap<String, Box<dyn Module>>, buff_cache: Rc<RefCell<HashMap<String, Vec<Sample>>>>) -> () {
    mdl_cache[&renderer].tick_sample(mdl_cache, 0); //i does nothing here
    // Sample::new(signal.i, signal.val)
    println!("fully itterated through module cache")
}


fn main() {
    let mut buffer_cache: Rc<RefCell<HashMap<String, Vec<Sample>>>> = Rc::new(RefCell::new(HashMap::new()));
    let mut module_cache: HashMap<String, Box<dyn Module>> = HashMap::new();

    // names cannot contain spaces.
    module_cache.insert("renderer_module".to_string(), Render::new("create_buffer".to_string(), "audio".to_string(), buffer_cache.clone()));
    module_cache.insert("create_buffer".to_string(), Buffer::new("gain".to_string(), "audio".to_string(), 22050, buffer_cache.clone()));
    module_cache.insert("gain".to_string(), Gain::new("parameter".to_string(), "parameter_two".to_string()));
    module_cache.insert("parameter".to_string(), Param::new(0.5));
    module_cache.insert("parameter_two".to_string(), Param::new(1.0));

    {
    // print!("enter choice: ");
    // let choice: String = read!("{}\n");
    // match choice.trim() {
    //     "Param" => module_cache = Param::create_new(module_cache),
    //     // "Render" => module_cache = Render::create_new(module_cache),
    //     "Buffer" => module_cache = Buffer::create_new(module_cache, buffer_cache),
    //     "exit" => return,
    //     _ => {
    //         println!("not an option!");
    //         return;
    //     },
    // }
    // for i in module_cache.keys() {
    //     println!("{i}");
    // }
    }   
        
    go("renderer_module".to_string(), &mut module_cache, buffer_cache);

}



// TESTS: 

fn _mutate_test(){
    let mut test_vec: Vec<i32> = Vec::new();
    // shadow_test(test_vec);
    for i in 0..5 {
        println!("{:?}", test_vec);
        test_vec = _get_thing(test_vec, i);
    }
    println!("{:?}", test_vec);
}

fn _get_thing(mut cache: Vec<i32>, value: i32) -> Vec<i32>{
    cache.push(value);
    cache
}

fn _parse_test() -> f32{
    "1.0".parse().unwrap()
}