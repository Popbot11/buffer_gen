use std::collections::HashMap;
use super::module::Module;


pub trait ConsoleUI{
    fn render_tree(&self, mdl_cache: &HashMap<String, Box<dyn Module>>) -> String;
    //render a tree in the console that displays what the current module network looks like

}
//should this be a trait? probably because i'd want to itterate through the module cache and then return a single string that displays each module with text