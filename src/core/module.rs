use std::{cell::RefCell, collections::HashMap, fmt::{self, Debug, Display}, rc::Rc};
use hound::WavSpec;

use crate::core::sample::Sample;

pub trait Module{
    //&self -- the instance of the respective struct that implements tick_sample
    fn tick_sample(&self, mdl_cache: &HashMap<String, Box<dyn Module>>, info: ModuleInfo) -> Sample;
}
impl std::fmt::Display for dyn Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self)
    }
} 


//struct with info so that i don't have to implement a bunch of stuff each time i want to add a new bit of info to be carried UPWARDS into modules
//this is actually a pain so nevermind. i'd have to do a bunch of stupid borrowing and at that point its not even worth it. 
#[derive(Debug, Clone, Copy)]
pub struct ModuleInfo{
    pub rep: usize, //to be used once I implement multi-rendering 
    pub i: usize, // sample index
    pub spec: WavSpec, //contains info about sample rate, bit depth, channel count, and sample format (int, float)
}
impl ModuleInfo{
    pub fn new(rep: usize, i: usize, spec: WavSpec) -> Self {
        Self{
            rep: rep,
            i: i,
            spec: spec,
        }
    }
    pub fn _incr_i(self) -> Self{
        ModuleInfo {
            i: self.i + 1,
            ..self
        }
    }
    pub fn _incr_rep(self) -> Self{
        ModuleInfo {
            rep: self.rep + 1,
            ..self
        }
    }
    pub fn _test(self) -> Self{
        let a = self.i;
        let b = self.i;
        ModuleInfo {
            i: a + b,
            ..self
        }
    }
    pub fn rep_name(self, name: String) -> String {
        match self.rep {
            0 => name,
            _ => format!("{}{}", name, self.rep)
        }
    }
}


