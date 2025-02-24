# buffer_gen

buffer_gen is a fully modular non-realtime synthesis engine aimed at performing many simple operations in bulk. 

this project is currently extraordinarily unfinished, and Likely to be completely restructured. 
I started this as I was beginning to learn about Rust and compiled languages in general; this was my first project with Rust and as such the way I've got things organized isn't very good. 


## usage

Currently, the only way to tell the engine what to do is by writing a module chain in an apropriately named and formatted `.toml` file. 
My goal is to eventually make an actual user interface for this, but for now that's what we're working with until I can finalize the actual underlying behaviour of the modular engine.

Each line in the toml file represents a singular modular, each of which has one output and some number of inputs. Each input either refers to the output of another module, or refers to some other singular parameter value. 
for example, the line:
```toml
static = {type = "param", params = [1.0]}
```
creates a new module with the _unique_ name `static`, where the type of module is `"param"`, with a single parameter being `1.0`. 


Most implemented modules, though, take the outputs of other modules as their input. Take the chain:
```toml
carrier = {type = "sine", params = ["modulator"]}
modulator = {type = "sine", params = ["freq"]}
freq = {type = "param", params = [300]}
```
This creates three modules: `carrier`, `modulator`, and `freq`. (Note that the names of modules don't really matter as long as they are unique and consistent). 
`carrier` is of type `"sine"`, as is `modulator`. The `sine` module is a very simple phase modulation sine oscillator, where the input simply specifies the oscillators frequency at a given sample.  
finally, the `freq` module generates a buffer of samples all with a value `300`. 

A complete functional chain of modules would look something like this:
```toml
repeat = {type = "repeat", params = ["render_module", 1]}
render_module = {type = "render", params = ["repeat_test_", "joe_buffer", "audio"]}
joe_buffer = {type = "buffer", params = ["filter", "audio", 4410]}

filter = {type = "simplest_lpf", params = ["sine"]}
sine = {type = "sine", params = ["nyquist"]}

nyquist = {type = "param", params = [11025.0]}
```
The program knows that the `repeat` module is the root module because I hardcoded that. Obviously this is not perminent. 
Whatever chain of modules exists above a module of type `"repeat"` is executed n times. In this instance, the first module above `repeat` is a render module. This means that if I told the repeat module to execute 40 times, 40 files would be rendered. (it is possible to increment values for each consecutive repetetion using the `"param_rep"` module type, although this module doesn't yet support nested repetetions, which are otherwise currently possible). The render module then needs a complete buffer of audio to render, which it aquires by referencing the title of the buffer module, and the name of the audio buffer that the buffer module produces, which in this case is `"audio"`. (this is convoluted, i know. in future versions hopefully it will be simpler). Finally, the buffer module is what gets the actual dsp going. 

For each sample, the buffer module asks its parent modle for a sample, which cascades up and down the module tree until the entire buffer is of the required length. 

I'm still very new to all of this, so my explination and implementation are both probably not very good. But it's a start; I've got plans for future projects that will hopefully use a lot of the same patterns I came up with for this. 

More info soon hopefully once I have a brek from school to continue working on this. 
