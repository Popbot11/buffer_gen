use std::{collections::HashMap, f32::consts::PI};
use crate::core::{module::{Module, ModuleInfo}, sample::Sample};

struct Sine {
    frequency: String
}
impl Sine {
    fn new(frequency: String) -> Box<dyn Module> {
        Box::from(Self{
            frequency: frequency,
        })
    }
}
impl Module for Sine {
    fn tick_sample(&self, mdl_cache: &HashMap<String, Box<dyn Module>>, info: ModuleInfo) -> Sample {
        let sample = ((info.i as f32 * 2.0 * PI) / (info.spec.sample_rate as f32)).sin();
        Sample::new(info.i, sample)
    }
}