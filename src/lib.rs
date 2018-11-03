#[macro_use]
extern crate samp_sdk;
extern crate eliza;
mod plugin;
use plugin::ChatBot;
new_plugin!(ChatBot);