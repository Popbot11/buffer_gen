use std::collections::HashMap;
use crate::core::{module::Module, sample::Sample};

// struct Sine {
//     frequency: String
// }
// impl Sine {
//     fn new(frequency: String) -> Box<dyn Module> {
//         Box::from(Self{
//             frequency: frequency,
//         })
//     }
// }
// impl Module for Sine {
//     fn tick_sample(&self, mdl_cache: &HashMap<String, Box<dyn Module>>, i: usize) -> Sample {
//         //copy code from sine in other implementation
//     }
// }