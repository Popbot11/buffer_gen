use std::{cell::RefCell, collections::HashMap, fmt::{self, Debug, Display}, rc::Rc};
use hound::WavSpec;

use crate::core::sample::Sample;

pub trait Module{
    fn tick_sample(&self, mdl_cache: &HashMap<String, Box<dyn Module>>, i: usize) -> Sample;
    //&self -- the instance of the respective struct that implements tick_sample
    //i: the requested sample index

}
impl std::fmt::Display for dyn Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self)
    }
} 


//struct with info so that i don't have to implement a bunch of stuff each time i want to add a new bit of info to be carried UPWARDS into modules
//this is actually a pain so nevermind. i'd have to do a bunch of stupid borrowing and at that point its not even worth it. 
pub struct ModuleInfo{
    iter: usize, //to be used once I implement multi-rendering to 
    i: usize, // sample index
    spec: WavSpec, //contains info about sample rate, bit depth, channel count, and sample format (int, float)
}


