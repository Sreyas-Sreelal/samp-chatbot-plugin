#[macro_use]
extern crate samp_sdk;
extern crate eliza;

mod plugin;
mod natives;

use plugin::ChatBot;

new_plugin!(ChatBot);
